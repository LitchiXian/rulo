use rulo_app::system::permission::model::{
    SysPermissionListDto, SysPermissionSaveDto, SysPermissionUpdateDto,
};
use rulo_app::system::permission::service;
use rulo_common::model::IdsDto;
use sqlx::PgPool;

#[sqlx::test(migrations = "../migrations")]
async fn test_permission_crud(pool: PgPool) {
    // 1. save
    let save_dto = SysPermissionSaveDto {
        perm_code: "test:read".to_string(),
        perm_name: "测试读取".to_string(),
        perm_type: 1,
        remark: None,
    };
    let result = service::save(&pool, &save_dto).await.unwrap();
    let perm = result.take_data().unwrap();
    assert_eq!(perm.perm_code, "test:read");
    let perm_id = perm.id;

    // 2. detail
    let detail_dto = rulo_common::model::IdDto { id: perm_id };
    let result = service::detail(&pool, &detail_dto).await.unwrap();
    assert_eq!(result.take_data().unwrap().perm_name, "测试读取");

    // 3. update
    let update_dto = SysPermissionUpdateDto {
        id: perm_id,
        perm_name: Some("更新名称".to_string()),
        remark: None,
    };
    service::update(&pool, &update_dto).await.unwrap();
    let result = service::detail(&pool, &detail_dto).await.unwrap();
    assert_eq!(result.take_data().unwrap().perm_name, "更新名称");

    // 4. list
    let list_dto = SysPermissionListDto {
        perm_code: Some("test".to_string()),
        perm_name: None,
        perm_type: Some(1),
        create_start_time: None,
        create_end_time: None,
        page_num: None,
        page_size: None,
    };
    let result = service::list(&pool, &list_dto).await.unwrap();
    let page = result.take_data().unwrap();
    assert_eq!(page.total, 1);

    // 5. remove
    let ids_dto = IdsDto { ids: vec![perm_id] };
    service::remove(&pool, &ids_dto).await.unwrap();
    assert!(service::detail(&pool, &detail_dto).await.is_err());
}

#[sqlx::test(migrations = "../migrations")]
async fn test_permission_save_duplicate_code_fails(pool: PgPool) {
    let dto = SysPermissionSaveDto {
        perm_code: "dup:code".to_string(),
        perm_name: "第一个".to_string(),
        perm_type: 1,
        remark: None,
    };
    service::save(&pool, &dto).await.unwrap();

    let dto2 = SysPermissionSaveDto {
        perm_code: "dup:code".to_string(),
        perm_name: "第二个".to_string(),
        perm_type: 1,
        remark: None,
    };
    assert!(service::save(&pool, &dto2).await.is_err());
}

#[sqlx::test(migrations = "../migrations")]
async fn test_permission_save_invalid_type_fails(pool: PgPool) {
    let dto = SysPermissionSaveDto {
        perm_code: "t:r".to_string(),
        perm_name: "name".to_string(),
        perm_type: 3,
        remark: None,
    };
    assert!(service::save(&pool, &dto).await.is_err());
}
