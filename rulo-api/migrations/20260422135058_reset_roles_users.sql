-- ================================================================
-- 重置角色与用户种子数据
-- 目标：
--   角色（4 个）：super_admin / admin / common / test
--   用户（5 个）：sadmin / admin / common01 / test01 / test02
--   所有用户初始密码：123456
-- 说明：
--   - 直接清空 sys_user / sys_role 及关联表，按新方案重新插入
--   - 不动菜单 sys_menu、权限 sys_permission（与之前迁移保持一致）
-- ================================================================

-- ---- 1. 清理旧关联 ----
TRUNCATE TABLE sys_user_role        RESTART IDENTITY CASCADE;
TRUNCATE TABLE sys_role_menu        RESTART IDENTITY CASCADE;
TRUNCATE TABLE sys_role_permission  RESTART IDENTITY CASCADE;
TRUNCATE TABLE sys_user             RESTART IDENTITY CASCADE;
TRUNCATE TABLE sys_role             RESTART IDENTITY CASCADE;

-- ---- 2. 角色 ----
INSERT INTO sys_role (id, role_name, role_key, is_super, is_active, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES
    (1, '超级管理员', 'super_admin', true,  true, false, 1, now(), 1, now(), '系统拥有者，跳过所有鉴权'),
    (2, '管理员',     'admin',       false, true, false, 1, now(), 1, now(), '业务管理员，拥有大部分管理权限'),
    (3, '普通用户',   'common',      false, true, false, 1, now(), 1, now(), '普通用户，仅基础查看权限'),
    (4, '测试角色',   'test',        false, true, false, 1, now(), 1, now(), '测试专用，权限按需分配');

-- ---- 3. 用户（密码均为 123456，argon2id 哈希复用 init.sql 中的 litchi 用户哈希） ----
INSERT INTO sys_user (id, user_name, nick_name, password, email, is_active, is_deleted, create_id, create_time, update_id, update_time, remark, avatar_url) VALUES
    (1001, 'sadmin',   '超级管理员',
        '$argon2id$v=19$m=19456,t=2,p=1$mAlYqqnt43F537NA/mydlw$qTrQG3C3T/b+8ycMYlGN5hX3XP3rTJ/COxRL1gliEW4',
        'sadmin@ruruo.com',   true, false, 1, now(), 1, now(), '系统超级管理员', NULL),
    (1002, 'admin',    '业务管理员',
        '$argon2id$v=19$m=19456,t=2,p=1$mAlYqqnt43F537NA/mydlw$qTrQG3C3T/b+8ycMYlGN5hX3XP3rTJ/COxRL1gliEW4',
        'admin@ruruo.com',    true, false, 1, now(), 1, now(), '业务管理员示范账号', NULL),
    (1003, 'common01', '普通用户01',
        '$argon2id$v=19$m=19456,t=2,p=1$mAlYqqnt43F537NA/mydlw$qTrQG3C3T/b+8ycMYlGN5hX3XP3rTJ/COxRL1gliEW4',
        'common01@ruruo.com', true, false, 1, now(), 1, now(), '普通用户示范账号', NULL),
    (1004, 'test01',   '测试用户01',
        '$argon2id$v=19$m=19456,t=2,p=1$mAlYqqnt43F537NA/mydlw$qTrQG3C3T/b+8ycMYlGN5hX3XP3rTJ/COxRL1gliEW4',
        'test01@ruruo.com',   true, false, 1, now(), 1, now(), '测试账号 01', NULL),
    (1005, 'test02',   '测试用户02',
        '$argon2id$v=19$m=19456,t=2,p=1$mAlYqqnt43F537NA/mydlw$qTrQG3C3T/b+8ycMYlGN5hX3XP3rTJ/COxRL1gliEW4',
        'test02@ruruo.com',   true, false, 1, now(), 1, now(), '测试账号 02', NULL);

-- ---- 4. 用户-角色绑定 ----
INSERT INTO sys_user_role (user_id, role_id) VALUES
    (1001, 1),  -- sadmin   -> super_admin
    (1002, 2),  -- admin    -> admin
    (1003, 3),  -- common01 -> common
    (1004, 4),  -- test01   -> test
    (1005, 4);  -- test02   -> test

-- ---- 5. 角色-权限绑定 ----
-- super_admin (role_id=1) 跳过鉴权，无需绑定，留空。
-- admin (role_id=2)：除"删除类"外所有管理权限 + 部门
INSERT INTO sys_role_permission (role_id, perm_id) VALUES
    (2,1),(2,2),(2,4),(2,5),(2,6),(2,7),(2,9),(2,10),
    (2,11),(2,12),(2,13),(2,14),(2,15),(2,16),(2,17),(2,19),(2,20),
    (2,21),(2,22),(2,23),(2,24),(2,25),(2,26),(2,27),(2,28),(2,29),(2,30),
    (2,31),(2,32),(2,33),(2,35),(2,36),(2,37),(2,38),(2,39),(2,40),(2,41),
    (2,1001001),(2,1001002),(2,1001003),(2,1001005),(2,1001006), -- 部门：list/save/update/detail/menu，无 remove
-- common (role_id=3)：基础查看 + 个人中心
    (3,21),(3,22),(3,23),(3,28),(3,29),(3,30),(3,31),(3,32),
-- test (role_id=4)：基础查看（去掉服务监控菜单）
    (4,21),(4,22),(4,23),(4,29),(4,30),(4,31),(4,32);

-- ---- 6. 角色-菜单绑定（仅审计日志默认隐藏控制；其他菜单走权限码） ----
INSERT INTO sys_role_menu (role_id, menu_id, is_hidden) VALUES
    (1, 302, false),  -- 超管：审计日志可见
    (2, 302, false);  -- 管理员：审计日志可见
