package com.l2.web;

import com.l2.common.domain.AjaxResult;
import com.l2.common.domain.dto.IdDto;
import com.l2.domain.Tag;
import com.l2.service.TagService;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

/**
 * @Author: Linzx
 * @Date: 2025/8/4 12:56
 * @Desc: TagController
 */
@RestController
@RequestMapping("/blog/tag")
@RequiredArgsConstructor
public class TagController {
    private final TagService tagService;

    @PostMapping("/save")
    public AjaxResult save(@RequestBody Tag tag) {
        tagService.save(tag);
        return AjaxResult.success();
    }

    @PostMapping("/remove")
    public AjaxResult remove(@RequestBody IdDto idDto) {
        tagService.removeById(idDto.getId());
        return AjaxResult.success();
    }

    @PostMapping("/update")
    public AjaxResult update(@RequestBody Tag tag) {
        tagService.updateById(tag);
        return AjaxResult.success();
    }

    @GetMapping("/list")
    public AjaxResult list() {
        return AjaxResult.success(tagService.list());
    }

    @GetMapping("/get")
    public AjaxResult get(IdDto idDto) {
        return AjaxResult.success(tagService.getById(idDto.getId()));
    }

}
