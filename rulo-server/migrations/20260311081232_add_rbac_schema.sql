-- 权限表（核心,独立于菜单）
CREATE TABLE IF NOT EXISTS sys_permission (
    id          BIGINT PRIMARY KEY,
    perm_code   VARCHAR(100) NOT NULL UNIQUE,             -- 权限标识 sys:user:list
    perm_name   VARCHAR(50)  NOT NULL,                   -- 权限名称
    perm_type   SMALLINT     NOT NULL DEFAULT 1,         -- 1=API权限 2=菜单权限
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


-- 菜单表（UI展示层，关联权限）
CREATE TABLE IF NOT EXISTS sys_menu (
    id          BIGINT PRIMARY KEY,
    parent_id   BIGINT       NOT NULL DEFAULT 0,         -- 顶级为0
    perm_id     BIGINT,                                  -- 关联权限（可为空，纯目录无需权限）
    name        VARCHAR(50)  NOT NULL,
    menu_type   SMALLINT     NOT NULL DEFAULT 1,         -- 1=目录 2=菜单 3=按钮
    path        VARCHAR(200),
    component   VARCHAR(200),
    icon        VARCHAR(100),
    sort_order  INT          NOT NULL DEFAULT 0,
    is_hidden   BOOLEAN      NOT NULL DEFAULT FALSE,     -- 系统默认隐藏（未启用功能）
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
    id          BIGINT PRIMARY KEY,
    role_name   VARCHAR(30)  NOT NULL,
    role_key    VARCHAR(50)  NOT NULL UNIQUE,             -- 角色标识 admin / editor
    is_super    BOOLEAN      NOT NULL DEFAULT FALSE,      -- 是否超管（跳过权限校验）
    is_active   BOOLEAN      NOT NULL DEFAULT TRUE,
    is_deleted  BOOLEAN      NOT NULL DEFAULT FALSE,
    create_id   BIGINT       NOT NULL,
    create_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_id   BIGINT       NOT NULL,
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

-- 角色权限关联表（后端鉴权基于此表，不基于角色）
CREATE TABLE IF NOT EXISTS sys_role_permission (
    role_id BIGINT NOT NULL,
    perm_id BIGINT NOT NULL,
    PRIMARY KEY (role_id, perm_id),
    FOREIGN KEY (role_id) REFERENCES sys_role(id) ON DELETE CASCADE,
    FOREIGN KEY (perm_id) REFERENCES sys_permission(id) ON DELETE CASCADE
);

COMMENT ON TABLE  sys_role_permission         IS '系统_角色权限关联表（后端鉴权基于此表，不基于角色）';
COMMENT ON COLUMN sys_role_permission.role_id IS '角色ID';
COMMENT ON COLUMN sys_role_permission.perm_id IS '权限ID';

-- 角色菜单关联表（前端展示基于此表，覆盖菜单默认隐藏）
CREATE TABLE IF NOT EXISTS sys_role_menu (
    role_id   BIGINT  NOT NULL,
    menu_id   BIGINT  NOT NULL,
    is_hidden BOOLEAN NOT NULL DEFAULT FALSE,            -- 角色级别隐藏覆盖
    PRIMARY KEY (role_id, menu_id),
    FOREIGN KEY (role_id) REFERENCES sys_role(id) ON DELETE CASCADE,
    FOREIGN KEY (menu_id) REFERENCES sys_menu(id) ON DELETE CASCADE
);

COMMENT ON TABLE  sys_role_menu           IS '系统_角色菜单关联表';
COMMENT ON COLUMN sys_role_menu.role_id   IS '角色ID';
COMMENT ON COLUMN sys_role_menu.menu_id   IS '菜单ID';
COMMENT ON COLUMN sys_role_menu.is_hidden IS '角色级别隐藏（true=隐藏），覆盖菜单默认隐藏';

