package com.l2.service.impl;

import cn.hutool.core.collection.CollUtil;
import cn.hutool.core.util.StrUtil;
import com.baomidou.mybatisplus.core.conditions.query.LambdaQueryWrapper;
import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.l2.domain.Article;
import com.l2.domain.ArticleTag;
import com.l2.domain.Tag;
import com.l2.domain.dto.SaveArticleDto;
import com.l2.common.exception.ErrorCodeEnum;
import com.l2.common.exception.ServiceException;
import com.l2.mapper.ArticleMapper;
import com.l2.mapper.ArticleTagMapper;
import com.l2.service.ArticleService;
import com.l2.framework.util.SaTokenUtil;
import lombok.RequiredArgsConstructor;
import org.jetbrains.annotations.NotNull;
import org.springframework.stereotype.Service;
import org.springframework.util.CollectionUtils;

import java.util.*;
import java.util.stream.Collectors;

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

    private final ArticleTagMapper articleTagMapper;

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
        // 插入标签
        if (dto.getTags() != null && !dto.getTags().trim().isEmpty()) {
            List<ArticleTag> articleTags = Arrays.stream(dto.getTags().split(","))
                    .map(String::trim)
                    .filter(tagId -> !tagId.isEmpty())
                    .map(tagId -> {
                        try {
                            ArticleTag articleTag = new ArticleTag();
                            articleTag.setArticleId(article.getId());
                            articleTag.setTagId(Long.parseLong(tagId));
                            return articleTag;
                        } catch (NumberFormatException e) {
                            throw new ServiceException(ErrorCodeEnum.REQUEST_PARAM_ERROR);
                        }
                    })
                    .collect(Collectors.toList());

            if (!articleTags.isEmpty()) {
                articleTagMapper.insert(articleTags); // 使用MyBatis Plus批量插入
            }
        }
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
            return new Article();
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

        List<ArticleTag> articleTagRelations = articleTagMapper.selectList(
                new LambdaQueryWrapper<ArticleTag>()
                        .eq(ArticleTag::getArticleId, id)  // 直接链式调用简化代码
        );

        article.setTags(CollUtil.isEmpty(articleTagRelations)
                ? ""
                : StrUtil.join(",",
                articleTagRelations.stream().map(ArticleTag::getTagId).collect(Collectors.toList())  // 直接流处理，无需collect到List
        ));

        return article;
    }

    @Override
    public List<Article> list() {
        List<Article> articles = baseMapper.selectList(
                new LambdaQueryWrapper<Article>()
                        .eq(Article::getIsDeleted, 0)
                        .eq(Article::getIsPublished, 1)
                        .orderByDesc(Article::getUpdateTime));

        return getArticleWithTags(articles);
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
        List<Article> articles = baseMapper.selectList(queryWrapper);

        return getArticleWithTags(articles);
    }

    @NotNull
    private List<Article> getArticleWithTags(List<Article> articles) {
        if (articles.isEmpty()) {
            return articles;
        }

        // 提取文章ID集合（使用 Set 去重，虽然这里 ID 本身唯一）
        Set<Long> articleIds = articles.stream()
                .map(Article::getId)
                .collect(Collectors.toSet());

        // 批量查询文章标签关系（仅当有文章时执行）
        List<ArticleTag> articleTags = articleTagMapper.selectList(
                new LambdaQueryWrapper<ArticleTag>()
                        .in(ArticleTag::getArticleId, articleIds)
        );

        // 按文章ID分组标签ID（使用 Stream 分组）
        Map<Long, List<Long>> articleTagMap = articleTags.stream()
                .collect(Collectors.groupingBy(
                        ArticleTag::getArticleId,
                        Collectors.mapping(ArticleTag::getTagId, Collectors.toList())
                ));

        // 绑定标签到文章对象（优化字符串拼接）
        articles.forEach(article -> {
            List<Long> tagIds = articleTagMap.getOrDefault(article.getId(), Collections.emptyList());
            // 清空原有标签（避免重复拼接），再拼接新标签
            article.setTags(StrUtil.join(",", tagIds));
        });

        return articles;
    }
}




