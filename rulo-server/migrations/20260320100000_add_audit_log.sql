-- 审计日志表
CREATE TABLE IF NOT EXISTS sys_audit_log (
    id            BIGINT PRIMARY KEY,
    user_id       BIGINT,
    user_name     VARCHAR(30),
    method        VARCHAR(10)   NOT NULL,
    path          VARCHAR(500)  NOT NULL,
    params        TEXT,
    status        SMALLINT      NOT NULL,
    duration_ms   BIGINT        NOT NULL,
    ip            VARCHAR(45),
    is_sensitive   BOOLEAN       NOT NULL DEFAULT FALSE,
    created_time  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

COMMENT ON TABLE  sys_audit_log              IS '系统_审计日志表';
COMMENT ON COLUMN sys_audit_log.id           IS 'ID（雪花ID）';
COMMENT ON COLUMN sys_audit_log.user_id      IS '操作用户ID';
COMMENT ON COLUMN sys_audit_log.user_name    IS '操作用户名';
COMMENT ON COLUMN sys_audit_log.method       IS 'HTTP 方法（GET/POST/PUT/DELETE）';
COMMENT ON COLUMN sys_audit_log.path         IS '请求路径';
COMMENT ON COLUMN sys_audit_log.params       IS '请求参数（POST body，敏感接口不记录）';
COMMENT ON COLUMN sys_audit_log.status       IS 'HTTP 响应状态码';
COMMENT ON COLUMN sys_audit_log.duration_ms  IS '请求耗时（毫秒）';
COMMENT ON COLUMN sys_audit_log.ip           IS '客户端IP';
COMMENT ON COLUMN sys_audit_log.is_sensitive IS '是否敏感操作';
COMMENT ON COLUMN sys_audit_log.created_time IS '创建时间';

-- 索引
CREATE INDEX IF NOT EXISTS idx_audit_user_id      ON sys_audit_log(user_id);
CREATE INDEX IF NOT EXISTS idx_audit_created_time  ON sys_audit_log(created_time DESC);
CREATE INDEX IF NOT EXISTS idx_audit_path          ON sys_audit_log(path);
CREATE INDEX IF NOT EXISTS idx_audit_is_sensitive  ON sys_audit_log(is_sensitive);
