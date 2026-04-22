/// token → user_id 映射，登录写入，登出/过期删除
pub const USER_TOKEN: &str = "system:auth:user:";

/// 用户基础信息缓存，user_id 维度，更新用户资料时清除
pub const USER_INFO: &str = "system:auth:user_info:";

/// 用户鉴权上下文缓存，value 为 `UserAuth { is_super, perms }`，角色/权限变动时清除
pub const USER_AUTH: &str = "system:auth:user_auth:";

/// 用户菜单树缓存，角色/菜单绑定变动时清除
pub const USER_MENUS: &str = "system:auth:menus:";

/// 登录验证码缓存，验证后立即删除
pub const LOGIN_CAPTCHA: &str = "system:auth:login_captcha:";

/// 限流计数器前缀
pub const RATE_LIMIT: &str = "rate_limit:";

pub const ONE_DAY: u64 = 60 * 60 * 24;
