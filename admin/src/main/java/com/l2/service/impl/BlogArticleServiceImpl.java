package com.l2.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.l2.domain.BlogArticle;
import com.l2.domain.dto.SaveBlogArticleDto;
import com.l2.exception.ErrorCodeEnum;
import com.l2.exception.ServiceException;
import com.l2.mapper.BlogArticleMapper;
import com.l2.service.BlogArticleService;
import com.l2.util.SaTokenUtil;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Service;

import java.util.Date;
import java.util.List;

/**
 * @author Administrator
 * @description 针对表【b_blog_article(博客文章核心表)】的数据库操作Service实现
 * @createDate 2025-07-10 15:08:54
 */
@Service
@RequiredArgsConstructor
public class BlogArticleServiceImpl extends ServiceImpl<BlogArticleMapper, BlogArticle>
        implements BlogArticleService {

//    private final BBlogArticleMapper blogArticleMapper;

    @Override
    public int save(SaveBlogArticleDto dto) {
        BlogArticle blogArticle = new BlogArticle();
//        bBlogArticle.setId(snowflakeConfig.snowflakeId());
        blogArticle.setTitle(dto.getTitle());
        blogArticle.setContent(dto.getContent());
        blogArticle.setUserId(1L);
        blogArticle.setUserName(dto.getUserName());
        Date date = new Date();
        blogArticle.setPublishedTime(date);
        blogArticle.setIsPublished(dto.getIsPublished());
//        bBlogArticle.setIsDeleted(0);
//        bBlogArticle.setCreateId(1L);
//        bBlogArticle.setCreateTime(date);
//        bBlogArticle.setUpdateId(1L);
//        bBlogArticle.setUpdateTime(date);

        int insert = baseMapper.insert(blogArticle);
        return insert;
    }

    @Override
    public List<BlogArticle> listBlogArticle() {
        return baseMapper.listBlogArticle();
    }

    @Override
    public BlogArticle getById(Long id) {
        BlogArticle blogArticle = baseMapper.selectById(id);
        // 文章不存在时直接返回null
        if (blogArticle == null) {
            return null;
        }

        // 只处理未发布的文章（isPublished=0表示未发布）
        if (blogArticle.getIsPublished() == 0) {
            // 获取当前登录用户ID（未登录时SaToken会自动抛出异常）
            Long loginId = SaTokenUtil.getLoginId();

            // 权限校验：当前用户必须为文章作者
            if (!loginId.equals(blogArticle.getUserId())) {
                throw new ServiceException(ErrorCodeEnum.UNAUTHORIZED_ACCESS);
            }
        }
        return blogArticle;
    }
}




