package com.l2.util;

import lombok.Data;

@Data
public class AjaxResult<T> {
    private Integer code = 200;
    private String msg;
    private T data;

    public AjaxResult() {
    }

    public AjaxResult(T data) {
        this.code = 200;
        this.data = data;
    }

    public AjaxResult(Integer code, String msg) {
        this.code = code;
        this.msg = msg;
    }

    public static AjaxResult error() {
        return new AjaxResult<>(500, "操作失败");
    }

    public static AjaxResult success() {
        return new AjaxResult<>();
    }

    public static <T> AjaxResult<T> success(T  data) {
        return new AjaxResult<>(data);
    }

}
