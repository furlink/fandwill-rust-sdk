use std::{
    collections::BTreeMap,
    io::{Read, Write},
    net::TcpListener,
    sync::mpsc::{self, Receiver},
    thread,
    time::Duration,
};

use fandwill_sdk::{
    Error, FandwillClient, NotificationsQuery, PaginationParams, ReviewFilter,
    fandwill_vo::{
        collections::AddToCollectionRequest,
        listings::CreateListingVO,
        reviews::{CreateReplyVO, CreateReviewVO},
    },
};
use reqwest::StatusCode;

struct MockServer {
    base_url: String,
    request: Receiver<String>,
    thread: thread::JoinHandle<()>,
}

impl MockServer {
    fn start(status: &str, headers: &[(&str, &str)], body: &str) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let (sender, request) = mpsc::channel();
        let status = status.to_owned();
        let headers = headers
            .iter()
            .map(|(name, value)| ((*name).to_owned(), (*value).to_owned()))
            .collect::<Vec<_>>();
        let body = body.to_owned();

        let thread = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();

            let mut request_bytes = Vec::new();
            let mut buffer = [0_u8; 4096];
            let expected_len = loop {
                let count = stream.read(&mut buffer).unwrap();
                assert!(count > 0, "client closed before sending complete headers");
                request_bytes.extend_from_slice(&buffer[..count]);

                if let Some(header_end) = find_bytes(&request_bytes, b"\r\n\r\n") {
                    let headers = String::from_utf8_lossy(&request_bytes[..header_end]);
                    let content_length = headers
                        .lines()
                        .find_map(|line| {
                            let (name, value) = line.split_once(':')?;
                            name.eq_ignore_ascii_case("content-length")
                                .then(|| value.trim().parse::<usize>().unwrap())
                        })
                        .unwrap_or(0);
                    break header_end + 4 + content_length;
                }
            };

            while request_bytes.len() < expected_len {
                let count = stream.read(&mut buffer).unwrap();
                assert!(count > 0, "client closed before sending complete body");
                request_bytes.extend_from_slice(&buffer[..count]);
            }
            sender
                .send(String::from_utf8(request_bytes).unwrap())
                .unwrap();

            write!(
                stream,
                "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n",
                body.len()
            )
            .unwrap();
            for (name, value) in headers {
                write!(stream, "{name}: {value}\r\n").unwrap();
            }
            write!(stream, "\r\n{body}").unwrap();
        });

        Self {
            base_url: format!("http://{address}/api"),
            request,
            thread,
        }
    }

    fn finish(self) -> String {
        let request = self.request.recv_timeout(Duration::from_secs(5)).unwrap();
        self.thread.join().unwrap();
        request
    }
}

fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn request_body(request: &str) -> &str {
    request.split_once("\r\n\r\n").unwrap().1
}

fn request_path_and_query(request: &str) -> (String, BTreeMap<String, String>) {
    let target = request
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap();
    let url = url::Url::parse(&format!("http://localhost{target}")).unwrap();
    let pairs = url.query_pairs().into_owned().collect();
    (url.path().to_owned(), pairs)
}

#[tokio::test]
async fn notification_query_and_jwt_header_match_the_contract() {
    let server = MockServer::start(
        "200 OK",
        &[("Content-Type", "application/json")],
        r#"{"items":[],"page_info":{"has_next":false,"total":0}}"#,
    );
    let client = FandwillClient::new(&server.base_url)
        .unwrap()
        .with_jwt("secret-token");
    let query = NotificationsQuery {
        page: 2,
        page_size: 10,
        unread_only: true,
        after_id: Some("watermark".into()),
    };

    let response = client.list_notifications(&query).await.unwrap();
    assert!(response.items.is_empty());

    let request = server.finish();
    let request_lower = request.to_ascii_lowercase();
    assert!(request.starts_with("GET /api/users/me/notifications?"));
    let (path, query) = request_path_and_query(&request);
    assert_eq!(path, "/api/users/me/notifications");
    assert_eq!(
        query,
        BTreeMap::from([
            ("after_id".into(), "watermark".into()),
            ("page".into(), "2".into()),
            ("page_size".into(), "10".into()),
            ("unread_only".into(), "true".into()),
        ])
    );
    assert!(request_lower.contains("authorization: bearer secret-token\r\n"));
}

