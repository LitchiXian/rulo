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

 Date: 17/07/2025 17:33:55
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for b_blog_article
-- ----------------------------
DROP TABLE IF EXISTS `b_blog_article`;
CREATE TABLE `b_blog_article`
(
    `id`             bigint(0) UNSIGNED                                            NOT NULL COMMENT 'ID',
    `title`          varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '文章标题',
    `content`        longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci     NOT NULL COMMENT 'Markdown内容',
    `user_id`        bigint(0) UNSIGNED                                            NOT NULL COMMENT '作者ID',
    `user_name`      varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NULL     DEFAULT NULL COMMENT '作者名称（冗余存储）',
    `published_time` datetime(0)                                                   NULL     DEFAULT NULL COMMENT '发布时间（当设置为公开时）',
    `is_published`   tinyint(1) UNSIGNED ZEROFILL                                  NOT NULL DEFAULT 0 COMMENT '是否发布（0=私密，1=公开）',
    `is_deleted`     tinyint(1) UNSIGNED ZEROFILL                                  NOT NULL DEFAULT 0 COMMENT '逻辑删除标志（0=正常，1=已删除）',
    `create_id`      bigint(0) UNSIGNED                                            NOT NULL COMMENT '创建人ID',
    `create_time`    datetime(0)                                                   NOT NULL DEFAULT CURRENT_TIMESTAMP(0) COMMENT '创建时间',
    `update_id`      bigint(0) UNSIGNED                                            NOT NULL COMMENT '更新人ID',
    `update_time`    datetime(0)                                                   NULL     DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP(0) COMMENT '更新时间',
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB
  CHARACTER SET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci COMMENT = '博客文章核心表'
  ROW_FORMAT = Dynamic;

-- ----------------------------
-- Table structure for sys_user
-- ----------------------------
DROP TABLE IF EXISTS `sys_user`;
CREATE TABLE `sys_user`
(
    `id`          bigint(0) UNSIGNED                                            NOT NULL COMMENT 'ID',
    `user_name`   varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci  NOT NULL COMMENT '用户账号',
    `nick_name`   varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci  NOT NULL COMMENT '用户昵称',
    `password`    varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT '密码（bcrypt哈希结果）',
    `email`       varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci  NULL     DEFAULT NULL COMMENT '用户邮箱',
    `is_active`   tinyint(3) UNSIGNED ZEROFILL                                  NOT NULL DEFAULT 000 COMMENT '账户状态（0=正常，1=异常）',
    `is_deleted`  tinyint(3) UNSIGNED ZEROFILL                                  NOT NULL DEFAULT 000 COMMENT '逻辑删除标志（0=正常，1=已删除）',
    `create_id`   bigint(0) UNSIGNED                                            NOT NULL COMMENT '创建人ID',
    `create_time` datetime(0)                                                   NOT NULL DEFAULT CURRENT_TIMESTAMP(0) COMMENT '创建时间',
    `update_id`   bigint(0) UNSIGNED                                            NOT NULL COMMENT '更新人ID',
    `update_time` datetime(0)                                                   NULL     DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP(0) COMMENT '更新时间',
    `remark`      varchar(500) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NULL     DEFAULT NULL COMMENT '备注',
    PRIMARY KEY (`id`) USING BTREE
) ENGINE = InnoDB
  CHARACTER SET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci COMMENT = '用户信息表'
  ROW_FORMAT = Dynamic;

SET FOREIGN_KEY_CHECKS = 1;
