package com.l2.service;

import com.l2.domain.SysUser;
import com.baomidou.mybatisplus.extension.service.IService;
import com.l2.domain.dto.UserDto;

/**
* @author Administrator
* @description 针对表【sys_user(用户信息表)】的数据库操作Service
* @createDate 2025-07-17 17:38:31
*/
public interface SysUserService extends IService<SysUser> {

    String login(UserDto userDto);

    int register(UserDto userDto);
}
