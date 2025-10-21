package com.l2.framework.util;

import cn.dev33.satoken.exception.NotLoginException;
import cn.dev33.satoken.stp.StpUtil;
import cn.hutool.extra.spring.SpringUtil;
import cn.hutool.json.JSONObject;
import cn.hutool.json.JSONUtil;
import com.l2.framework.domain.UserFullInfo;
import lombok.extern.slf4j.Slf4j;
import com.l2.common.constant.RedisConstant;

import java.util.concurrent.TimeUnit;

/**
 * @Author: LitchiXian
 * @Date: 2025/7/23 17:15
 * @Desc: SaTokenUtil
 */
@Slf4j
public class SaTokenUtil {
    /**
     * 获取当前用户ID
     *
     * @return 当前用户ID
     */
    public static Long getLoginId() {
        Long loginId = null;
        try {
            loginId = StpUtil.getLoginIdAsLong();
        } catch (NotLoginException e) {
            log.error("获取当前用户ID失败", e);
        }
        return loginId;
    }

    public static void setUserFullInfo(UserFullInfo userFullInfo) {
        SpringUtil.getBean(RedisCache.class).setCacheObject(RedisConstant.USER_FULL_INFO + getLoginId(), JSONUtil.toJsonStr(userFullInfo), 30, TimeUnit.DAYS);
    }

    /**
     * 获取当前用户完整信息
     * @return 用户完整信息
     */
    public static UserFullInfo getLoginUserFullInfo() {
        return getLoginUserFullInfo(getLoginId());
    }

    /**
     * 获取某个用户完整信息
     * @return 用户完整信息
     */
    public static UserFullInfo getLoginUserFullInfo(long userId) {
        Object userFullInfo = SpringUtil.getBean(RedisCache.class).getCacheObject(RedisConstant.USER_FULL_INFO + userId);
        if (userFullInfo == null) {
            return new UserFullInfo();
        }

        return JSONUtil.toBean(userFullInfo.toString(), UserFullInfo.class);
    }


}
