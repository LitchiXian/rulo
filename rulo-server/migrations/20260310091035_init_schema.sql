-- 创建用户表
CREATE TABLE IF NOT EXISTS sys_user (
    id              BIGINT PRIMARY KEY,
    user_name       VARCHAR(30)  NOT NULL,
    nick_name       VARCHAR(30)  NOT NULL,
    password        VARCHAR(255) NOT NULL,
    email           VARCHAR(50),
    is_active       BOOLEAN NOT NULL DEFAULT FALSE,
    is_deleted      BOOLEAN NOT NULL DEFAULT FALSE,
    create_id       BIGINT NOT NULL,
    create_time     TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_id       BIGINT NOT NULL,
    update_time     TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    remark          VARCHAR(500)
);

COMMENT ON TABLE  sys_user             IS '系统_用户信息表';
COMMENT ON COLUMN sys_user.id          IS 'ID';
COMMENT ON COLUMN sys_user.user_name   IS '用户账号';
COMMENT ON COLUMN sys_user.nick_name   IS '用户昵称';
COMMENT ON COLUMN sys_user.password    IS '密码（bcrypt哈希结果）';
COMMENT ON COLUMN sys_user.email       IS '用户邮箱';
COMMENT ON COLUMN sys_user.is_active   IS '账户状态（false=正常，true=异常）';
COMMENT ON COLUMN sys_user.is_deleted  IS '逻辑删除标志（false=正常，true=已删除）';
COMMENT ON COLUMN sys_user.create_id   IS '创建人ID';
COMMENT ON COLUMN sys_user.create_time IS '创建时间';
COMMENT ON COLUMN sys_user.update_id   IS '更新人ID';
COMMENT ON COLUMN sys_user.update_time IS '更新时间';
COMMENT ON COLUMN sys_user.remark      IS '备注';

-- 创建文章表
CREATE TABLE IF NOT EXISTS b_blog_article (
    id              BIGINT PRIMARY KEY,
    title           VARCHAR(255) NOT NULL,
    content         TEXT NOT NULL,
    user_id         BIGINT NOT NULL,
    user_name       VARCHAR(255),
    published_time  TIMESTAMP WITH TIME ZONE,
    is_published    BOOLEAN NOT NULL DEFAULT FALSE,
    is_deleted      BOOLEAN NOT NULL DEFAULT FALSE,
    create_id       BIGINT NOT NULL,
    create_time     TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_id       BIGINT NOT NULL,
    update_time     TIMESTAMP WITH TIME ZONE
);

COMMENT ON TABLE  b_blog_article                IS '博客文章核心表';
COMMENT ON COLUMN b_blog_article.id             IS 'ID';
COMMENT ON COLUMN b_blog_article.title          IS '文章标题';
COMMENT ON COLUMN b_blog_article.content        IS 'Markdown内容';
COMMENT ON COLUMN b_blog_article.user_id        IS '作者ID';
COMMENT ON COLUMN b_blog_article.user_name      IS '作者名称（冗余存储）';
COMMENT ON COLUMN b_blog_article.published_time IS '发布时间';
COMMENT ON COLUMN b_blog_article.is_published   IS '是否发布（false=私密，true=公开）';
COMMENT ON COLUMN b_blog_article.is_deleted     IS '逻辑删除标志（false=正常，true=已删除）';
COMMENT ON COLUMN b_blog_article.create_id      IS '创建人ID';
COMMENT ON COLUMN b_blog_article.create_time    IS '创建时间';
COMMENT ON COLUMN b_blog_article.update_id      IS '更新人ID';
COMMENT ON COLUMN b_blog_article.update_time    IS '更新时间';

-- 创建标签表
CREATE TABLE IF NOT EXISTS b_tag (
    id          BIGINT PRIMARY KEY,
    name        VARCHAR(50)  NOT NULL UNIQUE,
    remark      VARCHAR(500),
    status      BOOLEAN NOT NULL DEFAULT TRUE,
    create_id   BIGINT NOT NULL,
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

-- 创建文章标签关联表
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

-- 索引
CREATE INDEX IF NOT EXISTS idx_article_user_id     ON b_blog_article(user_id);
CREATE INDEX IF NOT EXISTS idx_article_create_time ON b_blog_article(create_time DESC);
CREATE INDEX IF NOT EXISTS idx_article_is_deleted  ON b_blog_article(is_deleted);
CREATE INDEX IF NOT EXISTS idx_user_username       ON sys_user(user_name);
CREATE INDEX IF NOT EXISTS idx_user_email          ON sys_user(email);
CREATE INDEX IF NOT EXISTS idx_tag_status          ON b_tag(status);

-- 初始种子数据
-- 默认管理员用户（password = Admin@123 的 bcrypt hash，首次部署后请修改密码）
INSERT INTO sys_user (id, user_name, nick_name, password, email, is_active, is_deleted, create_id, create_time, update_id, update_time, remark)
VALUES (
    1770261629457,
    'litchi',
    'litchi',
    '$2b$12$placeholder_change_this_password_hash',
    '3468845278@qq.com',
    false,
    false,
    1770261629457,
    CURRENT_TIMESTAMP,
    1770261629457,
    CURRENT_TIMESTAMP,
    '超级管理员'
) ON CONFLICT (id) DO NOTHING;

-- 初始标签
INSERT INTO b_tag (id, name, remark, status, create_id, create_time, update_id, update_time)
VALUES
    (1000000000001, 'Rust',       'Rust 语言相关', true, 1770261629457, CURRENT_TIMESTAMP, NULL, NULL),
    (1000000000002, 'Java',       'Java 语言相关', true, 1770261629457, CURRENT_TIMESTAMP, NULL, NULL),
    (1000000000003, 'Vue',        'Vue 前端框架',  true, 1770261629457, CURRENT_TIMESTAMP, NULL, NULL),
    (1000000000004, 'PostgreSQL', '关系型数据库',  true, 1770261629457, CURRENT_TIMESTAMP, NULL, NULL),
    (1000000000005, 'Linux',      'Linux 运维',    true, 1770261629457, CURRENT_TIMESTAMP, NULL, NULL)
ON CONFLICT (id) DO NOTHING;
