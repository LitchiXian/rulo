package com.l2.aspect;

import com.l2.stats.ApiStats;
import org.aspectj.lang.ProceedingJoinPoint;
import org.aspectj.lang.annotation.Around;
import org.aspectj.lang.annotation.Aspect;
import org.springframework.stereotype.Component;

@Aspect
@Component
public class ApiPerformanceAspect {

    /**
     * 监控所有 controller 方法
     * @param pjp
     * @return
     * @throws Throwable
     */
    @Around("execution(* com..web.*.*(..))")
    public Object monitorController(ProceedingJoinPoint pjp) throws Throwable {
        long start = System.currentTimeMillis();

        // 执行原始方法
        Object retVal = pjp.proceed();

        long costTime = System.currentTimeMillis() - start;

        ApiStats.record(getMethodSignature(pjp), costTime);
        return retVal;
    }

    /**
     * 获取简洁的方法签名（去除非必要信息）
     */
    private String getMethodSignature(ProceedingJoinPoint pjp) {
        String signature = pjp.getSignature().toShortString();
        // 简化为: 类名.方法名(参数类型)
        return signature
                .replace("execution(", "")
                .replace(")", "")
                .replace("com.example.", ""); // 简化为相对路径
    }
}
