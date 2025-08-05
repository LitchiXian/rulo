package com.l2.web;

import cn.dev33.satoken.stp.StpUtil;
import com.baomidou.mybatisplus.core.conditions.query.LambdaQueryWrapper;
import com.l2.domain.Article;
import com.l2.common.domain.dto.IdDto;
import com.l2.domain.dto.SaveArticleDto;
import com.l2.common.exception.ServiceException;
import com.l2.service.ArticleService;
import com.l2.common.domain.AjaxResult;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping("/blog/article")
@RequiredArgsConstructor
public class ArticleController {

    private final ArticleService articleService;

    @PostMapping("/save")
    public AjaxResult save(@RequestBody SaveArticleDto dto) {
        articleService.save(dto);
        return AjaxResult.success();
    }

    @PostMapping("/remove")
    public AjaxResult remove(@RequestBody IdDto idDto) {
        articleService.removeById(idDto.getId());
        return AjaxResult.success();
    }

    @PostMapping("/update")
    public AjaxResult update(@RequestBody Article article) {
        articleService.updateById(article);
        return AjaxResult.success();
    }

    @GetMapping("/list")
    public AjaxResult list() {
        System.out.println("是否登录：" + StpUtil.isLogin());
        if (StpUtil.isLogin()) {
            System.out.println("当前登录用户：" + StpUtil.getLoginId());
            System.out.println("当前登录用户：" + StpUtil.getTokenInfo());
        }
        return AjaxResult.success(articleService.list(
                        new LambdaQueryWrapper<Article>()
                                .eq(Article::getIsDeleted, 0)
                                .eq(Article::getIsPublished, 1)
                                .orderByDesc(Article::getUpdateTime)
                )
        );
    }

    @GetMapping("/get")
    public AjaxResult get(IdDto idDto) {
        Article article = articleService.getById(idDto.getId());
        return AjaxResult.success(article);
    }

    @GetMapping("/list2")
    public AjaxResult list2() {
//        int i = 1/0;
        throw new ServiceException("10086", "自定义异常");
//        return AjaxResult.success(bBlogArticleService.listBlogArticle());
    }

    @GetMapping("/getUserArticleList")
    public AjaxResult getUserArticleList(IdDto idDto) {
        List<Article> userArticleList = articleService.getUserArticleList(idDto.getId());
        return AjaxResult.success(userArticleList);
    }
}
