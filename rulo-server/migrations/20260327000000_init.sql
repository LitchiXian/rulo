-- ================================================================
-- RULO 数据库初始化脚本（合并所有迁移 + 当前全量数据）
-- 生成时间: 2026-03-27
-- 合并迁移: 20260310 / 20260311 / 20260320x2 / 20260324 / 20260326
-- ================================================================


-- ======================== SCHEMA ========================

-- 用户表（含 avatar_url，is_active 默认 TRUE=正常）
CREATE TABLE IF NOT EXISTS sys_user (
    id          BIGINT       PRIMARY KEY,
    user_name   VARCHAR(30)  NOT NULL,
    nick_name   VARCHAR(30)  NOT NULL,
    password    VARCHAR(255) NOT NULL,
    email       VARCHAR(50),
    is_active   BOOLEAN      NOT NULL DEFAULT TRUE,
    is_deleted  BOOLEAN      NOT NULL DEFAULT FALSE,
    create_id   BIGINT       NOT NULL,
    create_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_id   BIGINT       NOT NULL,
    update_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    remark      VARCHAR(500),
    avatar_url  VARCHAR(500) DEFAULT NULL
);

COMMENT ON TABLE  sys_user            IS '系统_用户信息表';
COMMENT ON COLUMN sys_user.id         IS 'ID';
COMMENT ON COLUMN sys_user.user_name  IS '用户账号';
COMMENT ON COLUMN sys_user.nick_name  IS '用户昵称';
COMMENT ON COLUMN sys_user.password   IS '密码（argon2id 哈希）';
COMMENT ON COLUMN sys_user.email      IS '用户邮箱';
COMMENT ON COLUMN sys_user.is_active  IS '账户状态（true=正常，false=冻结）';
COMMENT ON COLUMN sys_user.is_deleted IS '逻辑删除标志（false=正常，true=已删除）';
COMMENT ON COLUMN sys_user.create_id  IS '创建人ID';
COMMENT ON COLUMN sys_user.create_time IS '创建时间';
COMMENT ON COLUMN sys_user.update_id  IS '更新人ID';
COMMENT ON COLUMN sys_user.update_time IS '更新时间';
COMMENT ON COLUMN sys_user.remark     IS '备注';
COMMENT ON COLUMN sys_user.avatar_url IS '用户头像 URL（对象存储 key）';

-- 文章表
CREATE TABLE IF NOT EXISTS b_blog_article (
    id             BIGINT       PRIMARY KEY,
    title          VARCHAR(255) NOT NULL,
    content        TEXT         NOT NULL,
    user_id        BIGINT       NOT NULL,
    user_name      VARCHAR(255),
    published_time TIMESTAMP WITH TIME ZONE,
    is_published   BOOLEAN      NOT NULL DEFAULT FALSE,
    is_deleted     BOOLEAN      NOT NULL DEFAULT FALSE,
    create_id      BIGINT       NOT NULL,
    create_time    TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_id      BIGINT       NOT NULL,
    update_time    TIMESTAMP WITH TIME ZONE
);

COMMENT ON TABLE  b_blog_article               IS '博客文章核心表';
COMMENT ON COLUMN b_blog_article.id            IS 'ID';
COMMENT ON COLUMN b_blog_article.title         IS '文章标题';
COMMENT ON COLUMN b_blog_article.content       IS 'Markdown内容';
COMMENT ON COLUMN b_blog_article.user_id       IS '作者ID';
COMMENT ON COLUMN b_blog_article.user_name     IS '作者名称（冗余存储）';
COMMENT ON COLUMN b_blog_article.published_time IS '发布时间';
COMMENT ON COLUMN b_blog_article.is_published  IS '是否发布（false=私密，true=公开）';
COMMENT ON COLUMN b_blog_article.is_deleted    IS '逻辑删除标志（false=正常，true=已删除）';
COMMENT ON COLUMN b_blog_article.create_id     IS '创建人ID';
COMMENT ON COLUMN b_blog_article.create_time   IS '创建时间';
COMMENT ON COLUMN b_blog_article.update_id     IS '更新人ID';
COMMENT ON COLUMN b_blog_article.update_time   IS '更新时间';

-- 标签表
CREATE TABLE IF NOT EXISTS b_tag (
    id          BIGINT      PRIMARY KEY,
    name        VARCHAR(50) NOT NULL UNIQUE,
    remark      VARCHAR(500),
    status      BOOLEAN     NOT NULL DEFAULT TRUE,
    create_id   BIGINT      NOT NULL,
    create_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_id   BIGINT,
    update_time TIMESTAMP WITH TIME ZONE
);

