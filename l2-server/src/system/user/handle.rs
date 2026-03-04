use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{Json, extract::State};
use sqlx::query_as;
use tracing::info;

use super::model::*;
use crate::state::AppState;

pub async fn user_save_handler(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(dto): Json<SysUserSaveDto>,
) -> Json<&'static str> {
    tracing::info!("user_save_handler {:?}", dto);
    let mut s = state.lock().unwrap();
    let uid = s.next_id();
    let new_user = SysUser::new(uid, dto);
    s.users.insert(uid, new_user);
    Json("save user successful!")
}

pub async fn user_list_handler(
    State(state): State<Arc<Mutex<AppState>>>,
) -> Json<HashMap<u64, SysUser>> {
    info!("user_list_handler");
    let s = state.lock().unwrap();
    Json(s.users.clone())
}

pub async fn db_user_list_handler(
    State(state): State<Arc<Mutex<AppState>>>,
) -> Json<Vec<DbSysUser>> {
    info!("db_user_list_handler");
    let pool = {
        let s = state.lock().unwrap();
        s.db_pool.clone()
    };
    let data = query_as::<_, DbSysUser>("select * from sys_user;")
        .fetch_all(&pool)
        .await
        .unwrap_or_default();
    Json(data)
}
