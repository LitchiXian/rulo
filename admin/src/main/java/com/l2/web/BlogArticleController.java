package com.l2.web;

import com.baomidou.mybatisplus.core.conditions.query.LambdaQueryWrapper;
import com.l2.domain.BlogArticle;
import com.l2.domain.dto.IdDto;
import com.l2.domain.dto.SaveBlogArticleDto;
import com.l2.exception.ServiceException;
import com.l2.service.BlogArticleService;
import com.l2.util.AjaxResult;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/blog/article")
@RequiredArgsConstructor
public class BlogArticleController {

    private final BlogArticleService blogArticleService;

    @PostMapping("/save")
    public AjaxResult save(@RequestBody SaveBlogArticleDto dto) {
        blogArticleService.save(dto);
        return AjaxResult.success();
    }

    @PostMapping("/remove")
    public AjaxResult remove(@RequestBody IdDto idDto) {
        blogArticleService.removeById(idDto.getId());
        return AjaxResult.success();
    }

    @PostMapping("/update")
    public AjaxResult update(@RequestBody BlogArticle blogArticle) {
        blogArticleService.updateById(blogArticle);
        return AjaxResult.success();
    }

    @GetMapping("/list")
    public AjaxResult list() {
        return AjaxResult.success(blogArticleService.list(new LambdaQueryWrapper<BlogArticle>().orderByDesc(BlogArticle::getUpdateTime)));
    }

    @GetMapping("/get")
    public AjaxResult get(IdDto idDto) {
        return AjaxResult.success(blogArticleService.getById(idDto.getId()));
    }

    @GetMapping("/list2")
    public AjaxResult list2() {
//        int i = 1/0;
        throw new ServiceException(10086,"自定义异常");
//        return AjaxResult.success(bBlogArticleService.listBlogArticle());
    }
}