COMMENT ON TABLE  b_tag             IS '标签主表';
COMMENT ON COLUMN b_tag.id          IS '标签ID';
COMMENT ON COLUMN b_tag.name        IS '标签名称';
COMMENT ON COLUMN b_tag.remark      IS '标签描述';
COMMENT ON COLUMN b_tag.status      IS '状态（false=禁用，true=启用）';
COMMENT ON COLUMN b_tag.create_id   IS '创建者用户ID';
COMMENT ON COLUMN b_tag.create_time IS '创建时间';
COMMENT ON COLUMN b_tag.update_id   IS '最后更新者用户ID';
COMMENT ON COLUMN b_tag.update_time IS '最后更新时间';

-- 文章标签关联表
CREATE TABLE IF NOT EXISTS b_article_tag (
    article_id BIGINT NOT NULL,
    tag_id     BIGINT NOT NULL,
    PRIMARY KEY (article_id, tag_id),
    FOREIGN KEY (article_id) REFERENCES b_blog_article(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id)     REFERENCES b_tag(id)          ON DELETE CASCADE
);

COMMENT ON TABLE  b_article_tag            IS '文章与标签关联关系表';
COMMENT ON COLUMN b_article_tag.article_id IS '文章ID';
COMMENT ON COLUMN b_article_tag.tag_id     IS '标签ID';

-- 权限表（perm_code 唯一性由下方局部唯一索引保证，支持软删除后重建相同 code）
CREATE TABLE IF NOT EXISTS sys_permission (
    id          BIGINT       PRIMARY KEY,
    perm_code   VARCHAR(100) NOT NULL,
    perm_name   VARCHAR(50)  NOT NULL,
    perm_type   SMALLINT     NOT NULL DEFAULT 1,
    is_deleted  BOOLEAN      NOT NULL DEFAULT FALSE,
    create_id   BIGINT       NOT NULL,
    create_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_id   BIGINT       NOT NULL,
    update_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    remark      VARCHAR(500)
);

COMMENT ON TABLE  sys_permission             IS '系统_权限表（独立于菜单）';
COMMENT ON COLUMN sys_permission.id          IS 'ID';
COMMENT ON COLUMN sys_permission.perm_code   IS '权限标识，如 sys:user:list';
COMMENT ON COLUMN sys_permission.perm_name   IS '权限名称';
COMMENT ON COLUMN sys_permission.perm_type   IS '类型：1=API权限 2=菜单权限';
COMMENT ON COLUMN sys_permission.is_deleted  IS '逻辑删除标志（false=正常，true=已删除）';
COMMENT ON COLUMN sys_permission.create_id   IS '创建人ID';
COMMENT ON COLUMN sys_permission.create_time IS '创建时间';
COMMENT ON COLUMN sys_permission.update_id   IS '更新人ID';
COMMENT ON COLUMN sys_permission.update_time IS '更新时间';
COMMENT ON COLUMN sys_permission.remark      IS '备注';

-- 菜单表
CREATE TABLE IF NOT EXISTS sys_menu (
    id          BIGINT       PRIMARY KEY,
    parent_id   BIGINT       NOT NULL DEFAULT 0,
    perm_id     BIGINT,
    name        VARCHAR(50)  NOT NULL,
    menu_type   SMALLINT     NOT NULL DEFAULT 1,
    path        VARCHAR(200),
    component   VARCHAR(200),
    icon        VARCHAR(100),
    sort_order  INT          NOT NULL DEFAULT 0,
    is_hidden   BOOLEAN      NOT NULL DEFAULT FALSE,
    is_deleted  BOOLEAN      NOT NULL DEFAULT FALSE,
    create_id   BIGINT       NOT NULL,
    create_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_id   BIGINT       NOT NULL,
    update_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    remark      VARCHAR(500),
    FOREIGN KEY (perm_id) REFERENCES sys_permission(id) ON DELETE SET NULL
);

