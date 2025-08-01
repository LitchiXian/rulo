package com.l2.util;

import cn.dev33.satoken.exception.NotLoginException;
import cn.dev33.satoken.stp.StpUtil;
import lombok.extern.slf4j.Slf4j;

/**
 * @Author: Linzx
 * @Date: 2025/7/23 17:15
 * @Desc: SaTokenUtil
 */
@Slf4j
public class SaTokenUtil {
    public static Long getLoginId() {
        Long loginId = null;
        try {
            loginId = StpUtil.getLoginIdAsLong();
        } catch (NotLoginException e) {
            log.error("获取当前用户ID失败", e);
        }
        return loginId;
    }
}
