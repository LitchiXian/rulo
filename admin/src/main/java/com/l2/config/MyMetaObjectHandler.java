package com.l2.config;

import com.baomidou.mybatisplus.core.handlers.MetaObjectHandler;
import lombok.RequiredArgsConstructor;
import org.apache.ibatis.reflection.MetaObject;
import org.springframework.stereotype.Component;

import java.util.Date;

@Component
@RequiredArgsConstructor
public class MyMetaObjectHandler implements MetaObjectHandler {

    private final SnowflakeConfig snowflakeConfig;

    private static final Integer UN_DELETED_FLAG = 0;

    /**
     * 获取当前操作用户ID（需要根据实际项目实现）
     */
    private Long getCurrentUserId() {
        // 实际项目中：从线程安全的方式获取当前登录用户ID
        // 例如：SecurityContextHolder.getContext().getAuthentication().getName()
        return snowflakeConfig.snowflakeId();
    }

    @Override
    public void insertFill(MetaObject metaObject) {
        Long userId = this.getCurrentUserId();
        Date now = new Date();

        // 插入操作时自动填充
        this.strictInsertFill(metaObject, "id", Long.class, snowflakeConfig.snowflakeId());
        this.strictInsertFill(metaObject, "isDeleted", Integer.class, UN_DELETED_FLAG);
        this.strictInsertFill(metaObject, "isDeleted2", Integer.class, UN_DELETED_FLAG);
        this.strictInsertFill(metaObject, "createId", Long.class, userId);
        this.strictInsertFill(metaObject, "createTime", Date.class, now);
        this.strictInsertFill(metaObject, "updateId", Long.class, userId);
        this.strictInsertFill(metaObject, "updateTime", Date.class, now);
    }

    @Override
    public void updateFill(MetaObject metaObject) {
        // 更新操作时自动填充
        this.strictUpdateFill(metaObject, "updateId", Long.class, this.getCurrentUserId());
        this.strictUpdateFill(metaObject, "updateTime", Date.class, new Date());
    }
}
