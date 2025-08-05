package com.l2.service.impl;

import com.baomidou.mybatisplus.core.conditions.query.LambdaQueryWrapper;
import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.l2.domain.Article;
import com.l2.domain.dto.SaveArticleDto;
import com.l2.common.exception.ErrorCodeEnum;
import com.l2.common.exception.ServiceException;
import com.l2.mapper.ArticleMapper;
import com.l2.service.ArticleService;
import com.l2.framework.util.SaTokenUtil;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.Date;
import java.util.List;

/**
 * @author Administrator
 * @description 针对表【b_article(博客文章核心表)】的数据库操作Service实现
 * @createDate 2025-07-10 15:08:54
 */
@Service
@RequiredArgsConstructor
public class ArticleServiceImpl extends ServiceImpl<ArticleMapper, Article>
        implements ArticleService {

//    private final BBlogArticleMapper blogArticleMapper;

    @Override
    public int save(SaveArticleDto dto) {
        Article article = new Article();
//        bBlogArticle.setId(snowflakeConfig.snowflakeId());
        article.setTitle(dto.getTitle());
        article.setContent(dto.getContent());
        article.setUserId(SaTokenUtil.getLoginId());
        article.setUserName(SaTokenUtil.getLoginUserFullInfo().getSysUser().getNickName());
        Date date = new Date();
        article.setPublishedTime(date);
        article.setIsPublished(dto.getIsPublished());
//        bBlogArticle.setIsDeleted(0);
//        bBlogArticle.setCreateId(1L);
//        bBlogArticle.setCreateTime(date);
//        bBlogArticle.setUpdateId(1L);
//        bBlogArticle.setUpdateTime(date);

        int insert = baseMapper.insert(article);
        return insert;
    }

    @Override
    public List<Article> listBlogArticle() {
        return baseMapper.listBlogArticle();
    }

    @Override
    public Article getById(Long id) {
        Article article = baseMapper.selectById(id);
        // 文章不存在时直接返回null
        if (article == null) {
            return null;
        }

        // 只处理未发布的文章（isPublished=0表示未发布）
        if (article.getIsPublished() == 0) {
            // 获取当前登录用户ID（未登录时SaToken会自动抛出异常）
            Long loginId = SaTokenUtil.getLoginId();

            // 权限校验：当前用户必须为文章作者
            if (!loginId.equals(article.getUserId())) {
                throw new ServiceException(ErrorCodeEnum.UNAUTHORIZED_ACCESS);
            }
        }
        return article;
    }

    @Override
    public List<Article> getUserArticleList(long userId) {
        LambdaQueryWrapper<Article> queryWrapper = new LambdaQueryWrapper<>();
        Long loginId = SaTokenUtil.getLoginId();
        if (loginId == null || loginId != userId) {
            queryWrapper.eq(Article::getIsPublished, 1);
        }
        queryWrapper.eq(Article::getUserId, userId);
        queryWrapper.eq(Article::getIsDeleted, 0);
        queryWrapper.orderByDesc(Article::getUpdateTime);

        return baseMapper.selectList(queryWrapper);
    }
}




