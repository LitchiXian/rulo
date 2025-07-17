package com.l2.service;

import com.l2.domain.BlogArticle;
import com.baomidou.mybatisplus.extension.service.IService;
import com.l2.domain.dto.SaveBlogArticleDto;

import java.util.List;

/**
* @author Administrator
* @description 针对表【b_blog_article(博客文章核心表)】的数据库操作Service
* @createDate 2025-07-10 15:08:54
*/
public interface BlogArticleService extends IService<BlogArticle> {

    int save(SaveBlogArticleDto dto);

    List<BlogArticle> listBlogArticle();
}
