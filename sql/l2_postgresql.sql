-- PostgreSQL 数据库表结构
-- L2 博客系统

-- 删除现有表（如果存在）
DROP TABLE IF EXISTS b_article_tag CASCADE;
DROP TABLE IF EXISTS b_blog_article CASCADE;
DROP TABLE IF EXISTS b_tag CASCADE;
DROP TABLE IF EXISTS sys_user CASCADE;

-- 创建用户表
CREATE TABLE sys_user (
    id              BIGINT PRIMARY KEY,
    user_name       VARCHAR(30) NOT NULL,
    nick_name       VARCHAR(30) NOT NULL,
    password        VARCHAR(255) NOT NULL,
    email           VARCHAR(50),
    is_active       BOOLEAN NOT NULL DEFAULT FALSE,
    is_deleted      BOOLEAN NOT NULL DEFAULT FALSE,
    create_id       BIGINT NOT NULL,
    create_time     TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_id       BIGINT NOT NULL,
    update_time     TIMESTAMP WITH TIME ZONE,
    remark          VARCHAR(500)
);

COMMENT ON TABLE sys_user IS '用户信息表';
COMMENT ON COLUMN sys_user.id IS 'ID';
COMMENT ON COLUMN sys_user.user_name IS '用户账号';
COMMENT ON COLUMN sys_user.nick_name IS '用户昵称';
COMMENT ON COLUMN sys_user.password IS '密码（bcrypt哈希结果）';
COMMENT ON COLUMN sys_user.email IS '用户邮箱';
COMMENT ON COLUMN sys_user.is_active IS '账户状态（false=正常，true=异常）';
COMMENT ON COLUMN sys_user.is_deleted IS '逻辑删除标志（false=正常，true=已删除）';
COMMENT ON COLUMN sys_user.create_id IS '创建人ID';
COMMENT ON COLUMN sys_user.create_time IS '创建时间';
COMMENT ON COLUMN sys_user.update_id IS '更新人ID';
COMMENT ON COLUMN sys_user.update_time IS '更新时间';
COMMENT ON COLUMN sys_user.remark IS '备注';

-- 创建文章表
CREATE TABLE b_blog_article (
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

COMMENT ON TABLE b_blog_article IS '博客文章核心表';
COMMENT ON COLUMN b_blog_article.id IS 'ID';
COMMENT ON COLUMN b_blog_article.title IS '文章标题';
COMMENT ON COLUMN b_blog_article.content IS 'Markdown内容';
COMMENT ON COLUMN b_blog_article.user_id IS '作者ID';
COMMENT ON COLUMN b_blog_article.user_name IS '作者名称（冗余存储）';
COMMENT ON COLUMN b_blog_article.published_time IS '发布时间';
COMMENT ON COLUMN b_blog_article.is_published IS '是否发布（false=私密，true=公开）';
COMMENT ON COLUMN b_blog_article.is_deleted IS '逻辑删除标志（false=正常，true=已删除）';
COMMENT ON COLUMN b_blog_article.create_id IS '创建人ID';
COMMENT ON COLUMN b_blog_article.create_time IS '创建时间';
COMMENT ON COLUMN b_blog_article.update_id IS '更新人ID';
COMMENT ON COLUMN b_blog_article.update_time IS '更新时间';

-- 创建标签表
CREATE TABLE b_tag (
    id              BIGINT PRIMARY KEY,
    name            VARCHAR(50) NOT NULL UNIQUE,
    remark          VARCHAR(500),
    status          BOOLEAN NOT NULL DEFAULT TRUE,
    create_id       BIGINT NOT NULL,
    create_time     TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_id       BIGINT,
    update_time     TIMESTAMP WITH TIME ZONE
);

COMMENT ON TABLE b_tag IS '标签主表';
COMMENT ON COLUMN b_tag.id IS '标签ID';
COMMENT ON COLUMN b_tag.name IS '标签名称';
COMMENT ON COLUMN b_tag.remark IS '标签描述';
COMMENT ON COLUMN b_tag.status IS '状态（false=禁用，true=启用）';
COMMENT ON COLUMN b_tag.create_id IS '创建者用户ID';
COMMENT ON COLUMN b_tag.create_time IS '创建时间';
COMMENT ON COLUMN b_tag.update_id IS '最后更新者用户ID';
COMMENT ON COLUMN b_tag.update_time IS '最后更新时间';

-- 创建文章标签关联表
CREATE TABLE b_article_tag (
    article_id      BIGINT NOT NULL,
    tag_id          BIGINT NOT NULL,
    PRIMARY KEY (article_id, tag_id),
    FOREIGN KEY (article_id) REFERENCES b_blog_article(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES b_tag(id) ON DELETE CASCADE
);

COMMENT ON TABLE b_article_tag IS '文章与标签关联关系表';
COMMENT ON COLUMN b_article_tag.article_id IS '文章ID';
COMMENT ON COLUMN b_article_tag.tag_id IS '标签ID';

-- 创建索引
CREATE INDEX idx_article_user_id ON b_blog_article(user_id);
CREATE INDEX idx_article_create_time ON b_blog_article(create_time DESC);
CREATE INDEX idx_article_is_deleted ON b_blog_article(is_deleted);
CREATE INDEX idx_user_username ON sys_user(user_name);
CREATE INDEX idx_user_email ON sys_user(email);
CREATE INDEX idx_tag_status ON b_tag(status);
