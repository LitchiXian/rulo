package com.l2.exception;

/**
 * @Author Linzx
 * @Date 2025/7/15 12:13
 * @Desc ServiceException
 */
public class ServiceException extends RuntimeException{
    private static final long serialVersionUID = 1L;

    /**
     * 错误码
     */
    private String code;

    /**
     * 错误信息
     */
    private String message;

    public ServiceException(){}

    public ServiceException(String code){
        this.code = code;
    }

    public ServiceException(String code, String message){
        this.code = code;
        this.message = message;
    }

    public String getCode() {
        return code;
    }

    @Override
    public String getMessage() {
        return message;
    }
}
