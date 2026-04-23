-- ================================================================
-- RULO 数据库初始化迁移（全量快照）
-- 生成时间: 2026-04-23
-- 包含：表结构 + 索引 + 注释 + 种子数据（不含审计日志数据）
-- ================================================================

--
-- PostgreSQL database dump
--


-- Dumped from database version 17.9 (Debian 17.9-0+deb13u1)
-- Dumped by pg_dump version 17.9 (Debian 17.9-0+deb13u1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: sys_audit_log; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.sys_audit_log (
    id bigint NOT NULL,
    user_id bigint,
    user_name character varying(30),
    method character varying(10) NOT NULL,
    path character varying(500) NOT NULL,
    params text,
    status smallint NOT NULL,
    duration_ms bigint NOT NULL,
    ip character varying(45),
    is_sensitive boolean DEFAULT false NOT NULL,
    created_time timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


--
-- Name: TABLE sys_audit_log; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.sys_audit_log IS '系统_审计日志表';


--
-- Name: COLUMN sys_audit_log.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_audit_log.id IS 'ID（雪花ID）';


--
-- Name: COLUMN sys_audit_log.user_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_audit_log.user_id IS '操作用户ID';


--
-- Name: COLUMN sys_audit_log.user_name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_audit_log.user_name IS '操作用户名';


--
-- Name: COLUMN sys_audit_log.method; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_audit_log.method IS 'HTTP 方法（GET/POST/PUT/DELETE）';


--
-- Name: COLUMN sys_audit_log.path; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_audit_log.path IS '请求路径';


--
-- Name: COLUMN sys_audit_log.params; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_audit_log.params IS '请求参数（POST body，敏感接口不记录）';


--
-- Name: COLUMN sys_audit_log.status; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_audit_log.status IS 'HTTP 响应状态码';


--
-- Name: COLUMN sys_audit_log.duration_ms; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_audit_log.duration_ms IS '请求耗时（毫秒）';


--
-- Name: COLUMN sys_audit_log.ip; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_audit_log.ip IS '客户端IP';


--
-- Name: COLUMN sys_audit_log.is_sensitive; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_audit_log.is_sensitive IS '是否敏感操作';


--
-- Name: COLUMN sys_audit_log.created_time; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_audit_log.created_time IS '创建时间';


--
-- Name: sys_dept; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.sys_dept (
    id bigint NOT NULL,
    parent_id bigint DEFAULT 0 NOT NULL,
    name character varying(50) NOT NULL,
    sort_order integer DEFAULT 0 NOT NULL,
    leader character varying(50),
    phone character varying(30),
    email character varying(50),
    is_active boolean DEFAULT true NOT NULL,
    is_deleted boolean DEFAULT false NOT NULL,
    create_id bigint NOT NULL,
    create_time timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    update_id bigint NOT NULL,
    update_time timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    remark character varying(500)
);


--
-- Name: TABLE sys_dept; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.sys_dept IS '系统_部门表';


--
-- Name: COLUMN sys_dept.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_dept.id IS 'ID';


--
-- Name: COLUMN sys_dept.parent_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_dept.parent_id IS '父部门ID（0=顶级）';


--
-- Name: COLUMN sys_dept.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_dept.name IS '部门名称';


--
-- Name: COLUMN sys_dept.sort_order; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_dept.sort_order IS '排序';


--
-- Name: COLUMN sys_dept.leader; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_dept.leader IS '负责人';


--
-- Name: COLUMN sys_dept.phone; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_dept.phone IS '联系电话';


--
-- Name: COLUMN sys_dept.email; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_dept.email IS '邮箱';


--
-- Name: COLUMN sys_dept.is_active; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_dept.is_active IS '部门状态（true=启用，false=停用）';


--
-- Name: COLUMN sys_dept.is_deleted; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_dept.is_deleted IS '逻辑删除标志（false=正常，true=已删除）';


--
-- Name: COLUMN sys_dept.create_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_dept.create_id IS '创建人ID';


--
-- Name: COLUMN sys_dept.create_time; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_dept.create_time IS '创建时间';


--
-- Name: COLUMN sys_dept.update_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_dept.update_id IS '更新人ID';


--
-- Name: COLUMN sys_dept.update_time; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_dept.update_time IS '更新时间';


--
-- Name: COLUMN sys_dept.remark; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_dept.remark IS '备注';


--
-- Name: sys_menu; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.sys_menu (
    id bigint NOT NULL,
    parent_id bigint DEFAULT 0 NOT NULL,
    perm_id bigint,
    name character varying(50) NOT NULL,
    menu_type smallint DEFAULT 1 NOT NULL,
    path character varying(200),
    component character varying(200),
    icon character varying(100),
    sort_order integer DEFAULT 0 NOT NULL,
    is_hidden boolean DEFAULT false NOT NULL,
    is_deleted boolean DEFAULT false NOT NULL,
    create_id bigint NOT NULL,
    create_time timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    update_id bigint NOT NULL,
    update_time timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    remark character varying(500)
);


--
-- Name: TABLE sys_menu; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.sys_menu IS '系统_菜单表（UI展示层）';


--
-- Name: COLUMN sys_menu.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_menu.id IS 'ID';


--
-- Name: COLUMN sys_menu.parent_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_menu.parent_id IS '父菜单ID，顶级为0';


--
-- Name: COLUMN sys_menu.perm_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_menu.perm_id IS '关联权限ID（目录类可为空）';


--
-- Name: COLUMN sys_menu.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_menu.name IS '菜单名称';


--
-- Name: COLUMN sys_menu.menu_type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_menu.menu_type IS '类型：1=目录 2=菜单 3=按钮';


--
-- Name: COLUMN sys_menu.path; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_menu.path IS '路由路径';


--
-- Name: COLUMN sys_menu.component; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_menu.component IS '前端组件路径';


--
-- Name: COLUMN sys_menu.icon; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_menu.icon IS '图标';


--
-- Name: COLUMN sys_menu.sort_order; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_menu.sort_order IS '显示排序';


--
-- Name: COLUMN sys_menu.is_hidden; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_menu.is_hidden IS '系统默认隐藏（false=显示，true=隐藏），超管可见所有';


--
-- Name: COLUMN sys_menu.is_deleted; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_menu.is_deleted IS '逻辑删除标志（false=正常，true=已删除）';


--
-- Name: COLUMN sys_menu.create_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_menu.create_id IS '创建人ID';


--
-- Name: COLUMN sys_menu.create_time; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_menu.create_time IS '创建时间';


--
-- Name: COLUMN sys_menu.update_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_menu.update_id IS '更新人ID';


--
-- Name: COLUMN sys_menu.update_time; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_menu.update_time IS '更新时间';


--
-- Name: COLUMN sys_menu.remark; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_menu.remark IS '备注';


--
-- Name: sys_permission; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.sys_permission (
    id bigint NOT NULL,
    perm_code character varying(100) NOT NULL,
    perm_name character varying(50) NOT NULL,
    perm_type smallint DEFAULT 1 NOT NULL,
    is_deleted boolean DEFAULT false NOT NULL,
    create_id bigint NOT NULL,
    create_time timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    update_id bigint NOT NULL,
    update_time timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    remark character varying(500)
);


--
-- Name: TABLE sys_permission; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.sys_permission IS '系统_权限表（独立于菜单）';


--
-- Name: COLUMN sys_permission.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_permission.id IS 'ID';


--
-- Name: COLUMN sys_permission.perm_code; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_permission.perm_code IS '权限标识，如 sys:user:list';


--
-- Name: COLUMN sys_permission.perm_name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_permission.perm_name IS '权限名称';


--
-- Name: COLUMN sys_permission.perm_type; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_permission.perm_type IS '类型：1=API权限 2=菜单权限';


--
-- Name: COLUMN sys_permission.is_deleted; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_permission.is_deleted IS '逻辑删除标志（false=正常，true=已删除）';


--
-- Name: COLUMN sys_permission.create_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_permission.create_id IS '创建人ID';


--
-- Name: COLUMN sys_permission.create_time; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_permission.create_time IS '创建时间';


--
-- Name: COLUMN sys_permission.update_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_permission.update_id IS '更新人ID';


--
-- Name: COLUMN sys_permission.update_time; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_permission.update_time IS '更新时间';


--
-- Name: COLUMN sys_permission.remark; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_permission.remark IS '备注';


--
-- Name: sys_role; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.sys_role (
    id bigint NOT NULL,
    role_name character varying(30) NOT NULL,
    role_key character varying(50) NOT NULL,
    is_super boolean DEFAULT false NOT NULL,
    is_active boolean DEFAULT true NOT NULL,
    is_deleted boolean DEFAULT false NOT NULL,
    create_id bigint NOT NULL,
    create_time timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    update_id bigint NOT NULL,
    update_time timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    remark character varying(500)
);


--
-- Name: TABLE sys_role; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.sys_role IS '系统_角色表';


--
-- Name: COLUMN sys_role.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_role.id IS 'ID';


--
-- Name: COLUMN sys_role.role_name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_role.role_name IS '角色名称';


--
-- Name: COLUMN sys_role.role_key; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_role.role_key IS '角色标识（唯一），如 admin / editor';


--
-- Name: COLUMN sys_role.is_super; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_role.is_super IS '超管标志（true=超管，跳过所有权限/隐藏校验）';


--
-- Name: COLUMN sys_role.is_active; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_role.is_active IS '角色状态（false=禁用，true=启用）';


--
-- Name: COLUMN sys_role.is_deleted; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_role.is_deleted IS '逻辑删除标志（false=正常，true=已删除）';


--
-- Name: COLUMN sys_role.create_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_role.create_id IS '创建人ID';


--
-- Name: COLUMN sys_role.create_time; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_role.create_time IS '创建时间';


--
-- Name: COLUMN sys_role.update_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_role.update_id IS '更新人ID';


--
-- Name: COLUMN sys_role.update_time; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_role.update_time IS '更新时间';


--
-- Name: COLUMN sys_role.remark; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_role.remark IS '备注';


--
-- Name: sys_role_menu; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.sys_role_menu (
    role_id bigint NOT NULL,
    menu_id bigint NOT NULL,
    is_hidden boolean DEFAULT false NOT NULL
);


--
-- Name: TABLE sys_role_menu; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.sys_role_menu IS '系统_角色菜单关联表';


--
-- Name: COLUMN sys_role_menu.role_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_role_menu.role_id IS '角色ID';


--
-- Name: COLUMN sys_role_menu.menu_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_role_menu.menu_id IS '菜单ID';


--
-- Name: COLUMN sys_role_menu.is_hidden; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_role_menu.is_hidden IS '角色级别隐藏（true=隐藏），覆盖菜单默认隐藏';


--
-- Name: sys_role_permission; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.sys_role_permission (
    role_id bigint NOT NULL,
    perm_id bigint NOT NULL
);


--
-- Name: TABLE sys_role_permission; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.sys_role_permission IS '系统_角色权限关联表（后端鉴权基于此表）';


--
-- Name: COLUMN sys_role_permission.role_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_role_permission.role_id IS '角色ID';


--
-- Name: COLUMN sys_role_permission.perm_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_role_permission.perm_id IS '权限ID';


--
-- Name: sys_user; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.sys_user (
    id bigint NOT NULL,
    user_name character varying(30) NOT NULL,
    nick_name character varying(30) NOT NULL,
    password character varying(255) NOT NULL,
    email character varying(50),
    is_active boolean DEFAULT false NOT NULL,
    is_deleted boolean DEFAULT false NOT NULL,
    create_id bigint NOT NULL,
    create_time timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    update_id bigint NOT NULL,
    update_time timestamp with time zone NOT NULL,
    remark character varying(500),
    avatar_url character varying(500) DEFAULT NULL::character varying
);


--
-- Name: TABLE sys_user; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.sys_user IS '系统_用户信息表';


--
-- Name: COLUMN sys_user.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_user.id IS 'ID';


--
-- Name: COLUMN sys_user.user_name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_user.user_name IS '用户账号';


--
-- Name: COLUMN sys_user.nick_name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_user.nick_name IS '用户昵称';


--
-- Name: COLUMN sys_user.password; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_user.password IS '密码（argon2id 哈希）';


--
-- Name: COLUMN sys_user.email; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_user.email IS '用户邮箱';


--
-- Name: COLUMN sys_user.is_active; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_user.is_active IS '账户状态（true=正常，false=冻结）';


--
-- Name: COLUMN sys_user.is_deleted; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_user.is_deleted IS '逻辑删除标志（false=正常，true=已删除）';


--
-- Name: COLUMN sys_user.create_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_user.create_id IS '创建人ID';


--
-- Name: COLUMN sys_user.create_time; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_user.create_time IS '创建时间';


--
-- Name: COLUMN sys_user.update_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_user.update_id IS '更新人ID';


--
-- Name: COLUMN sys_user.update_time; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_user.update_time IS '更新时间';


--
-- Name: COLUMN sys_user.remark; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_user.remark IS '备注';


--
-- Name: COLUMN sys_user.avatar_url; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_user.avatar_url IS '用户头像 URL（对象存储 key）';


--
-- Name: sys_user_role; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.sys_user_role (
    user_id bigint NOT NULL,
    role_id bigint NOT NULL
);


--
-- Name: TABLE sys_user_role; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.sys_user_role IS '系统_用户角色关联表';


--
-- Name: COLUMN sys_user_role.user_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_user_role.user_id IS '用户ID';


--
-- Name: COLUMN sys_user_role.role_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_user_role.role_id IS '角色ID';


--
-- Name: sys_audit_log sys_audit_log_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sys_audit_log
    ADD CONSTRAINT sys_audit_log_pkey PRIMARY KEY (id);


--
-- Name: sys_dept sys_dept_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sys_dept
    ADD CONSTRAINT sys_dept_pkey PRIMARY KEY (id);


--
-- Name: sys_menu sys_menu_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sys_menu
    ADD CONSTRAINT sys_menu_pkey PRIMARY KEY (id);


--
-- Name: sys_permission sys_permission_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sys_permission
    ADD CONSTRAINT sys_permission_pkey PRIMARY KEY (id);


--
-- Name: sys_role_menu sys_role_menu_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sys_role_menu
    ADD CONSTRAINT sys_role_menu_pkey PRIMARY KEY (role_id, menu_id);


--
-- Name: sys_role_permission sys_role_permission_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sys_role_permission
    ADD CONSTRAINT sys_role_permission_pkey PRIMARY KEY (role_id, perm_id);


--
-- Name: sys_role sys_role_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sys_role
    ADD CONSTRAINT sys_role_pkey PRIMARY KEY (id);


--
-- Name: sys_role sys_role_role_key_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sys_role
    ADD CONSTRAINT sys_role_role_key_key UNIQUE (role_key);


--
-- Name: sys_user sys_user_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sys_user
    ADD CONSTRAINT sys_user_pkey PRIMARY KEY (id);


--
-- Name: sys_user_role sys_user_role_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sys_user_role
    ADD CONSTRAINT sys_user_role_pkey PRIMARY KEY (user_id, role_id);


--
-- Name: idx_audit_created_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_audit_created_time ON public.sys_audit_log USING btree (created_time DESC);


--
-- Name: idx_audit_is_sensitive; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_audit_is_sensitive ON public.sys_audit_log USING btree (is_sensitive);


--
-- Name: idx_audit_path; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_audit_path ON public.sys_audit_log USING btree (path);


--
-- Name: idx_audit_user_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_audit_user_id ON public.sys_audit_log USING btree (user_id);


--
-- Name: idx_dept_is_deleted; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_dept_is_deleted ON public.sys_dept USING btree (is_deleted);


--
-- Name: idx_dept_parent_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_dept_parent_id ON public.sys_dept USING btree (parent_id);


--
-- Name: idx_user_email; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_user_email ON public.sys_user USING btree (email);


--
-- Name: idx_user_username; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_user_username ON public.sys_user USING btree (user_name);


--
-- Name: uq_perm_code_active; Type: INDEX; Schema: public; Owner: -
--

CREATE UNIQUE INDEX uq_perm_code_active ON public.sys_permission USING btree (perm_code) WHERE (is_deleted = false);


--
-- Name: sys_menu sys_menu_perm_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sys_menu
    ADD CONSTRAINT sys_menu_perm_id_fkey FOREIGN KEY (perm_id) REFERENCES public.sys_permission(id) ON DELETE SET NULL;


--
-- Name: sys_role_menu sys_role_menu_menu_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sys_role_menu
    ADD CONSTRAINT sys_role_menu_menu_id_fkey FOREIGN KEY (menu_id) REFERENCES public.sys_menu(id) ON DELETE CASCADE;


--
-- Name: sys_role_menu sys_role_menu_role_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sys_role_menu
    ADD CONSTRAINT sys_role_menu_role_id_fkey FOREIGN KEY (role_id) REFERENCES public.sys_role(id) ON DELETE CASCADE;


--
-- Name: sys_role_permission sys_role_permission_perm_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sys_role_permission
    ADD CONSTRAINT sys_role_permission_perm_id_fkey FOREIGN KEY (perm_id) REFERENCES public.sys_permission(id) ON DELETE CASCADE;


--
-- Name: sys_role_permission sys_role_permission_role_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sys_role_permission
    ADD CONSTRAINT sys_role_permission_role_id_fkey FOREIGN KEY (role_id) REFERENCES public.sys_role(id) ON DELETE CASCADE;


--
-- Name: sys_user_role sys_user_role_role_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sys_user_role
    ADD CONSTRAINT sys_user_role_role_id_fkey FOREIGN KEY (role_id) REFERENCES public.sys_role(id) ON DELETE CASCADE;


--
-- Name: sys_user_role sys_user_role_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.sys_user_role
    ADD CONSTRAINT sys_user_role_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.sys_user(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--



-- ======================== SEED DATA ========================

--
-- PostgreSQL database dump
--


-- Dumped from database version 17.9 (Debian 17.9-0+deb13u1)
-- Dumped by pg_dump version 17.9 (Debian 17.9-0+deb13u1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Data for Name: sys_dept; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public.sys_dept (id, parent_id, name, sort_order, leader, phone, email, is_active, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (100, 0, '如若科技', 1, '李先生', '13800000000', 'admin@ruruo.com', true, false, 1, '2026-04-22 10:45:50.895634+08', 1, '2026-04-22 10:45:50.895634+08', '总公司');
INSERT INTO public.sys_dept (id, parent_id, name, sort_order, leader, phone, email, is_active, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (101, 100, '研发部', 10, '张三', '13800000001', 'dev@ruruo.com', true, false, 1, '2026-04-22 10:45:50.895634+08', 1, '2026-04-22 10:45:50.895634+08', NULL);
INSERT INTO public.sys_dept (id, parent_id, name, sort_order, leader, phone, email, is_active, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (102, 100, '产品部', 20, '李四', '13800000002', 'pd@ruruo.com', true, false, 1, '2026-04-22 10:45:50.895634+08', 1, '2026-04-22 10:45:50.895634+08', NULL);
INSERT INTO public.sys_dept (id, parent_id, name, sort_order, leader, phone, email, is_active, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (103, 100, '运营部', 30, '王五', '13800000003', 'op@ruruo.com', true, false, 1, '2026-04-22 10:45:50.895634+08', 1, '2026-04-22 10:45:50.895634+08', NULL);
INSERT INTO public.sys_dept (id, parent_id, name, sort_order, leader, phone, email, is_active, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (104, 100, '人力资源部', 40, '赵六', '13800000004', 'hr@ruruo.com', true, false, 1, '2026-04-22 10:45:50.895634+08', 1, '2026-04-22 10:45:50.895634+08', NULL);
INSERT INTO public.sys_dept (id, parent_id, name, sort_order, leader, phone, email, is_active, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (10101, 101, '前端组', 10, '钱七', '13800000011', NULL, true, false, 1, '2026-04-22 10:45:50.895634+08', 1, '2026-04-22 10:45:50.895634+08', NULL);
INSERT INTO public.sys_dept (id, parent_id, name, sort_order, leader, phone, email, is_active, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (10102, 101, '后端组', 20, '孙八', '13800000012', NULL, true, false, 1, '2026-04-22 10:45:50.895634+08', 1, '2026-04-22 10:45:50.895634+08', NULL);
INSERT INTO public.sys_dept (id, parent_id, name, sort_order, leader, phone, email, is_active, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (10103, 101, '测试组', 30, '周九', '13800000013', NULL, true, false, 1, '2026-04-22 10:45:50.895634+08', 1, '2026-04-22 10:45:50.895634+08', NULL);


--
-- Data for Name: sys_permission; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (1, 'sys:user:list', '用户管理-列表', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '用户列表页面菜单权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (6, 'sys:role:list', '角色管理-列表', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '角色列表页面菜单权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (11, 'sys:menu:list', '菜单管理-列表', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '菜单列表页面菜单权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (16, 'sys:permission:list', '权限管理-列表', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '权限列表页面菜单权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (2, 'sys:user:save', '用户管理-新增', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '新增用户按钮权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (3, 'sys:user:remove', '用户管理-删除', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '删除用户按钮权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (4, 'sys:user:update', '用户管理-编辑', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '编辑用户按钮权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (5, 'sys:user:detail', '用户管理-详情', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '用户详情查看权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (7, 'sys:role:save', '角色管理-新增', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '新增角色按钮权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (8, 'sys:role:remove', '角色管理-删除', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '删除角色按钮权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (9, 'sys:role:update', '角色管理-编辑', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '编辑角色按钮权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (10, 'sys:role:detail', '角色管理-详情', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '角色详情查看权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (12, 'sys:menu:save', '菜单管理-新增', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '新增菜单按钮权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (13, 'sys:menu:remove', '菜单管理-删除', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '删除菜单按钮权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (14, 'sys:menu:update', '菜单管理-编辑', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '编辑菜单按钮权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (15, 'sys:menu:detail', '菜单管理-详情', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '菜单详情查看权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (17, 'sys:permission:save', '权限管理-新增', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '新增权限按钮权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (18, 'sys:permission:remove', '权限管理-删除', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '删除权限按钮权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (19, 'sys:permission:update', '权限管理-编辑', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '编辑权限按钮权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (20, 'sys:permission:detail', '权限管理-详情', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '权限详情查看权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (22, 'sys:auth:info', '个人中心-查看', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '当前用户信息API权限，所有登录用户均有');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (23, 'sys:auth:logout', '退出登录', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '退出登录API权限，所有登录用户均有');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (24, 'sys:user:menu', '用户管理-页面入口', 2, false, 1770261629457, '2026-03-18 15:00:59.093681+08', 1770261629457, '2026-03-18 15:00:59.093681+08', '用户管理侧边栏菜单是否显示');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (25, 'sys:role:menu', '角色管理-页面入口', 2, false, 1770261629457, '2026-03-18 15:00:59.093681+08', 1770261629457, '2026-03-18 15:00:59.093681+08', '角色管理侧边栏菜单是否显示');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (26, 'sys:menu:menu', '菜单管理-页面入口', 2, false, 1770261629457, '2026-03-18 15:00:59.093681+08', 1770261629457, '2026-03-18 15:00:59.093681+08', '菜单管理侧边栏菜单是否显示');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (27, 'sys:permission:menu', '权限管理-页面入口', 2, false, 1770261629457, '2026-03-18 15:00:59.093681+08', 1770261629457, '2026-03-18 15:00:59.093681+08', '权限管理侧边栏菜单是否显示');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (28, 'sys:monitor:menu', '服务监控-页面入口', 2, false, 1770261629457, '2026-03-18 15:00:59.093681+08', 1770261629457, '2026-03-18 15:00:59.093681+08', '服务监控侧边栏菜单是否显示');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (29, 'sys:changelog:menu', '更新日志-页面入口', 2, false, 1770261629457, '2026-03-18 15:04:11.266865+08', 1770261629457, '2026-03-18 15:04:11.266865+08', '更新日志侧边栏菜单是否显示');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (30, 'sys:about:menu', '关于我们-页面入口', 2, false, 1770261629457, '2026-03-18 15:04:11.266865+08', 1770261629457, '2026-03-18 15:04:11.266865+08', '关于我们侧边栏菜单是否显示');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (31, 'sys:project:menu', '项目地址-页面入口', 2, false, 1770261629457, '2026-03-18 15:04:11.266865+08', 1770261629457, '2026-03-18 15:04:11.266865+08', '项目地址侧边栏菜单是否显示');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (32, 'sys:profile:menu', '个人中心-页面入口', 2, false, 1770261629457, '2026-03-18 15:09:17.373656+08', 1770261629457, '2026-03-18 15:09:17.373656+08', '个人中心侧边栏菜单是否显示');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (21, 'sys:monitor:server-info', '服务监控-查看', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '服务器信息监控菜单权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (33, 'sys:user:update-bind-roles', '用户管理-绑定角色', 1, false, 1770261629457, '2026-03-20 14:28:49.676336+08', 1770261629457, '2026-03-20 14:28:49.676336+08', '用户分配角色按钮权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (34, 'sys:user:list-bind-roles', '用户管理-查看绑定角色', 1, false, 1770261629457, '2026-03-20 14:28:49.676336+08', 1770261629457, '2026-03-20 14:28:49.676336+08', '查询用户已绑定角色列表');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (35, 'sys:role:update-bind-menus', '角色管理-绑定菜单', 1, false, 1770261629457, '2026-03-20 14:28:49.676336+08', 1770261629457, '2026-03-20 14:28:49.676336+08', '角色分配菜单按钮权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (36, 'sys:role:update-bind-perms', '角色管理-绑定权限', 1, false, 1770261629457, '2026-03-20 14:28:49.676336+08', 1770261629457, '2026-03-20 14:28:49.676336+08', '角色分配权限按钮权限');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (37, 'sys:role:list-bind-menus', '角色管理-查看绑定菜单', 1, false, 1770261629457, '2026-03-20 14:28:49.676336+08', 1770261629457, '2026-03-20 14:28:49.676336+08', '查询角色已绑定菜单列表');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (38, 'sys:role:list-bind-perms', '角色管理-查看绑定权限', 1, false, 1770261629457, '2026-03-20 14:28:49.676336+08', 1770261629457, '2026-03-20 14:28:49.676336+08', '查询角色已绑定权限列表');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (39, 'sys:audit:list', '审计日志-列表', 1, false, 1770261629457, '2026-03-25 10:32:16.369336+08', 1770261629457, '2026-03-25 10:32:16.369336+08', NULL);
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (40, 'sys:audit:menu', '审计日志-页面入口', 2, false, 1770261629457, '2026-03-25 10:32:16.369336+08', 1770261629457, '2026-03-25 10:32:16.369336+08', NULL);
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (1001001, 'sys:dept:list', '部门-查询', 1, false, 1, '2026-04-22 10:36:01.536272+08', 1, '2026-04-22 10:36:01.536272+08', NULL);
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (1001002, 'sys:dept:save', '部门-新增', 1, false, 1, '2026-04-22 10:36:01.536272+08', 1, '2026-04-22 10:36:01.536272+08', NULL);
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (1001003, 'sys:dept:update', '部门-修改', 1, false, 1, '2026-04-22 10:36:01.536272+08', 1, '2026-04-22 10:36:01.536272+08', NULL);
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (1001004, 'sys:dept:remove', '部门-删除', 1, false, 1, '2026-04-22 10:36:01.536272+08', 1, '2026-04-22 10:36:01.536272+08', NULL);
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (1001005, 'sys:dept:detail', '部门-详情', 1, false, 1, '2026-04-22 10:36:01.536272+08', 1, '2026-04-22 10:36:01.536272+08', NULL);
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (1001006, 'sys:dept:menu', '部门管理-页面入口', 2, false, 1, '2026-04-22 10:41:16.541073+08', 1, '2026-04-22 10:41:16.541073+08', '部门管理侧边栏菜单是否显示');
INSERT INTO public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (41, 'ai:ask:chat', 'AI对话', 1, false, 1770261629457, '2026-03-26 15:09:13.994767+08', 1770261629457, '2026-04-23 09:17:18.979861+08', NULL);


--
-- Data for Name: sys_menu; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public.sys_menu (id, parent_id, perm_id, name, menu_type, path, component, icon, sort_order, is_hidden, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (200, 0, NULL, '系统管理', 1, '/system', NULL, 'Setting', 1000, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL);
INSERT INTO public.sys_menu (id, parent_id, perm_id, name, menu_type, path, component, icon, sort_order, is_hidden, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (301, 300, 28, '服务监控', 2, '/monitor/server', 'view/monitor/server/index.vue', 'DataLine', 10, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL);
INSERT INTO public.sys_menu (id, parent_id, perm_id, name, menu_type, path, component, icon, sort_order, is_hidden, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (401, 400, 30, '关于我们', 2, '/other/about', 'view/other/about/index.vue', 'InfoFilled', 10, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL);
INSERT INTO public.sys_menu (id, parent_id, perm_id, name, menu_type, path, component, icon, sort_order, is_hidden, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (500, 0, 29, '更新日志', 2, '/changelog', 'view/changelog/index.vue', 'Notebook', 1030, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL);
INSERT INTO public.sys_menu (id, parent_id, perm_id, name, menu_type, path, component, icon, sort_order, is_hidden, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (403, 400, 32, '个人中心', 2, '/profile', 'view/profile/index.vue', 'User', 30, true, false, 1770261629457, '2026-03-18 15:09:17.373656+08', 1770261629457, '2026-03-18 15:09:17.373656+08', NULL);
INSERT INTO public.sys_menu (id, parent_id, perm_id, name, menu_type, path, component, icon, sort_order, is_hidden, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (300, 0, NULL, '系统监控', 1, '/monitor', NULL, 'Monitor', 1010, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL);
INSERT INTO public.sys_menu (id, parent_id, perm_id, name, menu_type, path, component, icon, sort_order, is_hidden, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (400, 0, NULL, '其他', 1, '/other', NULL, 'MoreFilled', 1020, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL);
INSERT INTO public.sys_menu (id, parent_id, perm_id, name, menu_type, path, component, icon, sort_order, is_hidden, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (402, 400, 31, '项目地址', 2, 'https://github.com/LitchiXian/rulo', NULL, 'Link', 20, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-20 14:49:26.63647+08', NULL);
INSERT INTO public.sys_menu (id, parent_id, perm_id, name, menu_type, path, component, icon, sort_order, is_hidden, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (302, 300, 40, '审计日志', 2, '/monitor/audit', 'view/monitor/audit/index.vue', 'Tickets', 20, false, false, 1770261629457, '2026-03-25 10:32:16.369336+08', 1770261629457, '2026-03-25 10:32:16.369336+08', NULL);
INSERT INTO public.sys_menu (id, parent_id, perm_id, name, menu_type, path, component, icon, sort_order, is_hidden, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (205, 200, 1001006, '部门管理', 2, '/system/dept', 'view/system/dept/index.vue', 'OfficeBuilding', 10, false, false, 1, '2026-04-22 10:41:16.542329+08', 1, '2026-04-22 17:23:25.604601+08', NULL);
INSERT INTO public.sys_menu (id, parent_id, perm_id, name, menu_type, path, component, icon, sort_order, is_hidden, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (201, 200, 24, '用户管理', 2, '/system/user', 'view/system/user/index.vue', 'UserFilled', 20, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-04-22 17:24:30.529103+08', NULL);
INSERT INTO public.sys_menu (id, parent_id, perm_id, name, menu_type, path, component, icon, sort_order, is_hidden, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (202, 200, 25, '角色管理', 2, '/system/role', 'view/system/role/index.vue', 'Key', 30, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-04-22 17:24:37.335118+08', NULL);
INSERT INTO public.sys_menu (id, parent_id, perm_id, name, menu_type, path, component, icon, sort_order, is_hidden, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (203, 200, 26, '菜单管理', 2, '/system/menu', 'view/system/menu/index.vue', 'Menu', 40, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-04-22 17:24:46.060727+08', NULL);
INSERT INTO public.sys_menu (id, parent_id, perm_id, name, menu_type, path, component, icon, sort_order, is_hidden, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (204, 200, 27, '权限管理', 2, '/system/permission', 'view/system/permission/index.vue', 'Lock', 50, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-04-23 09:03:37.937722+08', NULL);


--
-- Data for Name: sys_role; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public.sys_role (id, role_name, role_key, is_super, is_active, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (1, '超级管理员', 'super_admin', true, true, false, 1, '2026-04-22 15:35:45.200409+08', 1, '2026-04-22 15:35:45.200409+08', '系统拥有者，跳过所有鉴权');
INSERT INTO public.sys_role (id, role_name, role_key, is_super, is_active, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (2, '管理员', 'admin', false, true, false, 1, '2026-04-22 15:35:45.200409+08', 1, '2026-04-22 15:35:45.200409+08', '业务管理员，拥有大部分管理权限');
INSERT INTO public.sys_role (id, role_name, role_key, is_super, is_active, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (3, '普通用户', 'common', false, true, false, 1, '2026-04-22 15:35:45.200409+08', 1, '2026-04-22 15:35:45.200409+08', '普通用户，仅基础查看权限');
INSERT INTO public.sys_role (id, role_name, role_key, is_super, is_active, is_deleted, create_id, create_time, update_id, update_time, remark) VALUES (4, '测试角色', 'test', false, true, false, 1, '2026-04-22 15:35:45.200409+08', 1, '2026-04-22 15:35:45.200409+08', '测试专用，权限按需分配');


--
-- Data for Name: sys_role_menu; Type: TABLE DATA; Schema: public; Owner: -
--



--
-- Data for Name: sys_role_permission; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 24);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 25);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 26);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 27);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 28);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 29);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 30);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 31);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 32);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 40);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 1001006);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (3, 21);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (3, 22);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (3, 23);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (3, 28);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (3, 29);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (3, 30);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (3, 31);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (3, 32);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (4, 21);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (4, 22);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (4, 23);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (4, 29);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (4, 30);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (4, 31);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (4, 32);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 1);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 2);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 4);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 5);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 6);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 7);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 9);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 10);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 11);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 15);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 16);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 20);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 21);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 22);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 23);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 33);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 35);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 36);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 37);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 38);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 39);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 41);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 1001001);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 1001002);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 1001003);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 1001005);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 34);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 14);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 19);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (2, 1001004);
INSERT INTO public.sys_role_permission (role_id, perm_id) VALUES (3, 41);


--
-- Data for Name: sys_user; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public.sys_user (id, user_name, nick_name, password, email, is_active, is_deleted, create_id, create_time, update_id, update_time, remark, avatar_url) VALUES (1001, 'sadmin', '超级管理员', '$argon2id$v=19$m=19456,t=2,p=1$AuGjcjSi09i1OqzVsK7CmQ$yLiMSTtaZbwdOjjWK7pS3jrsxgKdURxG/0qcY6oeHi8', 'sadmin@ruruo.com', true, false, 1, '2026-04-22 15:35:45.200409+08', 1, '2026-04-22 15:35:45.212376+08', '系统超级管理员', NULL);
INSERT INTO public.sys_user (id, user_name, nick_name, password, email, is_active, is_deleted, create_id, create_time, update_id, update_time, remark, avatar_url) VALUES (1002, 'admin', '业务管理员', '$argon2id$v=19$m=19456,t=2,p=1$0L7ErQxkon7DyRTaFLoVpA$v7R7CT+BUu/TdYtmIRGeXjVnT2Nl8N7fsTKnPkhVL88', 'admin@ruruo.com', true, false, 1, '2026-04-22 15:35:45.200409+08', 1, '2026-04-22 15:35:45.212376+08', '业务管理员示范账号', NULL);
INSERT INTO public.sys_user (id, user_name, nick_name, password, email, is_active, is_deleted, create_id, create_time, update_id, update_time, remark, avatar_url) VALUES (1003, 'common01', '普通用户01', '$argon2id$v=19$m=19456,t=2,p=1$RK2EICs7uYwBmn7bvVnqAQ$kzEOP56Tv23XftwpkHtN9EFcc7UhsGcMrl1bJfKzpEk', 'common01@ruruo.com', true, false, 1, '2026-04-22 15:35:45.200409+08', 1, '2026-04-22 15:35:45.212376+08', '普通用户示范账号', NULL);
INSERT INTO public.sys_user (id, user_name, nick_name, password, email, is_active, is_deleted, create_id, create_time, update_id, update_time, remark, avatar_url) VALUES (1004, 'test01', '测试用户01', '$argon2id$v=19$m=19456,t=2,p=1$Gh2II12kx4Aq7rs+N4fKsA$N99o+YSzKhh6oVnONC1AQgbFhiyDwXciIehs00poIwk', 'test01@ruruo.com', true, false, 1, '2026-04-22 15:35:45.200409+08', 1, '2026-04-22 15:35:45.212376+08', '测试账号 01', NULL);
INSERT INTO public.sys_user (id, user_name, nick_name, password, email, is_active, is_deleted, create_id, create_time, update_id, update_time, remark, avatar_url) VALUES (1005, 'test02', '测试用户02', '$argon2id$v=19$m=19456,t=2,p=1$4hNkL2hj+7oUhSjtMBGGWQ$iM76O7w5KS0E9jlTxGYF703cWKLSwZ+KWfyaDmKA8bM', 'test02@ruruo.com', true, false, 1, '2026-04-22 15:35:45.200409+08', 1, '2026-04-22 15:35:45.212376+08', '测试账号 02', NULL);


--
-- Data for Name: sys_user_role; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public.sys_user_role (user_id, role_id) VALUES (1001, 1);
INSERT INTO public.sys_user_role (user_id, role_id) VALUES (1002, 2);
INSERT INTO public.sys_user_role (user_id, role_id) VALUES (1003, 3);
INSERT INTO public.sys_user_role (user_id, role_id) VALUES (1004, 4);
INSERT INTO public.sys_user_role (user_id, role_id) VALUES (1005, 4);


--
-- PostgreSQL database dump complete
--


