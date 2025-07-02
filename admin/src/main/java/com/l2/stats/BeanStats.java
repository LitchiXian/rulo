package com.l2.stats;

import java.util.LinkedHashMap;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

public class BeanStats {
    private static final Map<String, Long> beanInitTimes = new ConcurrentHashMap<>();

    /**
     * 记录初始化耗时
     *
     * @param beanName
     * @param costTime
     */
    public static void record(String beanName, long costTime) {
        beanInitTimes.put(beanName, costTime);
        System.out.printf("[Bean监控] %-40s 初始化耗时: %d ms%n", beanName, costTime);
    }

    public static Map<String, Long> getTopSlowBeans(int topNum) {
        return beanInitTimes.entrySet().stream()
                .sorted(Map.Entry.<String, Long>comparingByValue().reversed())
                .limit(topNum)
                .collect(LinkedHashMap::new,
                        (m, v) -> m.put(v.getKey(), v.getValue()), Map::putAll);
    }

}
