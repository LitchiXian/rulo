-- 重命名表名以匹配代码中的表名引用
-- 将 b_article 改为 b_blog_article

USE l2;

-- 重命名主表
ALTER TABLE b_article RENAME TO b_blog_article;

-- 更新表注释（如果需要）
ALTER TABLE b_blog_article COMMENT = '博客文章核心表';

-- 验证表是否重命名成功
SHOW TABLES;