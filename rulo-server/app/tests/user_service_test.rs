use app::system::user::model::{BindRolesDto, SysUserSaveDto};
use app::system::user::service;
use common::model::IdsDto;
use sqlx::PgPool;

#[sqlx::test(migrations = "../migrations")]
async fn test_user_save_and_remove(pool: PgPool) {
    let dto = SysUserSaveDto {
        user_name: "testuser".to_string(),
        nick_name: "Test".to_string(),
        password: "password123".to_string(),
        email: Some("test@example.com".to_string()),
        remark: None,
    };
    let result = service::save(&pool, &dto).await.unwrap();
    let user = result.take_data().unwrap();
    assert_eq!(user.user_name, "testuser");
    assert_ne!(user.password, "password123"); // 密码已被哈希
    assert!(user.is_active);
    assert!(!user.is_deleted);

    // soft delete
    let ids_dto = IdsDto { ids: vec![user.id] };
    service::remove(&pool, &ids_dto).await.unwrap();
}

#[sqlx::test(migrations = "../migrations")]
async fn test_user_save_duplicate_name_fails(pool: PgPool) {
    let dto = SysUserSaveDto {
        user_name: "dupuser".to_string(),
        nick_name: "D".to_string(),
        password: "password123".to_string(),
        email: None,
        remark: None,
    };
    service::save(&pool, &dto).await.unwrap();
    assert!(service::save(&pool, &dto).await.is_err());
}

#[sqlx::test(migrations = "../migrations")]
async fn test_update_bind_roles_with_invalid_role(pool: PgPool) {
    let user_dto = SysUserSaveDto {
        user_name: "binduser".to_string(),
        nick_name: "B".to_string(),
        password: "password123".to_string(),
        email: None,
        remark: None,
    };
    let user = service::save(&pool, &user_dto)
        .await
        .unwrap()
        .take_data()
        .unwrap();

    // 用不存在的 role_id 绑定应失败
    let bind_dto = BindRolesDto {
        user_id: user.id,
        role_ids: vec![999999],
    };
    assert!(service::update_bind_roles(&pool, &bind_dto).await.is_err());
}

#[sqlx::test(migrations = "../migrations")]
async fn test_update_bind_roles_empty_clears(pool: PgPool) {
    let user_dto = SysUserSaveDto {
        user_name: "clearuser".to_string(),
        nick_name: "C".to_string(),
        password: "password123".to_string(),
        email: None,
        remark: None,
    };
    let user = service::save(&pool, &user_dto)
        .await
        .unwrap()
        .take_data()
        .unwrap();

    // 空绑定应成功（清空现有绑定）
    let bind_dto = BindRolesDto {
        user_id: user.id,
        role_ids: vec![],
    };
    service::update_bind_roles(&pool, &bind_dto).await.unwrap();

    let result = service::list_bind_roles(&pool, user.id).await.unwrap();
    assert!(result.take_data().unwrap().is_empty());
}
