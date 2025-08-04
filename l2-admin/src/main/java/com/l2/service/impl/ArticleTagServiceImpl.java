package com.l2.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.l2.domain.ArticleTag;
import com.l2.service.ArticleTagService;
import com.l2.mapper.ArticleTagMapper;
import org.springframework.stereotype.Service;

/**
* @author Administrator
* @description 针对表【b_article_tag(文章与标签关联关系表)】的数据库操作Service实现
* @createDate 2025-08-04 12:53:13
*/
@Service
public class ArticleTagServiceImpl extends ServiceImpl<ArticleTagMapper, ArticleTag>
    implements ArticleTagService{

}




