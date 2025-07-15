package com.l2.web;

import com.baomidou.mybatisplus.core.conditions.query.LambdaQueryWrapper;
import com.l2.entity.BBlogArticle;
import com.l2.entity.dto.IdDto;
import com.l2.entity.dto.SaveBBlogArticleDto;
import com.l2.exception.ServiceException;
import com.l2.service.BBlogArticleService;
import com.l2.util.AjaxResult;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/blog/article")
@RequiredArgsConstructor
public class BBlogArticleController {

    private final BBlogArticleService bBlogArticleService;

    @PostMapping("/save")
    public AjaxResult save(@RequestBody SaveBBlogArticleDto dto) {
        bBlogArticleService.save(dto);
        return AjaxResult.success();
    }

    @PostMapping("/remove")
    public AjaxResult remove(@RequestBody IdDto idDto) {
        bBlogArticleService.removeById(idDto.getId());
        return AjaxResult.success();
    }

    @PostMapping("/update")
    public AjaxResult update(@RequestBody BBlogArticle bBlogArticle) {
        bBlogArticleService.updateById(bBlogArticle);
        return AjaxResult.success();
    }

    @GetMapping("/list")
    public AjaxResult list() {
        return AjaxResult.success(bBlogArticleService.list(new LambdaQueryWrapper<BBlogArticle>().orderByDesc(BBlogArticle::getUpdateTime)));
    }

    @GetMapping("/get")
    public AjaxResult get(IdDto idDto) {
        return AjaxResult.success(bBlogArticleService.getById(idDto.getId()));
    }

    @GetMapping("/list2")
    public AjaxResult list2() {
//        int i = 1/0;
        throw new ServiceException(10086,"自定义异常");
//        return AjaxResult.success(bBlogArticleService.listBlogArticle());
    }
}
