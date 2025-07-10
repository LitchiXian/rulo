package com.l2.config;

import cn.hutool.core.lang.Snowflake;
import cn.hutool.core.net.NetUtil;
import cn.hutool.core.util.IdUtil;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Component;


/**
 * 雪花算法生成id
 */
@Component
public class SnowflakeConfig {
    protected final Logger logger = LoggerFactory.getLogger(SnowflakeConfig.class);

    // 数据中心标志
    private long datacenterId = 1;

    // 机器标志
    @Value("${server.worker-id}")
    private long workerId;

    private final Snowflake snowflake = IdUtil.getSnowflake(workerId, datacenterId);

    public SnowflakeConfig() {
        String ipv4 = NetUtil.getLocalhostStr();
        long workerIda = NetUtil.ipv4ToLong(ipv4);
        logger.info("当前机器的workId:{},ipv4:{},server.worker-id:{}", workerIda, ipv4, workerId);
    }

    public long snowflakeId() {
        return snowflake.nextId();
    }

}