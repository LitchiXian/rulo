use std::sync::Arc;

use axum::{Json, extract::State};
use rulo_common::{
    error::{AppError, time_library::Timestamp},
    result::{R, success},
};
use sqlx::query_as;
use tracing::info;

use super::model::*;
use rulo_common::state::AppState;

// pub async fn user_save_handler(
//     State(state): State<Arc<Mutex<AppState>>>,
//     Json(dto): Json<SysUserSaveDto>,
// ) -> Json<&'static str> {
//     tracing::info!("user_save_handler {:?}", dto);
//     let mut s = state.lock().unwrap();
//     let uid = s.next_id();
//     let new_user = SysUser::new(uid, dto);
//     s.users.insert(uid, new_user);
//     Json("save user successful!")
// }
//
// pub async fn user_list_handler(
//     State(state): State<Arc<Mutex<AppState>>>,
// ) -> Json<HashMap<u64, SysUser>> {
//     info!("user_list_handler");
//     let s = state.lock().unwrap();
//     Json(s.users.clone())
// }

// pub async fn db_user_list_handler(
//     State(state): State<Arc<Mutex<AppState>>>,
// ) -> Json<Vec<DbSysUser>> {
//     info!("db_user_list_handler");
//     let pool = {
//         let s = state.lock().unwrap();
//         s.db_pool.clone()
//     };
//     let data = query_as::<_, DbSysUser>("select * from sys_user;")
//         .fetch_all(&pool)
//         .await
//         .unwrap_or_default();
//     Json(data)
// }

pub async fn db_user_list_handler(State(state): State<Arc<AppState>>) -> R<Vec<DbSysUser>> {
    info!("db_user_list_handler");
    let pool = state.db_pool.clone();
    let data = query_as::<_, DbSysUser>("select * from sys_user;")
        .fetch_all(&pool)
        .await?;
    success(data)
}

pub async fn hello_handler() -> R<()> {
    info!("hello_handler");
    success(())
}

pub async fn hello_error_handler() -> R<Timestamp> {
    info!("hello_error_handler");
    let s = match Timestamp::now() {
        Ok(s) => s,
        Err(_) => return Err(AppError::BadRequest("Hello".to_string())),
    };
    info!("now is {}", s.0);
    success(s)
}

pub async fn hello_redis_handler(State(state): State<Arc<AppState>>) -> R<String> {
    info!("hello_redis_handler");
    let mut conn = state.redis_pool.get().await?;
    let str = match redis::AsyncCommands::get(&mut conn, "abc".to_string()).await? {
        Some(v) => v,
        None => {
            let s = "abcdefg".to_string();
            let _: () = redis::AsyncCommands::set(&mut conn, "abc", &s).await?;
            s
        }
    };
    success(str)
}

pub async fn save_handle(State(state): State<Arc<AppState>>, Json(dto): Json<SysUserSaveDto>) {
let new_user = DbSysUser {

}
}
