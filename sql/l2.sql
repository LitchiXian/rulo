-- MySQL dump 10.13  Distrib 8.4.5, for Win64 (x86_64)
--
-- Host: localhost    Database: l2
-- ------------------------------------------------------
-- Server version	8.4.5

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `b_blog_article`
--

DROP TABLE IF EXISTS `b_blog_article`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
DROP TABLE IF EXISTS `b_blog_article`;
CREATE TABLE `b_blog_article`
(
    `id`             BIGINT(20) UNSIGNED NOT NULL COMMENT 'ID',
    `title`          VARCHAR(255)        NOT NULL COMMENT '文章标题',
    `content`        LONGTEXT            NOT NULL COMMENT 'Markdown内容',
    `user_id`        BIGINT(20) UNSIGNED NOT NULL COMMENT '作者ID',
    `user_name`      VARCHAR(255)        NOT NULL COMMENT '作者名称（冗余存储）',
    `published_time` DATETIME COMMENT '发布时间（当设置为公开时）',
    `is_published`   TINYINT(1)          NOT NULL DEFAULT 0 COMMENT '是否发布（0=私密，1=公开）',
    `is_deleted`     TINYINT(1)          NOT NULL DEFAULT 0 COMMENT '逻辑删除标志（0=正常，1=已删除）',
    `create_id`      BIGINT(20) UNSIGNED NOT NULL COMMENT '创建人ID',
    `create_time`    DATETIME            NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
    `update_id`      BIGINT(20) UNSIGNED NOT NULL COMMENT '更新人ID',
    `update_time`    DATETIME                     DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
    PRIMARY KEY (`id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_unicode_ci COMMENT '博客文章核心表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping routines for database 'l2'
--
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2025-07-10 17:44:31
