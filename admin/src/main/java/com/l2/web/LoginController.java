package com.l2.web;

import com.l2.domain.dto.UserDto;
import com.l2.service.SysUserService;
import com.l2.util.AjaxResult;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

/**
 * @Author: Linzx
 * @Date: 2025/7/17 20:10
 * @Desc: LoginController
 */
@RestController
@RequestMapping("/login")
@RequiredArgsConstructor
public class LoginController {

    private final SysUserService sysUserService;

    @PostMapping("/login")
    public AjaxResult login(UserDto userDto) {
        int i = sysUserService.login(userDto);
        return AjaxResult.success();
    }
}
