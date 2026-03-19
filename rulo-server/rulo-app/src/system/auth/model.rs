use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AuthUserDto {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UserId(pub i64);

#[derive(Debug, Clone)]
pub struct UserToken(pub String);

// 菜单树节点 (返回给前端渲染侧边栏)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MenuTreeNode {
    pub id: i64,
    pub parent_id: i64,
    pub name: String,
    pub menu_type: i16,
    pub path: Option<String>,
    pub component: Option<String>,
    pub icon: Option<String>,
    pub sort_order: i32,
    pub is_hidden: bool,
    #[serde(default)]
    pub children: Vec<MenuTreeNode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginInfoVo {
    pub user: UserInfoVo,
    pub perms: Vec<String>,
    pub menus: Vec<MenuTreeNode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoVo {
    pub id: i64,
    pub user_name: String,
    pub nick_name: String,
    pub email: Option<String>,
    pub is_active: bool,
    pub remark: Option<String>,
}
