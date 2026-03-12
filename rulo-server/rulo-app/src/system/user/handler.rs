use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
};
use rulo_common::{
    error::{AppError, time_library::Timestamp},
    model::{IdDto, IdsDto},
    result::{R, success},
};
use sqlx::query_as;
use tracing::info;

use crate::system::user::service;

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
// ) -> Json<Vec<SysUser>> {
//     info!("db_user_list_handler");
//     let pool = {
//         let s = state.lock().unwrap();
//         s.db_pool.clone()
//     };
//     let data = query_as::<_, SysUser>("select * from sys_user;")
//         .fetch_all(&pool)
//         .await
//         .unwrap_or_default();
//     Json(data)
// }

pub async fn db_user_list_handler(State(state): State<Arc<AppState>>) -> R<Vec<SysUser>> {
    info!("db_user_list_handler");
    let pool = state.db_pool.clone();
    let data = query_as::<_, SysUser>("select * from sys_user;")
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
        Err(_) => return Err(AppError::ServiceError("Hello".to_string())),
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

// 上面是例子,测试代码

pub async fn save_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<SysUserSaveDto>,
) -> R<SysUser> {
    // let _data = query(
    //     "insert into sys_user(
    //     id, user_name, nick_name, password, email,
    //     is_active, is_deleted, create_id, create_time,
    //      update_id, update_time, remark
    //      ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
    // )
    // .bind(new_user.id)
    // .bind(&new_user.user_name)
    // .bind(&new_user.nick_name)
    // .bind(&new_user.password)
    // .bind(&new_user.email)
    // .bind(new_user.is_active)
    // .bind(new_user.is_deleted)
    // .bind(new_user.create_id)
    // .bind(new_user.create_time)
    // .bind(new_user.update_id)
    // .bind(new_user.update_time)
    // .bind(&new_user.remark)
    // .execute(&state.db_pool)
    // .await?;
    service::save_handle(&state.db_pool, &dto).await
}

pub async fn remove_handler(State(state): State<Arc<AppState>>, Json(dto): Json<IdsDto>) -> R<()> {
    service::remove_handle(&state.db_pool, &dto).await
}

pub async fn update_handler(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<SysUserUpdateDto>,
) -> R<()> {
    service::update_handle(&state.db_pool, &dto).await
}

pub async fn detail_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<IdDto>,
) -> R<SysUser> {
    service::get_one_handle(&state.db_pool, &dto).await
}

pub async fn list_handler(
    State(state): State<Arc<AppState>>,
    Query(dto): Query<SysUserListDto>,
) -> R<Vec<SysUser>> {
    service::list_handle(&state.db_pool, &dto).await
}
