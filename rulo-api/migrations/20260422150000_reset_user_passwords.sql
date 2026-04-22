-- 重置 5 个用户密码为 用户名+123.Aa
-- sadmin   -> sadmin123.Aa
-- admin    -> admin123.Aa
-- common01 -> common01123.Aa
-- test01   -> test01123.Aa
-- test02   -> test02123.Aa

UPDATE sys_user SET password = '$argon2id$v=19$m=19456,t=2,p=1$AuGjcjSi09i1OqzVsK7CmQ$yLiMSTtaZbwdOjjWK7pS3jrsxgKdURxG/0qcY6oeHi8', update_time = now(), update_id = 1 WHERE id = 1001;
UPDATE sys_user SET password = '$argon2id$v=19$m=19456,t=2,p=1$0L7ErQxkon7DyRTaFLoVpA$v7R7CT+BUu/TdYtmIRGeXjVnT2Nl8N7fsTKnPkhVL88', update_time = now(), update_id = 1 WHERE id = 1002;
UPDATE sys_user SET password = '$argon2id$v=19$m=19456,t=2,p=1$RK2EICs7uYwBmn7bvVnqAQ$kzEOP56Tv23XftwpkHtN9EFcc7UhsGcMrl1bJfKzpEk', update_time = now(), update_id = 1 WHERE id = 1003;
UPDATE sys_user SET password = '$argon2id$v=19$m=19456,t=2,p=1$Gh2II12kx4Aq7rs+N4fKsA$N99o+YSzKhh6oVnONC1AQgbFhiyDwXciIehs00poIwk', update_time = now(), update_id = 1 WHERE id = 1004;
UPDATE sys_user SET password = '$argon2id$v=19$m=19456,t=2,p=1$4hNkL2hj+7oUhSjtMBGGWQ$iM76O7w5KS0E9jlTxGYF703cWKLSwZ+KWfyaDmKA8bM', update_time = now(), update_id = 1 WHERE id = 1005;
