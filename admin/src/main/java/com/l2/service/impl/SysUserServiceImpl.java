package com.l2.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.l2.domain.SysUser;
import com.l2.domain.dto.UserDto;
import com.l2.service.SysUserService;
import com.l2.mapper.SysUserMapper;
import org.springframework.stereotype.Service;

/**
* @author Administrator
* @description 针对表【sys_user(用户信息表)】的数据库操作Service实现
* @createDate 2025-07-17 17:38:31
*/
@Service
public class SysUserServiceImpl extends ServiceImpl<SysUserMapper, SysUser>
    implements SysUserService{

    @Override
    public int login(UserDto loginUser) {
//        // 校验用户名是否已存在
//        if (baseMapper.findByUsername(loginUser.getUsername()) != null) {
//            throw new RuntimeException("用户名已存在");
//        }
//
//        // 校验密码长度（基础安全策略）
//        if (plainPassword.length() < 8) {
//            throw new RuntimeException("密码长度至少8位");
//        }
//
//        // 生成 bcrypt 哈希（自动包含盐值，cost=12）
//        String passwordHash = BCrypt.hashpw(plainPassword, BCrypt.gensalt(12));
//
//        // 创建用户并保存
//        User user = User.builder()
//                .username(username)
//                .passwordHash(passwordHash)
//                .createTime(LocalDateTime.now())
//                .build();
//        return userRepository.save(user);
        return 0;
    }
}