#[tokio::test]
async fn listing_create_sends_json_and_keeps_markdown_validation() {
    let server = MockServer::start(
        "200 OK",
        &[("Content-Type", "application/json")],
        r#"{"id":"listing","created_by":"user","title":"Title","description":"Description","content":"Content","banners":[],"created_at":"2026-08-04T00:00:00Z","updated_at":"2026-08-04T00:00:00Z","markdown_validation":[{"level":"warning"}]}"#,
    );
    let client = FandwillClient::new(&server.base_url).unwrap();
    let body = CreateListingVO {
        title: "Title".into(),
        description: "Description".into(),
        content: "Content".into(),
        resources: vec!["resource".into()],
    };

    let response = client.add_listing(&body).await.unwrap();
    assert_eq!(response.inner.id, "listing");
    assert_eq!(response.markdown_validation.len(), 1);

    let request = server.finish();
    assert!(request.starts_with("POST /api/listings HTTP/1.1"));
    let sent: serde_json::Value = serde_json::from_str(request_body(&request)).unwrap();
    assert_eq!(sent["resources"], serde_json::json!(["resource"]));
}

#[tokio::test]
async fn listing_versions_parse_the_paged_wire_shape() {
    let server = MockServer::start(
        "200 OK",
        &[("Content-Type", "application/json")],
        r#"{"items":[{"id":"version","title":"Title","description":"Description","content":"Content","changed_by":null,"created_at":"2026-08-04T00:00:00Z","status":"published"}],"page_info":{"has_next":false,"total":1}}"#,
    );
    let client = FandwillClient::new(&server.base_url).unwrap();

    let response = client.get_listing_versions("listing").await.unwrap();
    assert_eq!(response.items.len(), 1);
    assert_eq!(response.page_info.total, 1);

    let request = server.finish();
    assert!(request.starts_with("GET /api/listings/listing/versions HTTP/1.1"));
    assert!(!request.starts_with("GET /api/listings/listing/versions?"));
}

#[tokio::test]
async fn structured_api_error_is_additive_to_status_and_raw_body() {
    let body = r#"{"code":404,"msg":"not found"}"#;
    let server = MockServer::start(
        "404 Not Found",
        &[("Content-Type", "application/json")],
        body,
    );
    let client = FandwillClient::new(&server.base_url).unwrap();

    let error = client.get_listing("missing").await.unwrap_err();
    assert_eq!(error.status(), Some(StatusCode::NOT_FOUND));
    assert_eq!(error.body(), Some(body));
    assert_eq!(error.api_error().unwrap().code, 404);
    assert_eq!(error.api_error().unwrap().msg, "not found");

    let request = server.finish();
    assert!(request.starts_with("GET /api/listings/missing HTTP/1.1"));
}

#[tokio::test]
async fn resource_redirect_returns_resolved_location_without_following() {
    let server = MockServer::start("302 Found", &[("Location", "/objects/file")], "");
    let client = FandwillClient::new(&server.base_url)
        .unwrap()
        .with_jwt("secret-token");

    let target = client.get_resource("resource").await.unwrap();
    assert_eq!(
        target.as_str(),
        format!("{}/objects/file", target.origin().ascii_serialization())
    );

    let request = server.finish();
    assert!(request.starts_with("GET /api/resources/resource HTTP/1.1"));
    assert!(
        request
            .to_ascii_lowercase()
            .contains("authorization: bearer secret-token\r\n")
    );
}

#[tokio::test]
async fn resource_redirect_reports_missing_or_invalid_location() {
    let server = MockServer::start("302 Found", &[], "");
    let client = FandwillClient::new(&server.base_url).unwrap();

    let error = client.get_resource("resource").await.unwrap_err();
    assert!(matches!(
        error,
        Error::MissingRedirectLocation {
            status: StatusCode::FOUND
        }
    ));
    let _ = server.finish();

    let server = MockServer::start("302 Found", &[("Location", "http://[invalid")], "");
    let client = FandwillClient::new(&server.base_url).unwrap();
    assert!(matches!(
        client.get_resource("resource").await.unwrap_err(),
        Error::InvalidRedirectLocation(_)
    ));
    let _ = server.finish();
}

