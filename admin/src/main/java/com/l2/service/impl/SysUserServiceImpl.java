package com.l2.service.impl;

import cn.dev33.satoken.stp.StpUtil;
import cn.hutool.core.collection.CollectionUtil;
import com.baomidou.mybatisplus.core.conditions.query.LambdaQueryWrapper;
import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.l2.config.JsonRedisTemplate;
import com.l2.config.SnowflakeConfig;
import com.l2.constant.RedisConstant;
import com.l2.domain.SysUser;
import com.l2.domain.dto.UserDto;
import com.l2.exception.ServiceException;
import com.l2.mapper.SysUserMapper;
import com.l2.service.SysUserService;
import com.l2.util.MailUtil;
import lombok.RequiredArgsConstructor;
import org.apache.commons.lang3.StringUtils;
import org.mindrot.jbcrypt.BCrypt;
import org.springframework.stereotype.Service;

import java.util.Collections;
import java.util.concurrent.ThreadLocalRandom;
import java.util.concurrent.TimeUnit;

/**
 * @author Administrator
 * @description 针对表【sys_user(用户信息表)】的数据库操作Service实现
 * @createDate 2025-07-17 17:38:31
 */
@Service
@RequiredArgsConstructor
public class SysUserServiceImpl extends ServiceImpl<SysUserMapper, SysUser>
        implements SysUserService {

    private final SnowflakeConfig snowflakeConfig;

    private final JsonRedisTemplate redisTemplate;

    private final MailUtil mailUtil;

    private static final String EMAIL_REGEX =
            "^[a-zA-Z0-9]+(?:[._+-][a-zA-Z0-9]+)*@[a-zA-Z0-9]+(?:[.-][a-zA-Z0-9]+)*\\.[a-zA-Z]{2,}$";

    @Override
    public String login(UserDto loginUser) {
        if (StringUtils.isEmpty(loginUser.getUserName()) || StringUtils.isEmpty(loginUser.getPassword())) {
            throw new ServiceException("用户名或密码不能为空");
        }
        // 查询用户是否存在且正常
        SysUser user = baseMapper.selectOne(new LambdaQueryWrapper<>(SysUser.class)
                .eq(SysUser::getUserName, loginUser.getUserName())
                .eq(SysUser::getIsDeleted, 0));
        if (user == null) {
            throw new RuntimeException("用户名不存在或未激活");
        }

        // 验证密码（bcrypt 自动比对盐值和计算成本）
        if (!BCrypt.checkpw(loginUser.getPassword(), user.getPassword())) {
            throw new RuntimeException("密码错误");
        }

        StpUtil.login(user.getId());

        String token = StpUtil.getTokenInfo().getTokenValue();
        return token;
    }

    @Override
    public int register(UserDto userDto) {
        if (StringUtils.isEmpty(userDto.getUserName()) || StringUtils.isEmpty(userDto.getPassword())) {
            throw new ServiceException("用户名或密码不能为空");
        }

        if (StringUtils.isEmpty(userDto.getEmail())) {
            throw new ServiceException("邮箱不能为空");
        }

        if (StringUtils.isEmpty(userDto.getCode())) {
            throw new ServiceException("验证码不能为空");
        }


        if (!userDto.getEmail().matches(EMAIL_REGEX)) {
            throw new ServiceException("邮箱格式不正确");
        }

        // 校验用户名是否已存在
        if (CollectionUtil.isNotEmpty(baseMapper.selectList(new LambdaQueryWrapper<>(SysUser.class)
                .eq(SysUser::getUserName, userDto.getUserName())
                .eq(SysUser::getIsDeleted, 0)))) {
            throw new ServiceException("用户名已存在");
        }

        // 校验密码长度（基础安全策略）
        if (userDto.getPassword().length() < 3) {
            throw new ServiceException("密码长度至少3位");
        }

        String code = (String) redisTemplate.opsForValue().get(RedisConstant.REGISTER_CODE + userDto.getEmail());
        if (!userDto.getCode().equals(code)) {
            throw new ServiceException("验证码错误");
        }

        // 生成 bcrypt 哈希（自动包含盐值，cost=12）
        String passwordHash = BCrypt.hashpw(userDto.getPassword(), BCrypt.gensalt(12));

        // 创建用户并保存
        SysUser user = new SysUser();
        long id = snowflakeConfig.snowflakeId();
        user.setId(id);
        user.setUserName(userDto.getUserName());
        user.setNickName(userDto.getUserName());
        user.setPassword(passwordHash);
        user.setEmail(userDto.getEmail());
        user.setRemark("");
        user.setCreateId(id);
        user.setUpdateId(id);

        int insert = baseMapper.insert(user);
        if (insert <= 0) {
            throw new ServiceException("注册失败");
        }
        return insert;
    }

    @Override
    public int logout() {
        StpUtil.logout();
        return 1;
    }

    @Override
    public int getRegisterCode(UserDto userDto) {
        if (StringUtils.isEmpty(userDto.getEmail())) {
            throw new ServiceException("邮箱不能为空");
        }

        if (!userDto.getEmail().matches(EMAIL_REGEX)) {
            throw new ServiceException("邮箱格式不正确");
        }

        long id = snowflakeConfig.snowflakeId();
        long salt = ThreadLocalRandom.current().nextLong(1000);
        String code = String.format("%06d", Math.abs(id ^ salt) % 1_000_000);

        boolean successFlag = mailUtil.sendTextEmail(
                Collections.singleton(userDto.getEmail()), "L2 注册验证码", "您的注册验证码为：" + code);

        if (!successFlag) {
            throw new ServiceException("发送验证码失败，请稍后重试");
        }

        redisTemplate.opsForValue().set(RedisConstant.REGISTER_CODE + userDto.getEmail(), code, 5, TimeUnit.MINUTES);

        return 1;
    }
}




