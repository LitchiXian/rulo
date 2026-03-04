use sqlx::{Pool, Postgres};

use crate::system::user::model::SysUser;
use std::collections::HashMap;

pub struct AppState {
    pub db_pool: Pool<Postgres>,
    pub users: HashMap<u64, SysUser>,
    pub next_id: u64,
}

impl AppState {
    pub fn next_id(&mut self) -> u64 {
        let tmp = self.next_id;
        self.next_id += 1;
        tmp
    }
}
