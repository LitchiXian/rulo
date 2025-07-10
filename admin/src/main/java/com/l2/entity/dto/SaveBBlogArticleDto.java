package com.l2.entity.dto;

import lombok.Data;

@Data
public class SaveBBlogArticleDto {

    /**
     * 文章标题
     */
    private String title;

    /**
     * Markdown内容
     */
    private String content;

    /**
     * 作者名称（冗余存储）
     */
    private String userName;

    /**
     * 是否发布（0=私密，1=公开）
     */
    private Integer isPublished;

}
