use std::sync::Arc;

use axum::{Router, http::{Request, StatusCode}, middleware::Next, response::Response};
use rulo_common::{state::AppState, util::jwt_util};
use tower_http::classify::StatusInRangeFailureClass;

use crate::system::{auth, menu, permission, role, user};

pub fn routes() -> Router<Arc<AppState>> {
    let public_router = Router::new().nest("/auth", auth::router::routes());

    let protected_router = Router::new()
        .nest("/user", user::router::routes())
        .nest("/role", role::router::routes())
        .nest("/menu", menu::router::routes())
        .nest("/permission", permission::router::routes())
        .nest("/auth", auth::router::routes());

    Router::new().merge(public_router).merge(protected_router)
}

pub async fn auth(mut req: Request,next: Next) -> Result<Response, StatusCode> {
    let token = req.headers().get("authorization")
        .and_then(|v| v.to_str().ok());

    let token = match token {
        Some(t) => t,
        None => return Err(StatusCode::UNAUTHORIZED);
    }

    if jwt_util::verify_token(token).is_err(){
         return Err(StatusCode::UNAUTHORIZED);
    };

    Ok(next.run(req).await)
}