#[tokio::test]
async fn resource_redirect_preserves_non_redirect_api_errors() {
    let body = r#"{"code":401,"msg":"unauthorized"}"#;
    let server = MockServer::start(
        "401 Unauthorized",
        &[("Content-Type", "application/json")],
        body,
    );
    let client = FandwillClient::new(&server.base_url).unwrap();

    let error = client.get_resource("resource").await.unwrap_err();
    assert_eq!(error.status(), Some(StatusCode::UNAUTHORIZED));
    assert_eq!(error.body(), Some(body));
    assert_eq!(error.api_error().unwrap().code, 401);
    let _ = server.finish();
}

macro_rules! assert_json_contract {
    ($client:ident, $method:literal, $path:literal, $response:expr, $call:expr) => {{
        let server =
            MockServer::start("200 OK", &[("Content-Type", "application/json")], $response);
        let $client = FandwillClient::new(&server.base_url).unwrap();
        let _ = $call.await.unwrap();
        let request = server.finish();
        let request_line = request.lines().next().unwrap();
        let mut parts = request_line.split_whitespace();
        assert_eq!(parts.next(), Some($method));
        let target = parts.next().unwrap();
        assert_eq!(target.split('?').next().unwrap(), concat!("/api", $path));
        request
    }};
}

#[tokio::test]
async fn added_endpoint_methods_and_paths_match_the_openapi_contract() {
    const EMPTY: &str = "{}";
    const PAGED_EMPTY: &str = r#"{"items":[],"page_info":{"has_next":false,"total":0}}"#;
    const BOOKMARK: &str = r#"{"listing_id":"listing","created_at":"2026-08-04T00:00:00Z"}"#;
    const CREATED_RESOURCE: &str = r#"{"id":"resource","upload":{"url":"https://uploads.example.test/","method":"POST","fields":{},"max_bytes":1024,"expires_in_secs":60}}"#;
    const RESOURCE: &str = r#"{"id":"resource","mime_type":null,"hash":null,"size_bytes":null,"created_at":"2026-08-04T00:00:00Z"}"#;
    const REVIEW: &str =
        r#"{"id":"review","created_by":"user","listing_id":"listing","content":"Content"}"#;
    const REVIEW_WITH_VALIDATION: &str = r#"{"id":"review","created_by":"user","listing_id":"listing","content":"Content","markdown_validation":[]}"#;
    const REPLY_WITH_VALIDATION: &str = r#"{"id":"reply","review_id":"review","parent_reply_id":null,"created_by":"user","content":"Reply","created_at":"2026-08-04T00:00:00Z","markdown_validation":[]}"#;
    const USER: &str = r#"{"id":"user","sub":"iam-subject"}"#;
    const COLLECTION: &str =
        r#"{"id":"entry","resource_id":"resource","created_at":"2026-08-04T00:00:00Z"}"#;
    const NOTIFICATION: &str = r#"{"id":"notification","actor_id":null,"payload":{"kind":"system","data":{"title":"Title","body":"Body","action":null}},"created_at":"2026-08-04T00:00:00Z","read_at":null}"#;

    let _ = assert_json_contract!(
        client,
        "GET",
        "/listings/listing/bookmark",
        BOOKMARK,
        client.get_bookmark("listing")
    );
    let _ = assert_json_contract!(
        client,
        "POST",
        "/resources",
        CREATED_RESOURCE,
        client.create_resource()
    );
    let _ = assert_json_contract!(
        client,
        "GET",
        "/resources/resource/metadata",
        RESOURCE,
        client.get_resource_metadata("resource")
    );

    let filter = ReviewFilter::default();
    let _ = assert_json_contract!(
        client,
        "GET",
        "/reviews",
        PAGED_EMPTY,
        client.get_reviews(&filter)
    );
    let create_review = CreateReviewVO {
        listing_id: "listing".into(),
        content: "Content".into(),
    };
    let request = assert_json_contract!(
        client,
        "POST",
        "/reviews",
        REVIEW_WITH_VALIDATION,
        client.add_review(&create_review)
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(request_body(&request)).unwrap(),
        serde_json::json!({ "listing_id": "listing", "content": "Content" })
    );
    let _ = assert_json_contract!(
        client,
        "GET",
        "/reviews/review",
        REVIEW,
        client.get_review("review")
    );
    let _ = assert_json_contract!(
        client,
        "DELETE",
        "/reviews/review",
        EMPTY,
        client.delete_review("review")
    );
    let _ = assert_json_contract!(
        client,
        "POST",
        "/reviews/review/like",
        EMPTY,
        client.like_review("review")
    );
    let _ = assert_json_contract!(
        client,
        "DELETE",
        "/reviews/review/like",
        EMPTY,
        client.unlike_review("review")
    );
    let pagination = PaginationParams::default();
    let _ = assert_json_contract!(
        client,
        "GET",
        "/reviews/review/replies",
        PAGED_EMPTY,
        client.get_replies("review", &pagination)
    );
    let create_reply = CreateReplyVO {
        content: "Reply".into(),
        parent_reply_id: None,
    };
    let request = assert_json_contract!(
        client,
        "POST",
        "/reviews/review/replies",
        REPLY_WITH_VALIDATION,
        client.add_reply("review", &create_reply)
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(request_body(&request)).unwrap(),
        serde_json::json!({ "content": "Reply", "parent_reply_id": null })
    );

    let _ = assert_json_contract!(client, "GET", "/users/me", USER, client.get_me());
    let _ = assert_json_contract!(
        client,
        "GET",
        "/users/me/collections",
        PAGED_EMPTY,
        client.list_collections(&pagination)
    );
    let add_to_collection = AddToCollectionRequest {
        resource_id: "resource".into(),
    };
    let request = assert_json_contract!(
        client,
        "POST",
        "/users/me/collections",
        COLLECTION,
        client.add_to_collections(&add_to_collection)
    );
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(request_body(&request)).unwrap(),
        serde_json::json!({ "resource_id": "resource" })
    );
    let _ = assert_json_contract!(
        client,
        "DELETE",
        "/users/me/collections/entry",
        EMPTY,
        client.remove_from_collections("entry")
    );
    let _ = assert_json_contract!(
        client,
        "GET",
        "/users/me/pending",
        PAGED_EMPTY,
        client.get_my_pending_listings(&pagination)
    );
    let _ = assert_json_contract!(
        client,
        "GET",
        "/users/me/recommendations",
        PAGED_EMPTY,
        client.get_my_recommendations(&pagination)
    );
    let _ = assert_json_contract!(client, "GET", "/users/user", USER, client.get_user("user"));
    let _ = assert_json_contract!(
        client,
        "GET",
        "/users/user/bookmarks",
        PAGED_EMPTY,
        client.get_user_bookmarks("user", &pagination)
    );
    let _ = assert_json_contract!(
        client,
        "GET",
        "/users/user/listings",
        PAGED_EMPTY,
        client.get_user_listings("user", &pagination)
    );
    let _ = assert_json_contract!(
        client,
        "GET",
        "/users/user/reviews",
        PAGED_EMPTY,
        client.get_user_reviews("user", &pagination)
    );

    let _ = assert_json_contract!(
        client,
        "PUT",
        "/users/me/notifications",
        r#"{"updated_count":1}"#,
        client.mark_all_notifications_read()
    );
    let _ = assert_json_contract!(
        client,
        "GET",
        "/users/me/notifications/summary",
        r#"{"unread_count":1}"#,
        client.get_notification_summary()
    );
    let _ = assert_json_contract!(
        client,
        "PUT",
        "/users/me/notifications/notification/read",
        NOTIFICATION,
        client.mark_notification_read("notification")
    );
}
