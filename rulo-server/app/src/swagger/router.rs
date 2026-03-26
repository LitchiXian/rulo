use axum::Router;
#[cfg(debug_assertions)]
use utoipa::OpenApi;
#[cfg(debug_assertions)]
use utoipa_scalar::{Scalar, Servable};

#[cfg(debug_assertions)]
use crate::swagger::doc::ApiDoc;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let router = Router::new();

    #[cfg(debug_assertions)]
    let router = router.merge(Scalar::with_url("/scalar", ApiDoc::openapi()));

    router
}