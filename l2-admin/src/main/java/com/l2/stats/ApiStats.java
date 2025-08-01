package com.l2.stats;

import org.apache.commons.collections4.queue.CircularFifoQueue;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Set;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class ApiStats {
    // 存储每个API的调用记录：方法签名 -> 最近1000次调用的耗时队列
    private static final ConcurrentMap<String, CircularFifoQueue<Long>> metrics = new ConcurrentHashMap<>();

    // 存储方法签名到API路径的映射
    private static final ConcurrentMap<String, String> methodToApi = new ConcurrentHashMap<>();

    /**
     * 记录API性能数据
     *
     * @param methodSignature
     * @param costTime
     */
    public static void record(String methodSignature, long costTime) {
        // 确保队列存在（每个方法独立队列）
        metrics.computeIfAbsent(methodSignature, k -> new CircularFifoQueue<>(1000)).add(costTime);
    }

    /**
     * 注册API路径(用于生成更友好的报告)
     *
     * @param methodSignature
     * @param apiPath
     */
    public static void registerApi(String methodSignature, String apiPath) {
        methodToApi.put(methodSignature, apiPath);
    }

    /**
     * 获取API的P99耗时
     *
     * @param methodSignature
     * @return
     */
    public static long getP99(String methodSignature) {
        CircularFifoQueue<Long> queue = metrics.get(methodSignature);
        if (queue == null || queue.isEmpty()) {
            return -1;
        }

        // 复制数据避免并发修改
        List<Long> list = new ArrayList<>(queue);
        Collections.sort(list);

        int index = (int) Math.ceil(list.size() * 0.99);
        return list.get(index - 1);
    }

    /**
     * 获取API的平均耗时
     *
     * @param methodSignature
     * @return
     */
    public static double getAvg(String methodSignature) {
        CircularFifoQueue<Long> queue = metrics.get(methodSignature);
        if (queue == null || queue.isEmpty()) {
            return -1;
        }

        return queue.stream().mapToLong(Long::longValue).average().orElse(0);
    }

    /**
     * 获取API的调用次数
     *
     * @param methodSignature
     * @return
     */
    public static int getCallCount(String methodSignature) {
        CircularFifoQueue<Long> queue = metrics.get(methodSignature);

        return (queue != null) ? queue.size() : 0;
    }

    /**
     * 获取所有被监控的API
     * @return
     */
    public static Set<String> getTrackedMethods() {
        return metrics.keySet();
    }

    /**
     * 获取API路径
     * @param methodSignature
     * @return
     */
    public static String getApiPath(String methodSignature){
        return methodToApi.getOrDefault(methodSignature, methodSignature);
    }
}
