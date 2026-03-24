-- 删除原有全字段唯一约束（不感知软删除，会导致删除后无法重建相同 code）
ALTER TABLE sys_permission DROP CONSTRAINT IF EXISTS sys_permission_perm_code_key;

-- 改为局部唯一索引：只约束未删除（is_deleted = false）的记录
-- 软删除后重新创建相同 code 完全无限制
CREATE UNIQUE INDEX uq_perm_code_active
  ON sys_permission (perm_code)
  WHERE is_deleted = false;
