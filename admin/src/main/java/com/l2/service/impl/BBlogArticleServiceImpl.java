package com.l2.service.impl;

import java.util.Date;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.l2.config.SnowflakeConfig;
import com.l2.entity.BBlogArticle;
import com.l2.entity.dto.SaveBBlogArticleDto;
import com.l2.service.BBlogArticleService;
import com.l2.mapper.BBlogArticleMapper;
import lombok.RequiredArgsConstructor;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

/**
 * @author Administrator
 * @description 针对表【b_blog_article(博客文章核心表)】的数据库操作Service实现
 * @createDate 2025-07-10 15:08:54
 */
@Service
@RequiredArgsConstructor
public class BBlogArticleServiceImpl extends ServiceImpl<BBlogArticleMapper, BBlogArticle>
        implements BBlogArticleService {

    private final SnowflakeConfig snowflakeConfig;

    @Override
    public int save(SaveBBlogArticleDto dto) {
        BBlogArticle bBlogArticle = new BBlogArticle();
        bBlogArticle.setId(snowflakeConfig.snowflakeId());
        bBlogArticle.setTitle(dto.getTitle());
        bBlogArticle.setContent(dto.getContent());
        bBlogArticle.setUserId(1L);
        bBlogArticle.setUserName(dto.getUserName());
        Date date = new Date();
        bBlogArticle.setPublishedTime(date);
        bBlogArticle.setIsPublished(dto.getIsPublished());
        bBlogArticle.setIsDeleted(0);
        bBlogArticle.setCreateId(1L);
        bBlogArticle.setCreateTime(date);
        bBlogArticle.setUpdateId(1L);
        bBlogArticle.setUpdateTime(date);

        int insert = baseMapper.insert(bBlogArticle);
        return insert;
    }
}