COMMENT ON TABLE  sys_menu             IS '系统_菜单表（UI展示层）';
COMMENT ON COLUMN sys_menu.id          IS 'ID';
COMMENT ON COLUMN sys_menu.parent_id   IS '父菜单ID，顶级为0';
COMMENT ON COLUMN sys_menu.perm_id     IS '关联权限ID（目录类可为空）';
COMMENT ON COLUMN sys_menu.name        IS '菜单名称';
COMMENT ON COLUMN sys_menu.menu_type   IS '类型：1=目录 2=菜单 3=按钮';
COMMENT ON COLUMN sys_menu.path        IS '路由路径';
COMMENT ON COLUMN sys_menu.component   IS '前端组件路径';
COMMENT ON COLUMN sys_menu.icon        IS '图标';
COMMENT ON COLUMN sys_menu.sort_order  IS '显示排序';
COMMENT ON COLUMN sys_menu.is_hidden   IS '系统默认隐藏（false=显示，true=隐藏），超管可见所有';
COMMENT ON COLUMN sys_menu.is_deleted  IS '逻辑删除标志（false=正常，true=已删除）';
COMMENT ON COLUMN sys_menu.create_id   IS '创建人ID';
COMMENT ON COLUMN sys_menu.create_time IS '创建时间';
COMMENT ON COLUMN sys_menu.update_id   IS '更新人ID';
COMMENT ON COLUMN sys_menu.update_time IS '更新时间';
COMMENT ON COLUMN sys_menu.remark      IS '备注';

-- 角色表
CREATE TABLE IF NOT EXISTS sys_role (
    id          BIGINT      PRIMARY KEY,
    role_name   VARCHAR(30) NOT NULL,
    role_key    VARCHAR(50) NOT NULL UNIQUE,
    is_super    BOOLEAN     NOT NULL DEFAULT FALSE,
    is_active   BOOLEAN     NOT NULL DEFAULT TRUE,
    is_deleted  BOOLEAN     NOT NULL DEFAULT FALSE,
    create_id   BIGINT      NOT NULL,
    create_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_id   BIGINT      NOT NULL,
    update_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    remark      VARCHAR(500)
);

COMMENT ON TABLE  sys_role             IS '系统_角色表';
COMMENT ON COLUMN sys_role.id          IS 'ID';
COMMENT ON COLUMN sys_role.role_name   IS '角色名称';
COMMENT ON COLUMN sys_role.role_key    IS '角色标识（唯一），如 admin / editor';
COMMENT ON COLUMN sys_role.is_super    IS '超管标志（true=超管，跳过所有权限/隐藏校验）';
COMMENT ON COLUMN sys_role.is_active   IS '角色状态（false=禁用，true=启用）';
COMMENT ON COLUMN sys_role.is_deleted  IS '逻辑删除标志（false=正常，true=已删除）';
COMMENT ON COLUMN sys_role.create_id   IS '创建人ID';
COMMENT ON COLUMN sys_role.create_time IS '创建时间';
COMMENT ON COLUMN sys_role.update_id   IS '更新人ID';
COMMENT ON COLUMN sys_role.update_time IS '更新时间';
COMMENT ON COLUMN sys_role.remark      IS '备注';

