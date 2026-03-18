use rulo_common::result::R;

use crate::system::monitor::{model::ServerInfo, service};

pub async fn server_info_handler() -> R<ServerInfo> {
    service::get_server_info()
}
