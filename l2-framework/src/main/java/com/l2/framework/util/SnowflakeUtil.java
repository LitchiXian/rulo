package com.l2.framework.util;

import cn.hutool.core.lang.Snowflake;
import cn.hutool.core.net.NetUtil;
import cn.hutool.core.util.IdUtil;
import lombok.extern.slf4j.Slf4j;
import org.springframework.stereotype.Component;


/**
 * 雪花算法生成id
 */
@Slf4j
@Component
public class SnowflakeUtil {

    // 数据中心标志
    private final long datacenterId = 1L;

    // 单体服务，所以没有必要从配置文件中获取，如果集群，则从配置文件中获取，每个服务器只能有一个workerId
    // 机器标志
//    @Value("${server.worker-id}")
    private final long workerId = 1L;

    private final Snowflake snowflake = IdUtil.getSnowflake(workerId, datacenterId);

    public SnowflakeUtil() {
        String ipv4 = NetUtil.getLocalhostStr();
        long workerIda = NetUtil.ipv4ToLong(ipv4);
        log.info("当前机器的workId:{},ipv4:{},server.worker-id:{}", workerIda, ipv4, workerId);
    }

    public long snowflakeId() {
        return snowflake.nextId();
    }

}