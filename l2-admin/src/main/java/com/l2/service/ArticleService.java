package com.l2.service;

import com.l2.domain.Article;
import com.baomidou.mybatisplus.extension.service.IService;
import com.l2.domain.dto.SaveArticleDto;

import java.util.List;

/**
 * @author Administrator
 * @description 针对表【b_article(博客文章核心表)】的数据库操作Service
 * @createDate 2025-07-10 15:08:54
 */
public interface ArticleService extends IService<Article> {

    int save(SaveArticleDto dto);

    List<Article> listBlogArticle();

    Article getById(Long id);

    List<Article> getUserArticleList(long userId);
}
