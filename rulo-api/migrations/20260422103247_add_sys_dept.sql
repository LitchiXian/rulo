-- ================================================================
-- 部门表（树形结构）
-- ================================================================

CREATE TABLE IF NOT EXISTS sys_dept (
    id          BIGINT       PRIMARY KEY,
    parent_id   BIGINT       NOT NULL DEFAULT 0,
    name        VARCHAR(50)  NOT NULL,
    sort_order  INTEGER      NOT NULL DEFAULT 0,
    leader      VARCHAR(50),
    phone       VARCHAR(30),
    email       VARCHAR(50),
    is_active   BOOLEAN      NOT NULL DEFAULT TRUE,
    is_deleted  BOOLEAN      NOT NULL DEFAULT FALSE,
    create_id   BIGINT       NOT NULL,
    create_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_id   BIGINT       NOT NULL,
    update_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    remark      VARCHAR(500)
);

COMMENT ON TABLE  sys_dept            IS '系统_部门表';
COMMENT ON COLUMN sys_dept.id         IS 'ID';
COMMENT ON COLUMN sys_dept.parent_id  IS '父部门ID（0=顶级）';
COMMENT ON COLUMN sys_dept.name       IS '部门名称';
COMMENT ON COLUMN sys_dept.sort_order IS '排序';
COMMENT ON COLUMN sys_dept.leader     IS '负责人';
COMMENT ON COLUMN sys_dept.phone      IS '联系电话';
COMMENT ON COLUMN sys_dept.email      IS '邮箱';
COMMENT ON COLUMN sys_dept.is_active  IS '部门状态（true=启用，false=停用）';
COMMENT ON COLUMN sys_dept.is_deleted IS '逻辑删除标志（false=正常，true=已删除）';
COMMENT ON COLUMN sys_dept.create_id  IS '创建人ID';
COMMENT ON COLUMN sys_dept.create_time IS '创建时间';
COMMENT ON COLUMN sys_dept.update_id  IS '更新人ID';
COMMENT ON COLUMN sys_dept.update_time IS '更新时间';
COMMENT ON COLUMN sys_dept.remark     IS '备注';

CREATE INDEX IF NOT EXISTS idx_dept_parent_id ON sys_dept(parent_id);
CREATE INDEX IF NOT EXISTS idx_dept_is_deleted ON sys_dept(is_deleted);

-- 权限种子：API 权限 + 菜单入口权限
INSERT INTO sys_permission(id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES
    (1001001, 'sys:dept:list',   '部门-查询',         1, false, 1, now(), 1, now(), NULL),
    (1001002, 'sys:dept:save',   '部门-新增',         1, false, 1, now(), 1, now(), NULL),
    (1001003, 'sys:dept:update', '部门-修改',         1, false, 1, now(), 1, now(), NULL),
    (1001004, 'sys:dept:remove', '部门-删除',         1, false, 1, now(), 1, now(), NULL),
    (1001005, 'sys:dept:detail', '部门-详情',         1, false, 1, now(), 1, now(), NULL),
    (1001006, 'sys:dept:menu',   '部门管理-页面入口', 2, false, 1, now(), 1, now(), '部门管理侧边栏菜单是否显示')
ON CONFLICT DO NOTHING;

-- 菜单种子：挂在「系统管理」(parent_id=200) 下
INSERT INTO sys_menu (id, parent_id, perm_id, name, menu_type, path, component, icon, sort_order, is_hidden, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES
    (205, 200, 1001006, '部门管理', 2, '/system/dept', 'view/system/dept/index.vue', 'OfficeBuilding', 50, false, false, 1, now(), 1, now(), NULL)
ON CONFLICT (id) DO NOTHING;

-- 默认给超级管理员角色绑定菜单与权限（角色 id=1）
INSERT INTO sys_role_menu (role_id, menu_id) VALUES (1, 205) ON CONFLICT DO NOTHING;
INSERT INTO sys_role_permission (role_id, perm_id) VALUES
    (1, 1001001), (1, 1001002), (1, 1001003), (1, 1001004), (1, 1001005), (1, 1001006)
ON CONFLICT DO NOTHING;

-- 初始部门数据：如若科技为顶级，下设 4 个二级部门，研发部下再分前端/后端/测试组
INSERT INTO sys_dept (id, parent_id, name, sort_order, leader, phone, email, is_active, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES
    (100,   0,   '如若科技',   1,  '李先生', '13800000000', 'admin@ruruo.com', true, false, 1, now(), 1, now(), '总公司'),
    (101,   100, '研发部',     10, '张三',   '13800000001', 'dev@ruruo.com',   true, false, 1, now(), 1, now(), NULL),
    (102,   100, '产品部',     20, '李四',   '13800000002', 'pd@ruruo.com',    true, false, 1, now(), 1, now(), NULL),
    (103,   100, '运营部',     30, '王五',   '13800000003', 'op@ruruo.com',    true, false, 1, now(), 1, now(), NULL),
    (104,   100, '人力资源部', 40, '赵六',   '13800000004', 'hr@ruruo.com',    true, false, 1, now(), 1, now(), NULL),
    (10101, 101, '前端组',     10, '钱七',   '13800000011', NULL,              true, false, 1, now(), 1, now(), NULL),
    (10102, 101, '后端组',     20, '孙八',   '13800000012', NULL,              true, false, 1, now(), 1, now(), NULL),
    (10103, 101, '测试组',     30, '周九',   '13800000013', NULL,              true, false, 1, now(), 1, now(), NULL)
ON CONFLICT (id) DO NOTHING;
