--
-- PostgreSQL database dump
--

\restrict a1neC3LoBVeNA403mYl25bwnH1lvTDwosMbDXIiXjsNdMjAXPX1uV5S2j3h6o0t

-- Dumped from database version 17.9 (Debian 17.9-0+deb13u1)
-- Dumped by pg_dump version 17.9 (Debian 17.9-0+deb13u1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: _sqlx_migrations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public._sqlx_migrations (
    version bigint NOT NULL,
    description text NOT NULL,
    installed_on timestamp with time zone DEFAULT now() NOT NULL,
    success boolean NOT NULL,
    checksum bytea NOT NULL,
    execution_time bigint NOT NULL
);


--
-- Name: b_article_tag; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.b_article_tag (
    article_id bigint NOT NULL,
    tag_id bigint NOT NULL
);


--
-- Name: TABLE b_article_tag; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.b_article_tag IS '文章与标签关联关系表';


--
-- Name: COLUMN b_article_tag.article_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.b_article_tag.article_id IS '文章ID';


--
-- Name: COLUMN b_article_tag.tag_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.b_article_tag.tag_id IS '标签ID';


--
-- Name: b_blog_article; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.b_blog_article (
    id bigint NOT NULL,
    title character varying(255) NOT NULL,
    content text NOT NULL,
    user_id bigint NOT NULL,
    user_name character varying(255),
    published_time timestamp with time zone,
    is_published boolean DEFAULT false NOT NULL,
    is_deleted boolean DEFAULT false NOT NULL,
    create_id bigint NOT NULL,
    create_time timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    update_id bigint NOT NULL,
    update_time timestamp with time zone
);


--
-- Name: TABLE b_blog_article; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.b_blog_article IS '博客文章核心表';


--
-- Name: COLUMN b_blog_article.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.b_blog_article.id IS 'ID';


--
-- Name: COLUMN b_blog_article.title; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.b_blog_article.title IS '文章标题';


--
-- Name: COLUMN b_blog_article.content; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.b_blog_article.content IS 'Markdown内容';


--
-- Name: COLUMN b_blog_article.user_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.b_blog_article.user_id IS '作者ID';


--
-- Name: COLUMN b_blog_article.user_name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.b_blog_article.user_name IS '作者名称（冗余存储）';


--
-- Name: COLUMN b_blog_article.published_time; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.b_blog_article.published_time IS '发布时间';


--
-- Name: COLUMN b_blog_article.is_published; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.b_blog_article.is_published IS '是否发布（false=私密，true=公开）';


--
-- Name: COLUMN b_blog_article.is_deleted; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.b_blog_article.is_deleted IS '逻辑删除标志（false=正常，true=已删除）';


--
-- Name: COLUMN b_blog_article.create_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.b_blog_article.create_id IS '创建人ID';


--
-- Name: COLUMN b_blog_article.create_time; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.b_blog_article.create_time IS '创建时间';


--
-- Name: COLUMN b_blog_article.update_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.b_blog_article.update_id IS '更新人ID';


--
-- Name: COLUMN b_blog_article.update_time; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.b_blog_article.update_time IS '更新时间';


--
-- Name: b_tag; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.b_tag (
    id bigint NOT NULL,
    name character varying(50) NOT NULL,
    remark character varying(500),
    status boolean DEFAULT true NOT NULL,
    create_id bigint NOT NULL,
    create_time timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL,
    update_id bigint,
    update_time timestamp with time zone
);


--
-- Name: TABLE b_tag; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TABLE public.b_tag IS '标签主表';


--
-- Name: COLUMN b_tag.id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.b_tag.id IS '标签ID';


--
-- Name: COLUMN b_tag.name; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.b_tag.name IS '标签名称';


--
-- Name: COLUMN b_tag.remark; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.b_tag.remark IS '标签描述';


--
-- Name: COLUMN b_tag.status; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.b_tag.status IS '状态（false=禁用，true=启用）';


--
-- Name: COLUMN b_tag.create_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.b_tag.create_id IS '创建者用户ID';


--
-- Name: COLUMN b_tag.create_time; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.b_tag.create_time IS '创建时间';


--
-- Name: COLUMN b_tag.update_id; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.b_tag.update_id IS '最后更新者用户ID';


--
-- Name: COLUMN b_tag.update_time; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.b_tag.update_time IS '最后更新时间';


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

COMMENT ON COLUMN public.sys_permission.perm_type IS '类型：1=菜单 2=按钮 3=API';


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

COMMENT ON TABLE public.sys_role_permission IS '系统_角色权限关联表（后端鉴权基于此表，不基于角色）';


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
    remark character varying(500)
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

COMMENT ON COLUMN public.sys_user.password IS '密码（bcrypt哈希结果）';


--
-- Name: COLUMN sys_user.email; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_user.email IS '用户邮箱';


--
-- Name: COLUMN sys_user.is_active; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON COLUMN public.sys_user.is_active IS '账户状态（false=正常，true=异常）';


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
-- Data for Name: _sqlx_migrations; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public._sqlx_migrations VALUES (20260310091035, 'init schema', '2026-03-11 16:40:48.842428+08', true, '\xded750d9d98d11f7142016900ae2bfae48d658ec459600f41d1733360416fc3fc18c25120380e4c0c16975f010cfeb07', 1765454);
INSERT INTO public._sqlx_migrations VALUES (20260311081232, 'add rbac schema', '2026-03-11 16:40:48.844789+08', true, '\xd02dc8e46ac90c7c09159d16c46cea7017a9fc43f326886f952454d59599cbe682de6d3b3acdcf51fdfbb6cbf2070e65', 12059588);


--
-- Data for Name: b_article_tag; Type: TABLE DATA; Schema: public; Owner: -
--



--
-- Data for Name: b_blog_article; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public.b_blog_article VALUES (1770270749362, '测试文章', '测试内容', 1770261629457, 'litchi', '2026-02-05 13:52:29+08', true, false, 1770261629457, '2026-02-05 13:52:29+08', 1770261629457, '2026-02-05 13:52:29+08');
INSERT INTO public.b_blog_article VALUES (1770270771251, '1234', '1234', 1770261629457, 'litchi', '2026-02-05 13:52:51+08', true, false, 1770261629457, '2026-02-05 13:52:51+08', 1770261629457, '2026-02-05 13:52:51+08');
INSERT INTO public.b_blog_article VALUES (1770271009228, '新测试', '新内容', 1770261629457, 'litchi', '2026-02-05 13:56:49+08', true, false, 1770261629457, '2026-02-05 13:56:49+08', 1770261629457, '2026-02-05 13:56:49+08');
INSERT INTO public.b_blog_article VALUES (1770271044255, '1234', '1234', 1770261629457, 'litchi', '2026-02-05 13:57:24+08', true, false, 1770261629457, '2026-02-05 13:57:24+08', 1770261629457, '2026-02-05 13:57:24+08');
INSERT INTO public.b_blog_article VALUES (1770276215554, '1234', '1234', 1, NULL, '2026-02-05 15:23:35.554798+08', true, false, 1, '2026-02-05 15:23:35.554798+08', 1, '2026-02-05 15:23:35.554798+08');
INSERT INTO public.b_blog_article VALUES (1770278041554, '551234', '12341234', 1, NULL, '2026-02-05 15:54:01.554225+08', true, false, 1, '2026-02-05 15:54:01.554225+08', 1, '2026-02-05 15:54:01.554225+08');
INSERT INTO public.b_blog_article VALUES (1770278674913, '1234qeraw', '爱上对方23412341234', 1, NULL, '2026-02-05 16:04:34.913083+08', true, false, 1, '2026-02-05 16:04:34.913083+08', 1, '2026-02-05 16:04:34.913083+08');
INSERT INTO public.b_blog_article VALUES (1770280996061, '9999', '9999', 1770261629457, 'litchi', '2026-02-05 16:43:16.061999+08', true, false, 1770261629457, '2026-02-05 16:43:16.061999+08', 1770261629457, '2026-02-05 16:43:16.061999+08');


--
-- Data for Name: b_tag; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public.b_tag VALUES (1000000000001, 'Rust', 'Rust 语言相关', true, 1770261629457, '2026-03-10 17:42:54.420678+08', NULL, NULL);
INSERT INTO public.b_tag VALUES (1000000000002, 'Java', 'Java 语言相关', true, 1770261629457, '2026-03-10 17:42:54.420678+08', NULL, NULL);
INSERT INTO public.b_tag VALUES (1000000000003, 'Vue', 'Vue 前端框架', true, 1770261629457, '2026-03-10 17:42:54.420678+08', NULL, NULL);
INSERT INTO public.b_tag VALUES (1000000000004, 'PostgreSQL', '关系型数据库', true, 1770261629457, '2026-03-10 17:42:54.420678+08', NULL, NULL);
INSERT INTO public.b_tag VALUES (1000000000005, 'Linux', 'Linux 运维', true, 1770261629457, '2026-03-10 17:42:54.420678+08', NULL, NULL);


--
-- Data for Name: sys_menu; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public.sys_menu VALUES (200, 0, NULL, '系统管理', 1, '/system', NULL, 'Setting', 1000, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL);
INSERT INTO public.sys_menu VALUES (201, 200, 24, '用户管理', 2, '/system/user', 'view/system/user/index.vue', 'UserFilled', 10, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL);
INSERT INTO public.sys_menu VALUES (202, 200, 25, '角色管理', 2, '/system/role', 'view/system/role/index.vue', 'Key', 20, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL);
INSERT INTO public.sys_menu VALUES (203, 200, 26, '菜单管理', 2, '/system/menu', 'view/system/menu/index.vue', 'Menu', 30, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL);
INSERT INTO public.sys_menu VALUES (204, 200, 27, '权限管理', 2, '/system/permission', 'view/system/permission/index.vue', 'Lock', 40, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL);
INSERT INTO public.sys_menu VALUES (301, 300, 28, '服务监控', 2, '/monitor/server', 'view/monitor/server/index.vue', 'DataLine', 10, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL);
INSERT INTO public.sys_menu VALUES (401, 400, 30, '关于我们', 2, '/other/about', 'view/other/about/index.vue', 'InfoFilled', 10, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL);
INSERT INTO public.sys_menu VALUES (500, 0, 29, '更新日志', 2, '/changelog', 'view/changelog/index.vue', 'Notebook', 1030, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL);
INSERT INTO public.sys_menu VALUES (403, 400, 32, '个人中心', 2, '/profile', 'view/profile/index.vue', 'User', 30, true, false, 1770261629457, '2026-03-18 15:09:17.373656+08', 1770261629457, '2026-03-18 15:09:17.373656+08', NULL);
INSERT INTO public.sys_menu VALUES (300, 0, NULL, '系统监控', 1, '/monitor', NULL, 'Monitor', 1010, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL);
INSERT INTO public.sys_menu VALUES (400, 0, NULL, '其他', 1, '/other', NULL, 'MoreFilled', 1020, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-18 15:06:30.568831+08', NULL);
INSERT INTO public.sys_menu VALUES (402, 400, 31, '项目地址', 2, 'https://github.com/LitchiXian/rulo', NULL, 'Link', 20, false, false, 1770261629457, '2026-03-18 15:06:30.568831+08', 1770261629457, '2026-03-20 14:49:26.63647+08', NULL);


--
-- Data for Name: sys_permission; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public.sys_permission VALUES (1, 'sys:user:list', '用户管理-列表', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '用户列表页面菜单权限');
INSERT INTO public.sys_permission VALUES (6, 'sys:role:list', '角色管理-列表', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '角色列表页面菜单权限');
INSERT INTO public.sys_permission VALUES (11, 'sys:menu:list', '菜单管理-列表', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '菜单列表页面菜单权限');
INSERT INTO public.sys_permission VALUES (16, 'sys:permission:list', '权限管理-列表', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '权限列表页面菜单权限');
INSERT INTO public.sys_permission VALUES (2, 'sys:user:save', '用户管理-新增', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '新增用户按钮权限');
INSERT INTO public.sys_permission VALUES (3, 'sys:user:remove', '用户管理-删除', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '删除用户按钮权限');
INSERT INTO public.sys_permission VALUES (4, 'sys:user:update', '用户管理-编辑', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '编辑用户按钮权限');
INSERT INTO public.sys_permission VALUES (5, 'sys:user:detail', '用户管理-详情', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '用户详情查看权限');
INSERT INTO public.sys_permission VALUES (7, 'sys:role:save', '角色管理-新增', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '新增角色按钮权限');
INSERT INTO public.sys_permission VALUES (8, 'sys:role:remove', '角色管理-删除', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '删除角色按钮权限');
INSERT INTO public.sys_permission VALUES (9, 'sys:role:update', '角色管理-编辑', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '编辑角色按钮权限');
INSERT INTO public.sys_permission VALUES (10, 'sys:role:detail', '角色管理-详情', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '角色详情查看权限');
INSERT INTO public.sys_permission VALUES (12, 'sys:menu:save', '菜单管理-新增', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '新增菜单按钮权限');
INSERT INTO public.sys_permission VALUES (13, 'sys:menu:remove', '菜单管理-删除', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '删除菜单按钮权限');
INSERT INTO public.sys_permission VALUES (14, 'sys:menu:update', '菜单管理-编辑', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '编辑菜单按钮权限');
INSERT INTO public.sys_permission VALUES (15, 'sys:menu:detail', '菜单管理-详情', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '菜单详情查看权限');
INSERT INTO public.sys_permission VALUES (17, 'sys:permission:save', '权限管理-新增', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '新增权限按钮权限');
INSERT INTO public.sys_permission VALUES (18, 'sys:permission:remove', '权限管理-删除', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '删除权限按钮权限');
INSERT INTO public.sys_permission VALUES (19, 'sys:permission:update', '权限管理-编辑', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '编辑权限按钮权限');
INSERT INTO public.sys_permission VALUES (20, 'sys:permission:detail', '权限管理-详情', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '权限详情查看权限');
INSERT INTO public.sys_permission VALUES (22, 'sys:auth:info', '个人中心-查看', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '当前用户信息API权限，所有登录用户均有');
INSERT INTO public.sys_permission VALUES (23, 'sys:auth:logout', '退出登录', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '退出登录API权限，所有登录用户均有');
INSERT INTO public.sys_permission VALUES (24, 'sys:user:menu', '用户管理-页面入口', 2, false, 1770261629457, '2026-03-18 15:00:59.093681+08', 1770261629457, '2026-03-18 15:00:59.093681+08', '用户管理侧边栏菜单是否显示');
INSERT INTO public.sys_permission VALUES (25, 'sys:role:menu', '角色管理-页面入口', 2, false, 1770261629457, '2026-03-18 15:00:59.093681+08', 1770261629457, '2026-03-18 15:00:59.093681+08', '角色管理侧边栏菜单是否显示');
INSERT INTO public.sys_permission VALUES (26, 'sys:menu:menu', '菜单管理-页面入口', 2, false, 1770261629457, '2026-03-18 15:00:59.093681+08', 1770261629457, '2026-03-18 15:00:59.093681+08', '菜单管理侧边栏菜单是否显示');
INSERT INTO public.sys_permission VALUES (27, 'sys:permission:menu', '权限管理-页面入口', 2, false, 1770261629457, '2026-03-18 15:00:59.093681+08', 1770261629457, '2026-03-18 15:00:59.093681+08', '权限管理侧边栏菜单是否显示');
INSERT INTO public.sys_permission VALUES (28, 'sys:monitor:menu', '服务监控-页面入口', 2, false, 1770261629457, '2026-03-18 15:00:59.093681+08', 1770261629457, '2026-03-18 15:00:59.093681+08', '服务监控侧边栏菜单是否显示');
INSERT INTO public.sys_permission VALUES (29, 'sys:changelog:menu', '更新日志-页面入口', 2, false, 1770261629457, '2026-03-18 15:04:11.266865+08', 1770261629457, '2026-03-18 15:04:11.266865+08', '更新日志侧边栏菜单是否显示');
INSERT INTO public.sys_permission VALUES (30, 'sys:about:menu', '关于我们-页面入口', 2, false, 1770261629457, '2026-03-18 15:04:11.266865+08', 1770261629457, '2026-03-18 15:04:11.266865+08', '关于我们侧边栏菜单是否显示');
INSERT INTO public.sys_permission VALUES (31, 'sys:project:menu', '项目地址-页面入口', 2, false, 1770261629457, '2026-03-18 15:04:11.266865+08', 1770261629457, '2026-03-18 15:04:11.266865+08', '项目地址侧边栏菜单是否显示');
INSERT INTO public.sys_permission VALUES (32, 'sys:profile:menu', '个人中心-页面入口', 2, false, 1770261629457, '2026-03-18 15:09:17.373656+08', 1770261629457, '2026-03-18 15:09:17.373656+08', '个人中心侧边栏菜单是否显示');
INSERT INTO public.sys_permission VALUES (21, 'sys:monitor:server-info', '服务监控-查看', 1, false, 1770261629457, '2026-03-18 14:29:46.515304+08', 1770261629457, '2026-03-18 14:29:46.515304+08', '服务器信息监控菜单权限');
INSERT INTO public.sys_permission VALUES (33, 'sys:user:update-bind-roles', '用户管理-绑定角色', 1, false, 1770261629457, '2026-03-20 14:28:49.676336+08', 1770261629457, '2026-03-20 14:28:49.676336+08', '用户分配角色按钮权限');
INSERT INTO public.sys_permission VALUES (34, 'sys:user:list-bind-roles', '用户管理-查看绑定角色', 1, false, 1770261629457, '2026-03-20 14:28:49.676336+08', 1770261629457, '2026-03-20 14:28:49.676336+08', '查询用户已绑定角色列表');
INSERT INTO public.sys_permission VALUES (35, 'sys:role:update-bind-menus', '角色管理-绑定菜单', 1, false, 1770261629457, '2026-03-20 14:28:49.676336+08', 1770261629457, '2026-03-20 14:28:49.676336+08', '角色分配菜单按钮权限');
INSERT INTO public.sys_permission VALUES (36, 'sys:role:update-bind-perms', '角色管理-绑定权限', 1, false, 1770261629457, '2026-03-20 14:28:49.676336+08', 1770261629457, '2026-03-20 14:28:49.676336+08', '角色分配权限按钮权限');
INSERT INTO public.sys_permission VALUES (37, 'sys:role:list-bind-menus', '角色管理-查看绑定菜单', 1, false, 1770261629457, '2026-03-20 14:28:49.676336+08', 1770261629457, '2026-03-20 14:28:49.676336+08', '查询角色已绑定菜单列表');
INSERT INTO public.sys_permission VALUES (38, 'sys:role:list-bind-perms', '角色管理-查看绑定权限', 1, false, 1770261629457, '2026-03-20 14:28:49.676336+08', 1770261629457, '2026-03-20 14:28:49.676336+08', '查询角色已绑定权限列表');


--
-- Data for Name: sys_role; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public.sys_role VALUES (1, '超级管理员', 'super_admin', true, true, false, 1770261629457, '2026-03-18 15:13:03.283803+08', 1770261629457, '2026-03-18 15:13:03.283803+08', '拥有所有权限，跳过所有鉴权校验');
INSERT INTO public.sys_role VALUES (2, '管理员', 'admin', false, true, false, 1770261629457, '2026-03-18 15:13:03.283803+08', 1770261629457, '2026-03-18 15:13:03.283803+08', '门户管理员，拥有大部分管理权限');
INSERT INTO public.sys_role VALUES (3, '普通角色', 'user', false, true, false, 1770261629457, '2026-03-18 15:13:03.283803+08', 1770261629457, '2026-03-20 11:06:11.461463+08', '普通用户，仅基础查看权限');
INSERT INTO public.sys_role VALUES (4, '测试角色', 'tester', false, true, false, 1770261629457, '2026-03-18 15:13:03.283803+08', 1770261629457, '2026-03-20 17:02:47.085602+08', '测试专用角色，权限按需分配,我要写得很长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长');


--
-- Data for Name: sys_role_menu; Type: TABLE DATA; Schema: public; Owner: -
--



--
-- Data for Name: sys_role_permission; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public.sys_role_permission VALUES (1, 1);
INSERT INTO public.sys_role_permission VALUES (1, 6);
INSERT INTO public.sys_role_permission VALUES (1, 11);
INSERT INTO public.sys_role_permission VALUES (1, 16);
INSERT INTO public.sys_role_permission VALUES (1, 21);
INSERT INTO public.sys_role_permission VALUES (1, 2);
INSERT INTO public.sys_role_permission VALUES (1, 3);
INSERT INTO public.sys_role_permission VALUES (1, 4);
INSERT INTO public.sys_role_permission VALUES (1, 5);
INSERT INTO public.sys_role_permission VALUES (1, 7);
INSERT INTO public.sys_role_permission VALUES (1, 8);
INSERT INTO public.sys_role_permission VALUES (1, 9);
INSERT INTO public.sys_role_permission VALUES (2, 25);
INSERT INTO public.sys_role_permission VALUES (1, 10);
INSERT INTO public.sys_role_permission VALUES (2, 31);
INSERT INTO public.sys_role_permission VALUES (1, 12);
INSERT INTO public.sys_role_permission VALUES (2, 29);
INSERT INTO public.sys_role_permission VALUES (1, 13);
INSERT INTO public.sys_role_permission VALUES (2, 30);
INSERT INTO public.sys_role_permission VALUES (1, 14);
INSERT INTO public.sys_role_permission VALUES (2, 26);
INSERT INTO public.sys_role_permission VALUES (1, 15);
INSERT INTO public.sys_role_permission VALUES (2, 27);
INSERT INTO public.sys_role_permission VALUES (1, 17);
INSERT INTO public.sys_role_permission VALUES (1, 18);
INSERT INTO public.sys_role_permission VALUES (2, 28);
INSERT INTO public.sys_role_permission VALUES (1, 19);
INSERT INTO public.sys_role_permission VALUES (2, 32);
INSERT INTO public.sys_role_permission VALUES (1, 20);
INSERT INTO public.sys_role_permission VALUES (2, 24);
INSERT INTO public.sys_role_permission VALUES (1, 22);
INSERT INTO public.sys_role_permission VALUES (1, 23);
INSERT INTO public.sys_role_permission VALUES (1, 24);
INSERT INTO public.sys_role_permission VALUES (1, 25);
INSERT INTO public.sys_role_permission VALUES (1, 26);
INSERT INTO public.sys_role_permission VALUES (1, 27);
INSERT INTO public.sys_role_permission VALUES (1, 28);
INSERT INTO public.sys_role_permission VALUES (1, 29);
INSERT INTO public.sys_role_permission VALUES (1, 30);
INSERT INTO public.sys_role_permission VALUES (1, 31);
INSERT INTO public.sys_role_permission VALUES (1, 32);
INSERT INTO public.sys_role_permission VALUES (3, 28);
INSERT INTO public.sys_role_permission VALUES (3, 29);
INSERT INTO public.sys_role_permission VALUES (3, 30);
INSERT INTO public.sys_role_permission VALUES (3, 31);
INSERT INTO public.sys_role_permission VALUES (3, 32);
INSERT INTO public.sys_role_permission VALUES (3, 22);
INSERT INTO public.sys_role_permission VALUES (3, 23);
INSERT INTO public.sys_role_permission VALUES (3, 21);
INSERT INTO public.sys_role_permission VALUES (4, 29);
INSERT INTO public.sys_role_permission VALUES (4, 30);
INSERT INTO public.sys_role_permission VALUES (4, 31);
INSERT INTO public.sys_role_permission VALUES (4, 32);
INSERT INTO public.sys_role_permission VALUES (4, 22);
INSERT INTO public.sys_role_permission VALUES (4, 23);
INSERT INTO public.sys_role_permission VALUES (4, 21);
INSERT INTO public.sys_role_permission VALUES (2, 1);
INSERT INTO public.sys_role_permission VALUES (2, 11);
INSERT INTO public.sys_role_permission VALUES (2, 16);
INSERT INTO public.sys_role_permission VALUES (2, 21);
INSERT INTO public.sys_role_permission VALUES (2, 2);
INSERT INTO public.sys_role_permission VALUES (2, 4);
INSERT INTO public.sys_role_permission VALUES (2, 5);
INSERT INTO public.sys_role_permission VALUES (2, 12);
INSERT INTO public.sys_role_permission VALUES (2, 13);
INSERT INTO public.sys_role_permission VALUES (2, 14);
INSERT INTO public.sys_role_permission VALUES (2, 15);
INSERT INTO public.sys_role_permission VALUES (2, 17);
INSERT INTO public.sys_role_permission VALUES (2, 19);
INSERT INTO public.sys_role_permission VALUES (2, 20);
INSERT INTO public.sys_role_permission VALUES (2, 22);
INSERT INTO public.sys_role_permission VALUES (2, 23);
INSERT INTO public.sys_role_permission VALUES (2, 10);
INSERT INTO public.sys_role_permission VALUES (2, 6);
INSERT INTO public.sys_role_permission VALUES (2, 37);
INSERT INTO public.sys_role_permission VALUES (2, 38);
INSERT INTO public.sys_role_permission VALUES (2, 7);
INSERT INTO public.sys_role_permission VALUES (2, 9);
INSERT INTO public.sys_role_permission VALUES (2, 35);
INSERT INTO public.sys_role_permission VALUES (2, 36);
INSERT INTO public.sys_role_permission VALUES (2, 33);


--
-- Data for Name: sys_user; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public.sys_user VALUES (1773301283713, 'lzx111', 'lzx111', '$argon2id$v=19$m=19456,t=2,p=1$mAlYqqnt43F537NA/mydlw$qTrQG3C3T/b+8ycMYlGN5hX3XP3rTJ/COxRL1gliEW4', NULL, true, false, 1773301283713, '2026-03-12 15:41:23.713735+08', 1773301283713, '2026-03-12 15:41:23.713735+08', NULL);
INSERT INTO public.sys_user VALUES (1773382000194, 'lzx222', 'lzx222', '$argon2id$v=19$m=19456,t=2,p=1$j9SakD6AwyT9gKD0WvtUgw$TnuynDkut6Pc+Dbl7F3hCdy1iZprOsYwpDfwUcvEgaQ', NULL, true, false, 1773382000194, '2026-03-13 14:06:40.194833+08', 1773382000194, '2026-03-13 14:06:40.194833+08', NULL);
INSERT INTO public.sys_user VALUES (1773729892189, 'lzx333', 'lzx333', '$argon2id$v=19$m=19456,t=2,p=1$3uUS8K6+0jJoanZAdJs9TQ$tBV8MuvA3NxCi+GQR8yKggAuDiDqCt5ZduEeYeqbxeA', '333@qq.com', true, false, 1773729892189, '2026-03-17 14:44:52.189848+08', 1773729892189, '2026-03-17 14:44:52.189848+08', NULL);
INSERT INTO public.sys_user VALUES (1773738757682, '1773738757682', 'lzx444', '123456', NULL, true, false, 1773738757682, '2026-03-17 17:12:37.682416+08', 1773738757682, '2026-03-17 17:12:44.125249+08', '我也有备注了222');
INSERT INTO public.sys_user VALUES (1770261629457, 'litchi', 'litchi', '$argon2id$v=19$m=19456,t=2,p=1$mAlYqqnt43F537NA/mydlw$qTrQG3C3T/b+8ycMYlGN5hX3XP3rTJ/COxRL1gliEW4', '3468845278@qq.com', true, false, 1770261629457, '2026-02-05 11:20:29+08', 1770261629457, '2026-02-05 11:20:29+08', NULL);


--
-- Data for Name: sys_user_role; Type: TABLE DATA; Schema: public; Owner: -
--

INSERT INTO public.sys_user_role VALUES (1770261629457, 1);
INSERT INTO public.sys_user_role VALUES (1773382000194, 3);
INSERT INTO public.sys_user_role VALUES (1773729892189, 4);
INSERT INTO public.sys_user_role VALUES (1773301283713, 2);


--
-- Name: _sqlx_migrations _sqlx_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public._sqlx_migrations
    ADD CONSTRAINT _sqlx_migrations_pkey PRIMARY KEY (version);


--
-- Name: b_article_tag b_article_tag_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.b_article_tag
    ADD CONSTRAINT b_article_tag_pkey PRIMARY KEY (article_id, tag_id);


--
-- Name: b_blog_article b_blog_article_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.b_blog_article
    ADD CONSTRAINT b_blog_article_pkey PRIMARY KEY (id);


--
-- Name: b_tag b_tag_name_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.b_tag
    ADD CONSTRAINT b_tag_name_key UNIQUE (name);


--
-- Name: b_tag b_tag_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.b_tag
    ADD CONSTRAINT b_tag_pkey PRIMARY KEY (id);


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
-- Name: idx_article_create_time; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_article_create_time ON public.b_blog_article USING btree (create_time DESC);


--
-- Name: idx_article_is_deleted; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_article_is_deleted ON public.b_blog_article USING btree (is_deleted);


--
-- Name: idx_article_user_id; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_article_user_id ON public.b_blog_article USING btree (user_id);


--
-- Name: idx_tag_status; Type: INDEX; Schema: public; Owner: -
--

CREATE INDEX idx_tag_status ON public.b_tag USING btree (status);


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
-- Name: b_article_tag b_article_tag_article_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.b_article_tag
    ADD CONSTRAINT b_article_tag_article_id_fkey FOREIGN KEY (article_id) REFERENCES public.b_blog_article(id) ON DELETE CASCADE;


--
-- Name: b_article_tag b_article_tag_tag_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.b_article_tag
    ADD CONSTRAINT b_article_tag_tag_id_fkey FOREIGN KEY (tag_id) REFERENCES public.b_tag(id) ON DELETE CASCADE;


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

\unrestrict a1neC3LoBVeNA403mYl25bwnH1lvTDwosMbDXIiXjsNdMjAXPX1uV5S2j3h6o0t

