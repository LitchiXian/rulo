package com.l2.framework.config;

import com.l2.framework.monitor.BeanMonitorProcessor;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.context.annotation.EnableAspectJAutoProxy;
import org.springframework.scheduling.annotation.EnableScheduling;

@Configuration
@EnableAspectJAutoProxy
@EnableScheduling
public class MonitorConfig {
    @Bean
    public BeanMonitorProcessor beanMonitorProcessor() {
        return new BeanMonitorProcessor();
    }
}
