/*
 Navicat Premium Data Transfer

 Source Server         : localhost_3306
 Source Server Type    : MySQL
 Source Server Version : 80405
 Source Host           : localhost:3306
 Source Schema         : l2

 Target Server Type    : MySQL
 Target Server Version : 80405
 File Encoding         : 65001

 Date: 06/08/2025 15:02:56
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for b_article
-- ----------------------------
DROP TABLE IF EXISTS `b_article`;
CREATE TABLE `b_article`
(
    `id`             bigint(0) UNSIGNED  NOT NULL COMMENT 'ID',
    `title`          varchar(255)        NOT NULL COMMENT '文章标题',
    `content`        longtext            NOT NULL COMMENT 'Markdown内容',
    `user_id`        bigint(0) UNSIGNED  NOT NULL COMMENT '作者ID',
    `user_name`      varchar(255)        NULL     DEFAULT NULL COMMENT '作者名称（冗余存储）',
    `published_time` datetime(0)         NULL     DEFAULT NULL COMMENT '发布时间（当设置为公开时）',
    `is_published`   tinyint(0) UNSIGNED NOT NULL DEFAULT 0 COMMENT '是否发布（0=私密，1=公开）',
    `is_deleted`     tinyint(0) UNSIGNED NOT NULL DEFAULT 0 COMMENT '逻辑删除标志（0=正常，1=已删除）',
    `create_id`      bigint(0) UNSIGNED  NOT NULL COMMENT '创建人ID',
    `create_time`    datetime(0)         NOT NULL DEFAULT CURRENT_TIMESTAMP(0) COMMENT '创建时间',
    `update_id`      bigint(0) UNSIGNED  NOT NULL COMMENT '更新人ID',
    `update_time`    datetime(0)         NULL     DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP(0) COMMENT '更新时间',
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB
  CHARACTER SET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci COMMENT = '博客文章核心表'
  ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for b_article_tag
-- ----------------------------
DROP TABLE IF EXISTS `b_article_tag`;
CREATE TABLE `b_article_tag`
(
    `article_id` bigint(0) UNSIGNED NOT NULL COMMENT '文章ID，关联文章表主键',
    `tag_id`     bigint(0) UNSIGNED NOT NULL COMMENT '标签ID，关联标签表主键',
    PRIMARY KEY (`article_id`, `tag_id`) USING BTREE
) ENGINE = InnoDB
  CHARACTER SET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci COMMENT = '文章与标签关联关系表'
  ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for b_tag
-- ----------------------------
DROP TABLE IF EXISTS `b_tag`;
CREATE TABLE `b_tag`
(
    `id`          bigint(0) UNSIGNED  NOT NULL COMMENT '标签ID，雪花算法生成的非负唯一ID',
    `name`        varchar(50)         NOT NULL COMMENT '标签名称',
    `remark`      varchar(500)        NULL     DEFAULT NULL COMMENT '标签描述',
    `status`      tinyint(0) UNSIGNED NOT NULL DEFAULT 1 COMMENT '状态（0-禁用，1-启用）',
    `create_id`   bigint(0) UNSIGNED  NOT NULL COMMENT '创建者用户ID',
    `create_time` datetime(0)         NOT NULL DEFAULT CURRENT_TIMESTAMP(0) COMMENT '创建时间',
    `update_id`   bigint(0) UNSIGNED  NULL     DEFAULT NULL COMMENT '最后更新者用户ID',
    `update_time` datetime(0)         NULL     DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP(0) COMMENT '最后更新时间',
    PRIMARY KEY (`id`) USING BTREE,
    UNIQUE INDEX `uniq_name` (`name`) USING BTREE
) ENGINE = InnoDB
  CHARACTER SET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci COMMENT = '标签主表'
  ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for sys_user
-- ----------------------------
DROP TABLE IF EXISTS `sys_user`;
CREATE TABLE `sys_user`
(
    `id`          bigint(0) UNSIGNED  NOT NULL COMMENT 'ID',
    `user_name`   varchar(30)         NOT NULL COMMENT '用户账号',
    `nick_name`   varchar(30)         NOT NULL COMMENT '用户昵称',
    `password`    varchar(255)        NOT NULL COMMENT '密码（bcrypt哈希结果）',
    `email`       varchar(50)         NULL     DEFAULT NULL COMMENT '用户邮箱',
    `is_active`   tinyint(0) UNSIGNED NOT NULL DEFAULT 0 COMMENT '账户状态（0=正常，1=异常）',
    `is_deleted`  tinyint(0) UNSIGNED NOT NULL DEFAULT 0 COMMENT '逻辑删除标志（0=正常，1=已删除）',
    `create_id`   bigint(0) UNSIGNED  NOT NULL COMMENT '创建人ID',
    `create_time` datetime(0)         NOT NULL DEFAULT CURRENT_TIMESTAMP(0) COMMENT '创建时间',
    `update_id`   bigint(0) UNSIGNED  NOT NULL COMMENT '更新人ID',
    `update_time` datetime(0)         NULL     DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP(0) COMMENT '更新时间',
    `remark`      varchar(500)        NULL     DEFAULT NULL COMMENT '备注',
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB
  CHARACTER SET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci COMMENT = '用户信息表'
  ROW_FORMAT = Dynamic;

SET FOREIGN_KEY_CHECKS = 1;
