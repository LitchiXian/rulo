use rulo_common::{
    model::{IdDto, IdsDto},
    result::{R, success},
};
use sqlx::{PgPool, query, query_as};

use crate::system::user::model::{SysUser, SysUserListDto, SysUserSaveDto, SysUserUpdateDto};

pub async fn save_handle(pool: &PgPool, dto: &SysUserSaveDto) -> R<SysUser> {
    let new_user = SysUser::new_user_from_save_dto(&dto);
    query!(
        "insert into sys_user(
        id, user_name, nick_name, password, email,
        is_active, is_deleted, create_id, create_time,
         update_id, update_time, remark
         ) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
        new_user.id,
        &new_user.user_name,
        &new_user.nick_name,
        &new_user.password,
        new_user.email.as_deref(),
        new_user.is_active,
        new_user.is_deleted,
        new_user.create_id,
        new_user.create_time,
        new_user.update_id,
        new_user.update_time,
        new_user.remark.as_deref()
    )
    .execute(pool)
    .await?;
    success(new_user)
}

pub async fn remove_handle(pool: &PgPool, dto: &IdsDto) -> R<()> {
    sqlx::query!("DELETE FROM sys_user WHERE id = ANY($1)", &dto.ids)
        .execute(pool)
        .await?;
    success(())
}

pub async fn update_handle(pool: &PgPool, dto: &SysUserUpdateDto) -> R<()> {
    sqlx::query!(
        "UPDATE sys_user SET 
            nick_name = COALESCE($2, nick_name),
            password = COALESCE($3, password),
            email = COALESCE($4, email),
            remark = COALESCE($5, remark),
            update_time = now() 
        WHERE id = $1",
        dto.id,
        dto.nick_name.as_deref(),
        dto.password.as_deref(),
        dto.email.as_deref(),
        dto.remark.as_deref()
    )
    .execute(pool)
    .await?;
    success(())
}

pub async fn get_one_handle(pool: &PgPool, dto: &IdDto) -> R<SysUser> {
    let data = query_as!(SysUser, "select * from sys_user where id = $1", dto.id)
        .fetch_one(pool)
        .await?;
    success(data)
}

pub async fn list_handle(pool: &PgPool, dto: &SysUserListDto) -> R<Vec<SysUser>> {
    let data = query_as!(SysUser, "select * from sys_user")
        .fetch_all(pool)
        .await?;
    success(data)
}
