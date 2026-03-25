use utoipa::OpenApi;

use crate::system;

#[derive(OpenApi)]
#[openapi(
    paths(
        system::auth::handler::login_handler,
        system::auth::handler::register_handler,
        system::auth::handler::logout_handler,
        system::auth::handler::info_handler,
        system::user::handler::save_handler,
        system::user::handler::remove_handler,
        system::user::handler::update_handler,
        system::user::handler::update_bind_roles_handler,
        system::user::handler::detail_handler,
        system::user::handler::list_handler,
        system::user::handler::list_bind_roles_handler,
        system::role::handler::save_handler,
        system::role::handler::remove_handler,
        system::role::handler::update_handler,
        system::role::handler::update_bind_menus_handler,
        system::role::handler::update_bind_perms_handler,
        system::role::handler::detail_handler,
        system::role::handler::list_handler,
        system::role::handler::list_bind_menus_handler,
        system::role::handler::list_bind_perms_handler,
        system::permission::handler::save_handler,
        system::permission::handler::remove_handler,
        system::permission::handler::update_handler,
        system::permission::handler::detail_handler,
        system::permission::handler::list_handler,
        system::menu::handler::save_handler,
        system::menu::handler::remove_handler,
        system::menu::handler::update_handler,
        system::menu::handler::detail_handler,
        system::menu::handler::list_handler,
        system::monitor::handler::server_info_handler,
        system::monitor::handler::health_handler,
        system::audit::handler::list_handler,
    ),
    components(
        schemas(
            rulo_common::model::IdDto,
            rulo_common::model::IdsDto,
            system::auth::model::AuthUserDto,
            system::auth::model::LoginInfoVo,
            system::auth::model::UserInfoVo,
            system::auth::model::MenuTreeNode,
            system::user::model::SysUser,
            system::user::model::SysUserSaveDto,
            system::user::model::SysUserUpdateDto,
            system::user::model::SysUserListDto,
            system::user::model::BindRolesDto,
            system::role::model::SysRole,
            system::role::model::SysRoleSaveDto,
            system::role::model::SysRoleUpdateDto,
            system::role::model::SysRoleListDto,
            system::role::model::BindMenusDto,
            system::role::model::BindPermsDto,
            system::permission::model::SysPermission,
            system::permission::model::SysPermissionSaveDto,
            system::permission::model::SysPermissionUpdateDto,
            system::permission::model::SysPermissionListDto,
            system::menu::model::SysMenu,
            system::menu::model::SysMenuSaveDto,
            system::menu::model::SysMenuUpdateDto,
            system::menu::model::SysMenuListDto,
            system::monitor::model::ServerInfo,
            system::monitor::model::CpuInfo,
            system::monitor::model::MemInfo,
            system::monitor::model::SysInfo,
            system::monitor::model::DiskInfo,
            system::monitor::model::RustInfo,
            system::monitor::handler::HealthResponse,
            system::audit::model::SysAuditLog,
            system::audit::model::AuditLogListDto,
        )
    ),
    modifiers(&SecurityAddon),
    info(title = "Rulo API", description = "Rulo Admin System API", version = "1.0.0")
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::ApiKey(
                    utoipa::openapi::security::ApiKey::Header(
                        utoipa::openapi::security::ApiKeyValue::new("authorization"),
                    ),
                ),
            );
        }
    }
}