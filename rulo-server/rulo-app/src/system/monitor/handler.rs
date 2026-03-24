use rulo_common::result::R;
use rulo_macro::perm;

use crate::system::monitor::{model::ServerInfo, service};

#[utoipa::path(
    get, path = "/system/monitor/server-info",
    responses((status = 200, description = "success", body = ServerInfo)),
    security(("bearer_auth" = []))
)]
#[perm("sys:monitor:server-info")]
pub async fn server_info_handler() -> R<ServerInfo> {
    tokio::task::spawn_blocking(service::get_server_info)
        .await
        .map_err(|e| rulo_common::error::AppError::Internal(e.to_string()))?
}
