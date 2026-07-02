#[macro_export]
macro_rules! frb_dispatch {
    ($vis:vis async fn $outer:ident = $inner:ident ($client:ident $(, $arg:ident: $ty:ty)* $(,)?) -> Result<$ret:ty, Error> $body:block) => {
        async fn $inner($client: &$crate::client::FandwillClient $(, $arg: $ty)*) -> Result<$ret, Error> $body

        impl $crate::client::FandwillClient {
            #[cfg(not(feature = "frb"))]
            $vis async fn $outer(&self $(, $arg: $ty)*) -> Result<$ret, Error> {
                $inner(self $(, $arg)*).await
            }

            #[cfg(feature = "frb")]
            $vis async fn $outer(&self $(, $arg: $ty)*) -> Result<$ret, $crate::error::FrbError> {
                $inner(self $(, $arg)*).await.map_err(Into::into)
            }
        }
    };
}
