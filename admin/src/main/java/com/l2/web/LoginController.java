package com.l2.web;

import com.l2.domain.dto.UserDto;
import com.l2.service.SysUserService;
import com.l2.util.AjaxResult;
import com.l2.util.SaTokenUtil;
import lombok.RequiredArgsConstructor;
import org.springframework.web.bind.annotation.*;

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
    public AjaxResult login(@RequestBody UserDto userDto) {
        String token = sysUserService.login(userDto);
        return AjaxResult.success(token);
    }

    @PostMapping("/register")
    public AjaxResult register(@RequestBody UserDto userDto) {
        int i = sysUserService.register(userDto);
        return AjaxResult.success();
    }

    @GetMapping("/logout")
    public AjaxResult logout() {
        int i = sysUserService.logout();
        return AjaxResult.success();
    }

    @GetMapping("/getRegisterCode")
    public AjaxResult getRegisterCode(@ModelAttribute UserDto userDto) {
        sysUserService.getRegisterCode(userDto);
        return AjaxResult.success();
    }

    @GetMapping("/getLoginInfo")
    public AjaxResult getLoginInfo() {
        return AjaxResult.success(sysUserService.getById(SaTokenUtil.getLoginId()));
    }
}
