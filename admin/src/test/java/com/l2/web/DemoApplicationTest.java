package com.l2.web;

import org.junit.jupiter.api.Test;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.data.redis.core.StringRedisTemplate;

import java.time.Duration;

/**
 * @Author: Linzx
 * @Date: 2025/7/21 12:41
 * @Desc: DemoApplicationTest
 */
@SpringBootTest(webEnvironment = SpringBootTest.WebEnvironment.RANDOM_PORT)
public class DemoApplicationTest {

    static final Logger logger = LoggerFactory.getLogger(DemoApplicationTest.class);

    // 注入 StringRedisTemplate
    @Autowired
    StringRedisTemplate stringRedisTemplate;

    @Test
    public void test() {

        // 设置
        this.stringRedisTemplate.opsForValue().set("title", "spring 中文网", Duration.ofMinutes(5));

        // 读取
        String val = this.stringRedisTemplate.opsForValue().get("title");

        logger.info("value={}", val);
    }
}
