package com.l2.domain.dto;

import lombok.Data;

import java.io.Serializable;

/**
 * @Author: Linzx
 * @Date: 2025/7/17 20:12
 * @Desc: UserDto
 */
@Data
public class UserDto implements Serializable {
    private static final long serialVersionUID = 1L;

    private String userName;

    private String password;

    private String email;

    private String code;
}
