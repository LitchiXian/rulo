/*
 Navicat Premium Data Transfer

 Source Server         : localhost_3306
 Source Server Type    : MySQL
 Source Server Version : 80405
 Source Host           : localhost:3306
 Source Schema         : l2

 Target Server Type    : PostgreSQL
 Target Server Version : 16.0+
 File Encoding         : UTF8

 Date: 06/08/2025 15:02:56
 Converted to PostgreSQL
*/

SET client_encoding = 'UTF8';

-- ----------------------------
-- Table structure for b_article
-- ----------------------------
DROP TABLE IF EXISTS b_article;
CREATE TABLE b_article
(
    id             BIGINT       NOT NULL,
    title          VARCHAR(255) NOT NULL,
    content        TEXT         NOT NULL,
    user_id        BIGINT       NOT NULL,
    user_name      VARCHAR(255)          DEFAULT NULL,
    published_time TIMESTAMP             DEFAULT NULL,
    is_published   SMALLINT     NOT NULL DEFAULT 0,
    is_deleted     SMALLINT     NOT NULL DEFAULT 0,
    create_id      BIGINT       NOT NULL,
    create_time    TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_id      BIGINT       NOT NULL,
    update_time    TIMESTAMP             DEFAULT NULL,
    PRIMARY KEY (id)
);

COMMENT ON TABLE b_article IS '博客文章核心表';
COMMENT ON COLUMN b_article.id IS 'ID';
COMMENT ON COLUMN b_article.title IS '文章标题';
COMMENT ON COLUMN b_article.content IS 'Markdown内容';
COMMENT ON COLUMN b_article.user_id IS '作者ID';
COMMENT ON COLUMN b_article.user_name IS '作者名称（冗余存储）';
COMMENT ON COLUMN b_article.published_time IS '发布时间（当设置为公开时）';
COMMENT ON COLUMN b_article.is_published IS '是否发布（0=私密，1=公开）';
COMMENT ON COLUMN b_article.is_deleted IS '逻辑删除标志（0=正常，1=已删除）';
COMMENT ON COLUMN b_article.create_id IS '创建人ID';
COMMENT ON COLUMN b_article.create_time IS '创建时间';
COMMENT ON COLUMN b_article.update_id IS '更新人ID';
COMMENT ON COLUMN b_article.update_time IS '更新时间';

-- Create trigger for update_time auto-update
CREATE OR REPLACE FUNCTION update_b_article_update_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.update_time = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_b_article_update_time
BEFORE UPDATE ON b_article
FOR EACH ROW
EXECUTE FUNCTION update_b_article_update_time();

-- ----------------------------
-- Table structure for b_article_tag
-- ----------------------------
DROP TABLE IF EXISTS b_article_tag;
CREATE TABLE b_article_tag
(
    article_id BIGINT NOT NULL,
    tag_id     BIGINT NOT NULL,
    PRIMARY KEY (article_id, tag_id)
);

COMMENT ON TABLE b_article_tag IS '文章与标签关联关系表';
COMMENT ON COLUMN b_article_tag.article_id IS '文章ID，关联文章表主键';
COMMENT ON COLUMN b_article_tag.tag_id IS '标签ID，关联标签表主键';

-- ----------------------------
-- Table structure for b_tag
-- ----------------------------
DROP TABLE IF EXISTS b_tag;
CREATE TABLE b_tag
(
    id          BIGINT       NOT NULL,
    name        VARCHAR(50)  NOT NULL,
    remark      VARCHAR(500)          DEFAULT NULL,
    status      SMALLINT     NOT NULL DEFAULT 1,
    create_id   BIGINT       NOT NULL,
    create_time TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_id   BIGINT                DEFAULT NULL,
    update_time TIMESTAMP             DEFAULT NULL,
    PRIMARY KEY (id),
    CONSTRAINT uniq_name UNIQUE (name)
);

COMMENT ON TABLE b_tag IS '标签主表';
COMMENT ON COLUMN b_tag.id IS '标签ID，雪花算法生成的非负唯一ID';
COMMENT ON COLUMN b_tag.name IS '标签名称';
COMMENT ON COLUMN b_tag.remark IS '标签描述';
COMMENT ON COLUMN b_tag.status IS '状态（0-禁用，1-启用）';
COMMENT ON COLUMN b_tag.create_id IS '创建者用户ID';
COMMENT ON COLUMN b_tag.create_time IS '创建时间';
COMMENT ON COLUMN b_tag.update_id IS '最后更新者用户ID';
COMMENT ON COLUMN b_tag.update_time IS '最后更新时间';

-- Create trigger for update_time auto-update
CREATE OR REPLACE FUNCTION update_b_tag_update_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.update_time = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_b_tag_update_time
BEFORE UPDATE ON b_tag
FOR EACH ROW
EXECUTE FUNCTION update_b_tag_update_time();

-- ----------------------------
-- Table structure for sys_user
-- ----------------------------
DROP TABLE IF EXISTS sys_user;
CREATE TABLE sys_user
(
    id          BIGINT       NOT NULL,
    user_name   VARCHAR(30)  NOT NULL,
    nick_name   VARCHAR(30)  NOT NULL,
    password    VARCHAR(255) NOT NULL,
    email       VARCHAR(50)           DEFAULT NULL,
    is_active   SMALLINT     NOT NULL DEFAULT 0,
    is_deleted  SMALLINT     NOT NULL DEFAULT 0,
    create_id   BIGINT       NOT NULL,
    create_time TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    update_id   BIGINT       NOT NULL,
    update_time TIMESTAMP             DEFAULT NULL,
    remark      VARCHAR(500)          DEFAULT NULL,
    PRIMARY KEY (id)
);

COMMENT ON TABLE sys_user IS '用户信息表';
COMMENT ON COLUMN sys_user.id IS 'ID';
COMMENT ON COLUMN sys_user.user_name IS '用户账号';
COMMENT ON COLUMN sys_user.nick_name IS '用户昵称';
COMMENT ON COLUMN sys_user.password IS '密码（bcrypt哈希结果）';
COMMENT ON COLUMN sys_user.email IS '用户邮箱';
COMMENT ON COLUMN sys_user.is_active IS '账户状态（0=正常，1=异常）';
COMMENT ON COLUMN sys_user.is_deleted IS '逻辑删除标志（0=正常，1=已删除）';
COMMENT ON COLUMN sys_user.create_id IS '创建人ID';
COMMENT ON COLUMN sys_user.create_time IS '创建时间';
COMMENT ON COLUMN sys_user.update_id IS '更新人ID';
COMMENT ON COLUMN sys_user.update_time IS '更新时间';
COMMENT ON COLUMN sys_user.remark IS '备注';

-- Create trigger for update_time auto-update
CREATE OR REPLACE FUNCTION update_sys_user_update_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.update_time = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_sys_user_update_time
BEFORE UPDATE ON sys_user
FOR EACH ROW
EXECUTE FUNCTION update_sys_user_update_time();
