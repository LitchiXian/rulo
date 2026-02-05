use crate::handlers::{auth_handler, blog_handler, tag_handler};
use crate::services::{auth_service::AuthService, blog_service::BlogService, tag_service::TagService};
use axum::{
    http::{header, Method},
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};

// 创建所有路由
pub fn create_routes(db_pool: PgPool) -> Router {
    // 创建服务
    let auth_service = AuthService::new(db_pool.clone());
    let blog_service = BlogService::new(db_pool.clone());
    let tag_service = TagService::new(db_pool.clone());

    // 配置CORS - 允许前端跨域访问
    let cors = CorsLayer::new()
        .allow_origin(Any) // 允许所有来源，生产环境应限制
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::HeaderName::from_static("satoken")])
        .allow_credentials(false);

    // 创建主路由并注入服务
    let app = Router::new()
        // 根路径欢迎页面
        .route("/", get(root_handler))
        // 认证相关路由
        .route("/login/login", post(auth_handler::login_handler))
        .route("/login/register", post(auth_handler::register_handler))
        .route("/login/logout", get(auth_handler::logout_handler))
        .route(
            "/login/getRegisterCode",
            post(auth_handler::get_register_code_handler),
        )
        .route(
            "/login/getLoginInfo",
            get(auth_handler::get_login_info_handler),
        )
        // 博客文章相关路由
        .route(
            "/blog/article/save",
            post(blog_handler::save_article_handler),
        )
        .route(
            "/blog/article/remove",
            post(blog_handler::remove_article_handler),
        )
        .route(
            "/blog/article/update",
            post(blog_handler::update_article_handler),
        )
        .route(
            "/blog/article/list",
            get(blog_handler::list_articles_handler),
        )
        .route("/blog/article/get", get(blog_handler::get_article_handler))
        .route(
            "/blog/article/getUserArticleList",
            get(blog_handler::get_user_articles_handler),
        )
        // 注入服务状态
        .with_state((auth_service, blog_service))
        // 添加CORS中间件
        .layer(cors.clone());

    // 创建标签路由，使用单独的状态
    let tag_router = Router::new()
        .route("/blog/tag/list", get(tag_handler::list_tags_handler))
        .route("/blog/tag/get", get(tag_handler::get_tag_handler))
        .route("/blog/tag/save", post(tag_handler::save_tag_handler))
        .route("/blog/tag/update", post(tag_handler::update_tag_handler))
        .route("/blog/tag/remove", post(tag_handler::remove_tag_handler))
        .with_state(tag_service)
        .layer(cors);

    // 合并路由
    app.merge(tag_router)
}

// 根路径处理函数
pub async fn root_handler() -> &'static str {
    "欢迎使用 Rust 博客系统 API 服务！\n\n\
    可用的 API 端点:\n\
    - POST /login/login - 用户登录\n\
    - POST /login/register - 用户注册\n\
    - GET /login/logout - 用户登出\n\
    - POST /login/getRegisterCode - 获取注册验证码\n\
    - GET /login/getLoginInfo - 获取登录信息\n\
    - POST /blog/article/save - 保存文章\n\
    - POST /blog/article/remove - 删除文章\n\
    - POST /blog/article/update - 更新文章\n\
    - GET /blog/article/list - 获取文章列表\n\
    - GET /blog/article/get - 获取文章详情\n\
    - GET /blog/article/getUserArticleList - 获取用户文章列表\n\n\
    服务运行正常，请使用相应的 API 端点进行操作。"
}
