-- sys_user 增加 avatar_url 字段（存储对象存储中的文件 key）
ALTER TABLE sys_user ADD COLUMN avatar_url VARCHAR(500) DEFAULT NULL;

COMMENT ON COLUMN sys_user.avatar_url IS '用户头像 URL（对象存储 key）';
