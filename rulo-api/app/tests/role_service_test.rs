use app::system::role::model::{SysRoleListDto, SysRoleSaveDto, SysRoleUpdateDto};
use app::system::role::service;
use common::model::IdsDto;
use sqlx::PgPool;

#[sqlx::test(migrations = "../migrations")]
async fn test_role_crud(pool: PgPool) {
    // 1. save
    let save_dto = SysRoleSaveDto {
        role_name: "测试角色".to_string(),
        role_key: "test_role".to_string(),
        remark: Some("integration test".to_string()),
    };
    let result = service::save(&pool, &save_dto).await.unwrap();
    let role = result.take_data().unwrap();
    assert_eq!(role.role_name, "测试角色");
    assert_eq!(role.role_key, "test_role");
    let role_id = role.id;

    // 2. detail
    let detail_dto = common::model::IdDto { id: role_id };
    let result = service::detail(&pool, &detail_dto, true).await.unwrap();
    let fetched = result.take_data().unwrap();
    assert_eq!(fetched.role_key, "test_role");

    // 3. update
    let update_dto = SysRoleUpdateDto {
        id: role_id,
        role_name: Some("更新角色".to_string()),
        role_key: None,
        is_active: None,
        remark: None,
    };
    service::update(&pool, &update_dto, true).await.unwrap();
    let result = service::detail(&pool, &detail_dto, true).await.unwrap();
    assert_eq!(result.take_data().unwrap().role_name, "更新角色");

    // 4. list
    let list_dto = SysRoleListDto {
        role_name: Some("更新".to_string()),
        role_key: None,
        is_active: None,
        create_start_time: None,
        create_end_time: None,
        page_num: None,
        page_size: None,
    };
    let result = service::list(&pool, &list_dto, true).await.unwrap();
    let page = result.take_data().unwrap();
    assert_eq!(page.total, 1);

    // 5. remove (soft delete)
    let ids_dto = IdsDto { ids: vec![role_id] };
    service::remove(&pool, &ids_dto, true).await.unwrap();
    let result = service::detail(&pool, &detail_dto, true).await;
    assert!(result.is_err()); // NotFound
}

#[sqlx::test(migrations = "../migrations")]
async fn test_role_save_empty_name_fails(pool: PgPool) {
    let dto = SysRoleSaveDto {
        role_name: "  ".to_string(),
        role_key: "key".to_string(),
        remark: None,
    };
    assert!(service::save(&pool, &dto).await.is_err());
}

#[sqlx::test(migrations = "../migrations")]
async fn test_role_save_empty_key_fails(pool: PgPool) {
    let dto = SysRoleSaveDto {
        role_name: "name".to_string(),
        role_key: "  ".to_string(),
        remark: None,
    };
    assert!(service::save(&pool, &dto).await.is_err());
}
