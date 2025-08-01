package com.l2.stats;

import org.springframework.scheduling.annotation.Scheduled;
import org.springframework.stereotype.Component;

import java.text.DecimalFormat;
import java.util.Comparator;
import java.util.List;
import java.util.stream.Collectors;

@Component
public class ApiStatsReporter {

    private static final DecimalFormat AVG_FORMAT = new DecimalFormat("#.##");

    @Scheduled(fixedRate = 1 * 60 * 60 * 1000)
    public void generateReport() {
        System.out.println("\n[API性能日报]");
        System.out.println("========================================================================================");
        System.out.printf("%-50s %-10s %-10s %-10s%n", "API", "平均(ms)", "P99(ms)", "调用次数");

        // 获取所有方法并按照调用次数排序
        List<String> methods = ApiStats.getTrackedMethods().stream()
                .sorted(Comparator.comparingInt(ApiStats::getCallCount).reversed())
                .collect(Collectors.toList());

        for (String method : methods) {
            String apiPath = ApiStats.getApiPath(method);
            double avg = ApiStats.getAvg(method);
            long p99 = ApiStats.getP99(method);
            int count = ApiStats.getCallCount(method);

            // 高亮标出需要优化的API
            String highlight = (p99 > 300 || avg > 100) ? "<- 需要优化!" : "";

            System.out.printf("%-50s %-10s %-10d %-10d %s%n",
                    trimApiPath(apiPath),
                    AVG_FORMAT.format(avg),
                    p99,
                    count,
                    highlight);
        }
        System.out.println("========================================================================================\n");
    }

    // 简化和美化API路径显示
    private String trimApiPath(String path) {
        if (path.length() <= 20) return path;
        return path.substring(0, 17) + "...";
    }

}