-- 用户角色关联表
CREATE TABLE IF NOT EXISTS sys_user_role (
    user_id BIGINT NOT NULL,
    role_id BIGINT NOT NULL,
    PRIMARY KEY (user_id, role_id),
    FOREIGN KEY (user_id) REFERENCES sys_user(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES sys_role(id) ON DELETE CASCADE
);

COMMENT ON TABLE  sys_user_role         IS '系统_用户角色关联表';
COMMENT ON COLUMN sys_user_role.user_id IS '用户ID';
COMMENT ON COLUMN sys_user_role.role_id IS '角色ID';

-- 角色权限关联表
CREATE TABLE IF NOT EXISTS sys_role_permission (
    role_id BIGINT NOT NULL,
    perm_id BIGINT NOT NULL,
    PRIMARY KEY (role_id, perm_id),
    FOREIGN KEY (role_id) REFERENCES sys_role(id)       ON DELETE CASCADE,
    FOREIGN KEY (perm_id) REFERENCES sys_permission(id) ON DELETE CASCADE
);

COMMENT ON TABLE  sys_role_permission         IS '系统_角色权限关联表（后端鉴权基于此表）';
COMMENT ON COLUMN sys_role_permission.role_id IS '角色ID';
COMMENT ON COLUMN sys_role_permission.perm_id IS '权限ID';

-- 角色菜单关联表
CREATE TABLE IF NOT EXISTS sys_role_menu (
    role_id   BIGINT  NOT NULL,
    menu_id   BIGINT  NOT NULL,
    is_hidden BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (role_id, menu_id),
    FOREIGN KEY (role_id) REFERENCES sys_role(id) ON DELETE CASCADE,
    FOREIGN KEY (menu_id) REFERENCES sys_menu(id) ON DELETE CASCADE
);

COMMENT ON TABLE  sys_role_menu           IS '系统_角色菜单关联表';
COMMENT ON COLUMN sys_role_menu.role_id   IS '角色ID';
COMMENT ON COLUMN sys_role_menu.menu_id   IS '菜单ID';
COMMENT ON COLUMN sys_role_menu.is_hidden IS '角色级别隐藏（true=隐藏），覆盖菜单默认隐藏';

-- 审计日志表
CREATE TABLE IF NOT EXISTS sys_audit_log (
    id           BIGINT       PRIMARY KEY,
    user_id      BIGINT,
    user_name    VARCHAR(30),
    method       VARCHAR(10)  NOT NULL,
    path         VARCHAR(500) NOT NULL,
    params       TEXT,
    status       SMALLINT     NOT NULL,
    duration_ms  BIGINT       NOT NULL,
    ip           VARCHAR(45),
    is_sensitive BOOLEAN      NOT NULL DEFAULT FALSE,
    created_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON TABLE  sys_audit_log              IS '系统_审计日志表';
COMMENT ON COLUMN sys_audit_log.id           IS 'ID（雪花ID）';
COMMENT ON COLUMN sys_audit_log.user_id      IS '操作用户ID';
COMMENT ON COLUMN sys_audit_log.user_name    IS '操作用户名';
COMMENT ON COLUMN sys_audit_log.method       IS 'HTTP 方法（GET/POST/PUT/DELETE）';
COMMENT ON COLUMN sys_audit_log.path         IS '请求路径';
COMMENT ON COLUMN sys_audit_log.params       IS '请求参数（POST body，敏感接口不记录）';
COMMENT ON COLUMN sys_audit_log.status       IS 'HTTP 响应状态码';
COMMENT ON COLUMN sys_audit_log.duration_ms  IS '请求耗时（毫秒）';
COMMENT ON COLUMN sys_audit_log.ip           IS '客户端IP';
COMMENT ON COLUMN sys_audit_log.is_sensitive IS '是否敏感操作';
COMMENT ON COLUMN sys_audit_log.created_time IS '创建时间';


-- ======================== INDEXES ========================

CREATE INDEX IF NOT EXISTS idx_user_username       ON sys_user(user_name);
CREATE INDEX IF NOT EXISTS idx_user_email          ON sys_user(email);

CREATE INDEX IF NOT EXISTS idx_article_user_id     ON b_blog_article(user_id);
CREATE INDEX IF NOT EXISTS idx_article_create_time ON b_blog_article(create_time DESC);
CREATE INDEX IF NOT EXISTS idx_article_is_deleted  ON b_blog_article(is_deleted);

CREATE INDEX IF NOT EXISTS idx_tag_status          ON b_tag(status);

CREATE INDEX IF NOT EXISTS idx_audit_user_id       ON sys_audit_log(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_created_time  ON sys_audit_log(created_time DESC);
CREATE INDEX IF NOT EXISTS idx_audit_path          ON sys_audit_log(path);
CREATE INDEX IF NOT EXISTS idx_audit_is_sensitive  ON sys_audit_log(is_sensitive);

-- perm_code 局部唯一索引（软删除后可重建相同 code）
CREATE UNIQUE INDEX IF NOT EXISTS uq_perm_code_active
    ON sys_permission (perm_code)
    WHERE is_deleted = false;


-- ======================== SEED DATA ========================

-- sys_user (4 rows)
INSERT INTO sys_user (id, user_name, nick_name, password, email, is_active, is_deleted, create_id, create_time, update_id, update_time, remark, avatar_url) VALUES
(1770261629457, 'litchi', 'litchi',
    '$argon2id$v=19$m=19456,t=2,p=1$mAlYqqnt43F537NA/mydlw$qTrQG3C3T/b+8ycMYlGN5hX3XP3rTJ/COxRL1gliEW4',
    '3468845278@qq.com', true, false, 1770261629457,
    '2026-02-05 11:20:29+08', 1770261629457, '2026-02-05 11:20:29+08', NULL, NULL),
(1773301283713, 'lzx111', 'lzx111',
    '$argon2id$v=19$m=19456,t=2,p=1$mAlYqqnt43F537NA/mydlw$qTrQG3C3T/b+8ycMYlGN5hX3XP3rTJ/COxRL1gliEW4',
    NULL, true, false, 1773301283713,
    '2026-03-12 15:41:23.713735+08', 1773301283713, '2026-03-25 09:44:14.388621+08', NULL,
    '2026/03/25/336dc3b5-1776-4c3b-951d-ebcc2b74a12d.jpg'),
(1773382000194, 'lzx222', 'lzx222',
    '$argon2id$v=19$m=19456,t=2,p=1$j9SakD6AwyT9gKD0WvtUgw$TnuynDkut6Pc+Dbl7F3hCdy1iZprOsYwpDfwUcvEgaQ',
    NULL, true, false, 1773382000194,
    '2026-03-13 14:06:40.194833+08', 1773382000194, '2026-03-13 14:06:40.194833+08', NULL, NULL),
(1773729892189, 'lzx333', 'lzx333',
    '$argon2id$v=19$m=19456,t=2,p=1$3uUS8K6+0jJoanZAdJs9TQ$tBV8MuvA3NxCi+GQR8yKggAuDiDqCt5ZduEeYeqbxeA',
    '333@qq.com', true, false, 1773729892189,
    '2026-03-17 14:44:52.189848+08', 1773729892189, '2026-03-17 14:44:52.189848+08', NULL, NULL)
ON CONFLICT (id) DO NOTHING;

-- b_blog_article (8 rows，含早期测试数据 user_id=1)
INSERT INTO b_blog_article (id, title, content, user_id, user_name, published_time, is_published, is_deleted, create_id, create_time, update_id, update_time) VALUES
(1770270749362, '测试文章',       '测试内容',           1770261629457, 'litchi', '2026-02-05 13:52:29+08',        true, false, 1770261629457, '2026-02-05 13:52:29+08',        1770261629457, '2026-02-05 13:52:29+08'),
(1770270771251, '1234',           '1234',               1770261629457, 'litchi', '2026-02-05 13:52:51+08',        true, false, 1770261629457, '2026-02-05 13:52:51+08',        1770261629457, '2026-02-05 13:52:51+08'),
(1770271009228, '新测试',         '新内容',             1770261629457, 'litchi', '2026-02-05 13:56:49+08',        true, false, 1770261629457, '2026-02-05 13:56:49+08',        1770261629457, '2026-02-05 13:56:49+08'),
(1770271044255, '1234',           '1234',               1770261629457, 'litchi', '2026-02-05 13:57:24+08',        true, false, 1770261629457, '2026-02-05 13:57:24+08',        1770261629457, '2026-02-05 13:57:24+08'),
(1770276215554, '1234',           '1234',               1,             NULL,     '2026-02-05 15:23:35.554798+08', true, false, 1,             '2026-02-05 15:23:35.554798+08', 1,             '2026-02-05 15:23:35.554798+08'),
(1770278041554, '551234',         '12341234',           1,             NULL,     '2026-02-05 15:54:01.554225+08', true, false, 1,             '2026-02-05 15:54:01.554225+08', 1,             '2026-02-05 15:54:01.554225+08'),
(1770278674913, '1234qeraw',      '爱上对方23412341234',1,             NULL,     '2026-02-05 16:04:34.913083+08', true, false, 1,             '2026-02-05 16:04:34.913083+08', 1,             '2026-02-05 16:04:34.913083+08'),
(1770280996061, '9999',           '9999',               1770261629457, 'litchi', '2026-02-05 16:43:16.061999+08', true, false, 1770261629457, '2026-02-05 16:43:16.061999+08', 1770261629457, '2026-02-05 16:43:16.061999+08')
ON CONFLICT (id) DO NOTHING;

-- b_tag (5 rows)
INSERT INTO b_tag (id, name, remark, status, create_id, create_time, update_id, update_time) VALUES
(1000000000001, 'Rust',       'Rust 语言相关', true, 1770261629457, '2026-03-10 17:42:54.420678+08', NULL, NULL),
(1000000000002, 'Java',       'Java 语言相关', true, 1770261629457, '2026-03-10 17:42:54.420678+08', NULL, NULL),
(1000000000003, 'Vue',        'Vue 前端框架',  true, 1770261629457, '2026-03-10 17:42:54.420678+08', NULL, NULL),
(1000000000004, 'PostgreSQL', '关系型数据库',  true, 1770261629457, '2026-03-10 17:42:54.420678+08', NULL, NULL),
(1000000000005, 'Linux',      'Linux 运维',    true, 1770261629457, '2026-03-10 17:42:54.420678+08', NULL, NULL)
ON CONFLICT (id) DO NOTHING;

-- sys_permission (41 条正式权限 + 1 条测试脏数据，如需清理请删除 id=7443214863368851456 的行)
INSERT INTO sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES
(1,  'sys:user:list',              '用户管理-列表',         1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '用户列表页面菜单权限'),
(2,  'sys:user:save',              '用户管理-新增',         1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '新增用户按钮权限'),
(3,  'sys:user:remove',            '用户管理-删除',         1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '删除用户按钮权限'),
(4,  'sys:user:update',            '用户管理-编辑',         1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '编辑用户按钮权限'),
(5,  'sys:user:detail',            '用户管理-详情',         1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '用户详情查看权限'),
(6,  'sys:role:list',              '角色管理-列表',         1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '角色列表页面菜单权限'),
(7,  'sys:role:save',              '角色管理-新增',         1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '新增角色按钮权限'),
(8,  'sys:role:remove',            '角色管理-删除',         1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '删除角色按钮权限'),
(9,  'sys:role:update',            '角色管理-编辑',         1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '编辑角色按钮权限'),
(10, 'sys:role:detail',            '角色管理-详情',         1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '角色详情查看权限'),
(11, 'sys:menu:list',              '菜单管理-列表',         1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '菜单列表页面菜单权限'),
(12, 'sys:menu:save',              '菜单管理-新增',         1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '新增菜单按钮权限'),
(13, 'sys:menu:remove',            '菜单管理-删除',         1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '删除菜单按钮权限'),
(14, 'sys:menu:update',            '菜单管理-编辑',         1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '编辑菜单按钮权限'),
(15, 'sys:menu:detail',            '菜单管理-详情',         1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '菜单详情查看权限'),
(16, 'sys:permission:list',        '权限管理-列表',         1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '权限列表页面菜单权限'),
(17, 'sys:permission:save',        '权限管理-新增',         1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '新增权限按钮权限'),
(18, 'sys:permission:remove',      '权限管理-删除',         1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '删除权限按钮权限'),
(19, 'sys:permission:update',      '权限管理-编辑',         1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '编辑权限按钮权限'),
(20, 'sys:permission:detail',      '权限管理-详情',         1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '权限详情查看权限'),
(21, 'sys:monitor:server-info',    '服务监控-查看',         1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '服务器信息监控菜单权限'),
(22, 'sys:auth:info',              '个人中心-查看',         1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '当前用户信息API权限，所有登录用户均有'),
(23, 'sys:auth:logout',            '退出登录',              1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '退出登录API权限，所有登录用户均有'),
(24, 'sys:user:menu',              '用户管理-页面入口',     2, false, 1770261629457, '2026-03-18 15:00:59.093681+08', 1770261629457, '2026-03-18 15:00:59.093681+08', '用户管理侧边栏菜单是否显示'),
(25, 'sys:role:menu',              '角色管理-页面入口',     2, false, 1770261629457, '2026-03-18 15:00:59.093681+08', 1770261629457, '2026-03-18 15:00:59.093681+08', '角色管理侧边栏菜单是否显示'),
(26, 'sys:menu:menu',              '菜单管理-页面入口',     2, false, 1770261629457, '2026-03-18 15:00:59.093681+08', 1770261629457, '2026-03-18 15:00:59.093681+08', '菜单管理侧边栏菜单是否显示'),
(27, 'sys:permission:menu',        '权限管理-页面入口',     2, false, 1770261629457, '2026-03-18 15:00:59.093681+08', 1770261629457, '2026-03-18 15:00:59.093681+08', '权限管理侧边栏菜单是否显示'),
(28, 'sys:monitor:menu',           '服务监控-页面入口',     2, false, 1770261629457, '2026-03-18 15:00:59.093681+08', 1770261629457, '2026-03-18 15:00:59.093681+08', '服务监控侧边栏菜单是否显示'),
(29, 'sys:changelog:menu',         '更新日志-页面入口',     2, false, 1770261629457, '2026-03-18 15:04:11.266865+08', 1770261629457, '2026-03-18 15:04:11.266865+08', '更新日志侧边栏菜单是否显示'),
(30, 'sys:about:menu',             '关于我们-页面入口',     2, false, 1770261629457, '2026-03-18 15:04:11.266865+08', 1770261629457, '2026-03-18 15:04:11.266865+08', '关于我们侧边栏菜单是否显示'),
(31, 'sys:project:menu',           '项目地址-页面入口',     2, false, 1770261629457, '2026-03-18 15:04:11.266865+08', 1770261629457, '2026-03-18 15:04:11.266865+08', '项目地址侧边栏菜单是否显示'),
(32, 'sys:profile:menu',           '个人中心-页面入口',     2, false, 1770261629457, '2026-03-18 15:09:17.373656+08', 1770261629457, '2026-03-18 15:09:17.373656+08', '个人中心侧边栏菜单是否显示'),
(33, 'sys:user:update-bind-roles', '用户管理-绑定角色',     1, false, 1770261629457, '2026-03-20 14:28:49.676336+08', 1770261629457, '2026-03-20 14:28:49.676336+08', '用户分配角色按钮权限'),
(34, 'sys:user:list-bind-roles',   '用户管理-查看绑定角色', 1, false, 1770261629457, '2026-03-20 14:28:49.676336+08', 1770261629457, '2026-03-20 14:28:49.676336+08', '查询用户已绑定角色列表'),
(35, 'sys:role:update-bind-menus', '角色管理-绑定菜单',     1, false, 1770261629457, '2026-03-20 14:28:49.676336+08', 1770261629457, '2026-03-20 14:28:49.676336+08', '角色分配菜单按钮权限'),
(36, 'sys:role:update-bind-perms', '角色管理-绑定权限',     1, false, 1770261629457, '2026-03-20 14:28:49.676336+08', 1770261629457, '2026-03-20 14:28:49.676336+08', '角色分配权限按钮权限'),
(37, 'sys:role:list-bind-menus',   '角色管理-查看绑定菜单', 1, false, 1770261629457, '2026-03-20 14:28:49.676336+08', 1770261629457, '2026-03-20 14:28:49.676336+08', '查询角色已绑定菜单列表'),
(38, 'sys:role:list-bind-perms',   '角色管理-查看绑定权限', 1, false, 1770261629457, '2026-03-20 14:28:49.676336+08', 1770261629457, '2026-03-20 14:28:49.676336+08', '查询角色已绑定权限列表'),
(39, 'sys:audit:list',             '审计日志-列表',         1, false, 1770261629457, '2026-03-25 10:32:16.369336+08', 1770261629457, '2026-03-25 10:32:16.369336+08', NULL),
(40, 'sys:audit:menu',             '审计日志-页面入口',     2, false, 1770261629457, '2026-03-25 10:32:16.369336+08', 1770261629457, '2026-03-25 10:32:16.369336+08', NULL),
(41, 'ai:ask:chat',                'AI对话',                1, false, 1770261629457, '2026-03-26 15:09:13.994767+08', 1770261629457, '2026-03-26 15:09:13.994767+08', NULL),
-- 测试脏数据，如不需要可删除此行
(7443214863368851456, 'sys_user',  '测试菜单1-页面入口',    2, false, 7443214863368851456, '2026-03-27 16:38:31.671961+08', 7443214863368851456, '2026-03-27 16:38:31.671961+08', NULL)
ON CONFLICT (id) DO NOTHING;

-- sys_menu (13 rows)，依赖 sys_permission，按 parent_id 优先顺序插入
INSERT INTO sys_menu (id, parent_id, perm_id, name, menu_type, path, component, icon, sort_order, is_hidden, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES
(200, 0,   NULL, '系统管理', 1, '/system',            NULL,                                    'Setting',    1000, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL),
(300, 0,   NULL, '系统监控', 1, '/monitor',           NULL,                                    'Monitor',    1010, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL),
(400, 0,   NULL, '其他',     1, '/other',             NULL,                                    'MoreFilled', 1020, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL),
(500, 0,   29,   '更新日志', 2, '/changelog',         'view/changelog/index.vue',              'Notebook',   1030, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL),
(201, 200, 24,   '用户管理', 2, '/system/user',       'view/system/user/index.vue',            'UserFilled', 10,   false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL),
(202, 200, 25,   '角色管理', 2, '/system/role',       'view/system/role/index.vue',            'Key',        20,   false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL),
(203, 200, 26,   '菜单管理', 2, '/system/menu',       'view/system/menu/index.vue',            'Menu',       30,   false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL),
(204, 200, 27,   '权限管理', 2, '/system/permission', 'view/system/permission/index.vue',      'Lock',       40,   false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL),
(301, 300, 28,   '服务监控', 2, '/monitor/server',    'view/monitor/server/index.vue',         'DataLine',   10,   false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL),
(302, 300, 40,   '审计日志', 2, '/monitor/audit',     'view/monitor/audit/index.vue',          'Tickets',    20,   false, false, 1770261629457, '2026-03-25 10:32:16.369336+08', 1770261629457, '2026-03-25 10:32:16.369336+08', NULL),
(401, 400, 30,   '关于我们', 2, '/other/about',       'view/other/about/index.vue',            'InfoFilled', 10,   false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL),
(402, 400, 31,   '项目地址', 2, 'https://github.com/LitchiXian/rulo', NULL,                   'Link',       20,   false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-20 14:49:26.636470+08', NULL),
(403, 400, 32,   '个人中心', 2, '/profile',           'view/profile/index.vue',                'User',       30,   true,  false, 1770261629457, '2026-03-18 15:09:17.373656+08', 1770261629457, '2026-03-18 15:09:17.373656+08', NULL)
ON CONFLICT (id) DO NOTHING;

-- sys_role (4 rows)
INSERT INTO sys_role (id, role_name, role_key, is_super, is_active, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES
(1, '超级管理员', 'super_admin', true,  true, false, 1770261629457, '2026-03-18 15:13:03.283803+08', 1770261629457, '2026-03-18 15:13:03.283803+08', '拥有所有权限，跳过所有鉴权校验'),
(2, '管理员',     'admin',       false, true, false, 1770261629457, '2026-03-18 15:13:03.283803+08', 1770261629457, '2026-03-18 15:13:03.283803+08', '门户管理员，拥有大部分管理权限'),
(3, '普通角色',   'user',        false, true, false, 1770261629457, '2026-03-18 15:13:03.283803+08', 1770261629457, '2026-03-20 11:06:11.461463+08', '普通用户，仅基础查看权限'),
(4, '测试角色',   'tester',      false, true, false, 1770261629457, '2026-03-18 15:13:03.283803+08', 1770261629457, '2026-03-20 17:02:47.085602+08', '测试专用角色，权限按需分配,我要写得很长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长')
ON CONFLICT (id) DO NOTHING;

-- sys_user_role (4 rows)
INSERT INTO sys_user_role (user_id, role_id) VALUES
(1770261629457, 1),
(1773301283713, 2),
(1773382000194, 3),
(1773729892189, 4)
ON CONFLICT (user_id, role_id) DO NOTHING;

-- sys_role_permission (86 rows)
INSERT INTO sys_role_permission (role_id, perm_id) VALUES
-- 超级管理员 (34 perms: 1-32, 39, 40)
(1,1),(1,2),(1,3),(1,4),(1,5),(1,6),(1,7),(1,8),(1,9),(1,10),
(1,11),(1,12),(1,13),(1,14),(1,15),(1,16),(1,17),(1,18),(1,19),(1,20),
(1,21),(1,22),(1,23),(1,24),(1,25),(1,26),(1,27),(1,28),(1,29),(1,30),
(1,31),(1,32),(1,39),(1,40),
-- 管理员 (37 perms: 1,2,4-7,9-17,19-33,35-41，无删除类权限3/8/18)
(2,1),(2,2),(2,4),(2,5),(2,6),(2,7),(2,9),(2,10),
(2,11),(2,12),(2,13),(2,14),(2,15),(2,16),(2,17),(2,19),(2,20),
(2,21),(2,22),(2,23),(2,24),(2,25),(2,26),(2,27),(2,28),(2,29),(2,30),
(2,31),(2,32),(2,33),(2,35),(2,36),(2,37),(2,38),(2,39),(2,40),(2,41),
-- 普通角色 (8 perms)
(3,21),(3,22),(3,23),(3,28),(3,29),(3,30),(3,31),(3,32),
-- 测试角色 (7 perms)
(4,21),(4,22),(4,23),(4,29),(4,30),(4,31),(4,32)
ON CONFLICT (role_id, perm_id) DO NOTHING;

-- sys_role_menu (2 rows)
INSERT INTO sys_role_menu (role_id, menu_id, is_hidden) VALUES
(1, 302, false),
(2, 302, false)
ON CONFLICT (role_id, menu_id) DO NOTHING;
