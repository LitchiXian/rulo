package com.l2.monitor;

import com.baomidou.mybatisplus.core.mapper.Mapper;
import com.l2.stats.BeanStats;
import org.springframework.beans.factory.config.BeanPostProcessor;

import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

/**
 * 记录Bean初始化耗时
 */
public class BeanMonitorProcessor implements BeanPostProcessor {
    // 记录初始化开始时间
    private final ConcurrentMap<String, Long> startTime = new ConcurrentHashMap<>();

    @Override
    public Object postProcessBeforeInitialization(Object bean, String beanName) {
        startTime.put(beanName, System.currentTimeMillis());
        return bean;
    }

    @Override
    public Object postProcessAfterInitialization(Object bean, String beanName) {
        // 跳过MyBatis Mapper的监控
        if(beanName.contains("Mapper") || bean instanceof Mapper) {
            return bean;
        }
        // 计算耗时
        long cost = System.currentTimeMillis() - startTime.get(beanName);
        // 记录到统计器
        BeanStats.record(beanName, cost);
        // 清除ThreadLocal，避免内存泄漏
        startTime.remove(beanName);
        return bean;
    }
}
