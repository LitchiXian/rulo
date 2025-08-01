package com.l2.framework.config;

import com.baomidou.mybatisplus.core.handlers.MetaObjectHandler;
import com.l2.framework.util.SaTokenUtil;
import com.l2.framework.util.SnowflakeUtil;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.apache.ibatis.reflection.MetaObject;
import org.springframework.stereotype.Component;

import java.util.Date;

@Slf4j
@Component
@RequiredArgsConstructor
public class MyMetaObjectHandler implements MetaObjectHandler {

    private final SnowflakeUtil snowflakeUtil;

    private static final Integer UN_DELETED_FLAG = 0;

    /**
     * 获取当前操作用户ID（需要根据实际项目实现）
     */
    private Long getCurrentUserId() {
        return SaTokenUtil.getLoginId();
    }

    @Override
    public void insertFill(MetaObject metaObject) {
        Long userId = this.getCurrentUserId();
        Date now = new Date();

        // 插入操作时自动填充
        this.strictInsertFill(metaObject, "id", Long.class, snowflakeUtil.snowflakeId());
        this.strictInsertFill(metaObject, "isDeleted", Integer.class, UN_DELETED_FLAG);
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
