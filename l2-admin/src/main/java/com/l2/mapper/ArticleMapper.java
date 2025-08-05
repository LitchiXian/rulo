package com.l2.mapper;

import com.l2.domain.Article;
import com.baomidou.mybatisplus.core.mapper.BaseMapper;

import java.util.List;

/**
* @author Administrator
* @description 针对表【b_article(博客文章核心表)】的数据库操作Mapper
* @createDate 2025-07-10 15:08:54
* @Entity com.l2.entity.Article
*/
public interface ArticleMapper extends BaseMapper<Article> {

    List<Article> listBlogArticle();

}




