package com.l2.util;

import java.util.Collections;
import java.util.HashMap;
import java.util.Map;

public class AjaxResult {
    // 使用枚举定义状态类型
    public enum Status {
        SUCCESS("200"),
        ERROR("500");

        private final String code;

        Status(String code) {
            this.code = code;
        }

        public String getCode() {
            return code;
        }
    }

    // 使用静态常量替代字符串硬编码
    public static final String CODE_KEY = "code";
    public static final String MSG_KEY = "msg";
    public static final String DATA_KEY = "data";

    // 使用不可变Map封装结果
    private final Map<String, Object> data;

    /**
     * 构造结果对象
     *
     * @param status  状态枚举
     * @param message 提示消息
     * @param data    业务数据
     */
    private AjaxResult(Status status, String message, Object data) {
        Map<String, Object> map = new HashMap<>(3);
        map.put(CODE_KEY, status.getCode());
        if (message != null) {
            map.put(MSG_KEY, message);
        }
        if (data != null) {
            map.put(DATA_KEY, data);
        }
        this.data = Collections.unmodifiableMap(map);
    }

    /**
     * 获取结果映射
     */
    public Map<String, Object> getData() {
        return data;
    }

    /**
     * 获取状态码
     */
    public String getCode() {
        return (String) data.get(CODE_KEY);
    }

    /**
     * 判断是否成功
     */
    public boolean isSuccess() {
        return getCode().equals(Status.SUCCESS.getCode());
    }

    // ================= 工厂方法 =================

    /**
     * 成功响应（无消息无数据）
     */
    public static AjaxResult success() {
        return new AjaxResult(Status.SUCCESS, null, null);
    }

    /**
     * 成功响应（无消息有数据）
     *
     * @param data 需要返回的数据
     */
    public static AjaxResult success(Object data) {
        return new AjaxResult(Status.SUCCESS, null, data);
    }

    /**
     * 成功响应（有消息有数据）
     *
     * @param message 提示消息
     * @param data    需要返回的数据
     */
    public static AjaxResult success(String message, Object data) {
        return new AjaxResult(Status.SUCCESS, message, data);
    }

    /**
     * 失败响应（默认消息）
     */
    public static AjaxResult error() {
        return error(Status.ERROR.getCode(), "操作失败");
    }

    /**
     * 失败响应（自定义消息）
     *
     * @param message 错误提示
     */
    public static AjaxResult error(String message) {
        return error(Status.ERROR.getCode(), message);
    }

    /**
     * 失败响应（自定义错误码+消息）
     *
     * @param code    自定义错误码
     * @param message 错误提示
     */
    public static AjaxResult error(String code, String message) {
        Map<String, Object> map = new HashMap<>(3);
        map.put(CODE_KEY, code);
        map.put(MSG_KEY, message);
        return new AjaxResult(Status.SUCCESS, message, null) {
            @Override
            public Map<String, Object> getData() {
                return Collections.unmodifiableMap(map);
            }
        };
    }
}