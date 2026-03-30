--
-- PostgreSQL database dump
--

\restrict QGHcgyDDBSNh2LntqtuarHiqNN9LwCV6SbLfc4Zn49N7NwMPVaPRuQ6HhFdtMfa

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

--
-- Data for Name: databases; Type: TABLE DATA; Schema: _sqlx_test; Owner: rulo
--

COPY _sqlx_test.databases (db_name, test_path, created_at) FROM stdin;
\.


--
-- Data for Name: _sqlx_migrations; Type: TABLE DATA; Schema: public; Owner: rulo
--

COPY public._sqlx_migrations (version, description, installed_on, success, checksum, execution_time) FROM stdin;
20260310091035	init schema	2026-03-11 16:40:48.842428+08	t	\\xded750d9d98d11f7142016900ae2bfae48d658ec459600f41d1733360416fc3fc18c25120380e4c0c16975f010cfeb07	1765454
20260311081232	add rbac schema	2026-03-11 16:40:48.844789+08	t	\\xd02dc8e46ac90c7c09159d16c46cea7017a9fc43f326886f952454d59599cbe682de6d3b3acdcf51fdfbb6cbf2070e65	12059588
\.


--
-- Data for Name: b_blog_article; Type: TABLE DATA; Schema: public; Owner: rulo
--

COPY public.b_blog_article (id, title, content, user_id, user_name, published_time, is_published, is_deleted, create_id, create_time, update_id, update_time) FROM stdin;
1770270749362	测试文章	测试内容	1770261629457	litchi	2026-02-05 13:52:29+08	t	f	1770261629457	2026-02-05 13:52:29+08	1770261629457	2026-02-05 13:52:29+08
1770270771251	1234	1234	1770261629457	litchi	2026-02-05 13:52:51+08	t	f	1770261629457	2026-02-05 13:52:51+08	1770261629457	2026-02-05 13:52:51+08
1770271009228	新测试	新内容	1770261629457	litchi	2026-02-05 13:56:49+08	t	f	1770261629457	2026-02-05 13:56:49+08	1770261629457	2026-02-05 13:56:49+08
1770271044255	1234	1234	1770261629457	litchi	2026-02-05 13:57:24+08	t	f	1770261629457	2026-02-05 13:57:24+08	1770261629457	2026-02-05 13:57:24+08
1770276215554	1234	1234	1	\N	2026-02-05 15:23:35.554798+08	t	f	1	2026-02-05 15:23:35.554798+08	1	2026-02-05 15:23:35.554798+08
1770278041554	551234	12341234	1	\N	2026-02-05 15:54:01.554225+08	t	f	1	2026-02-05 15:54:01.554225+08	1	2026-02-05 15:54:01.554225+08
1770278674913	1234qeraw	爱上对方23412341234	1	\N	2026-02-05 16:04:34.913083+08	t	f	1	2026-02-05 16:04:34.913083+08	1	2026-02-05 16:04:34.913083+08
1770280996061	9999	9999	1770261629457	litchi	2026-02-05 16:43:16.061999+08	t	f	1770261629457	2026-02-05 16:43:16.061999+08	1770261629457	2026-02-05 16:43:16.061999+08
\.


--
-- Data for Name: b_tag; Type: TABLE DATA; Schema: public; Owner: rulo
--

COPY public.b_tag (id, name, remark, status, create_id, create_time, update_id, update_time) FROM stdin;
1000000000001	Rust	Rust 语言相关	t	1770261629457	2026-03-10 17:42:54.420678+08	\N	\N
1000000000002	Java	Java 语言相关	t	1770261629457	2026-03-10 17:42:54.420678+08	\N	\N
1000000000003	Vue	Vue 前端框架	t	1770261629457	2026-03-10 17:42:54.420678+08	\N	\N
1000000000004	PostgreSQL	关系型数据库	t	1770261629457	2026-03-10 17:42:54.420678+08	\N	\N
1000000000005	Linux	Linux 运维	t	1770261629457	2026-03-10 17:42:54.420678+08	\N	\N
\.


--
-- Data for Name: b_article_tag; Type: TABLE DATA; Schema: public; Owner: rulo
--

COPY public.b_article_tag (article_id, tag_id) FROM stdin;
\.


--
-- Data for Name: sys_audit_log; Type: TABLE DATA; Schema: public; Owner: rulo
--

COPY public.sys_audit_log (id, user_id, user_name, method, path, params, status, duration_ms, ip, is_sensitive, created_time) FROM stdin;
\.


--
-- Data for Name: sys_permission; Type: TABLE DATA; Schema: public; Owner: rulo
--

COPY public.sys_permission (id, perm_code, perm_name, perm_type, is_deleted, create_id, create_time, update_id, update_time, remark) FROM stdin;
1	sys:user:list	用户管理-列表	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	用户列表页面菜单权限
6	sys:role:list	角色管理-列表	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	角色列表页面菜单权限
11	sys:menu:list	菜单管理-列表	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	菜单列表页面菜单权限
16	sys:permission:list	权限管理-列表	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	权限列表页面菜单权限
2	sys:user:save	用户管理-新增	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	新增用户按钮权限
3	sys:user:remove	用户管理-删除	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	删除用户按钮权限
4	sys:user:update	用户管理-编辑	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	编辑用户按钮权限
5	sys:user:detail	用户管理-详情	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	用户详情查看权限
7	sys:role:save	角色管理-新增	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	新增角色按钮权限
8	sys:role:remove	角色管理-删除	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	删除角色按钮权限
9	sys:role:update	角色管理-编辑	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	编辑角色按钮权限
10	sys:role:detail	角色管理-详情	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	角色详情查看权限
12	sys:menu:save	菜单管理-新增	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	新增菜单按钮权限
13	sys:menu:remove	菜单管理-删除	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	删除菜单按钮权限
14	sys:menu:update	菜单管理-编辑	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	编辑菜单按钮权限
15	sys:menu:detail	菜单管理-详情	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	菜单详情查看权限
17	sys:permission:save	权限管理-新增	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	新增权限按钮权限
18	sys:permission:remove	权限管理-删除	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	删除权限按钮权限
19	sys:permission:update	权限管理-编辑	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	编辑权限按钮权限
20	sys:permission:detail	权限管理-详情	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	权限详情查看权限
22	sys:auth:info	个人中心-查看	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	当前用户信息API权限，所有登录用户均有
23	sys:auth:logout	退出登录	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	退出登录API权限，所有登录用户均有
24	sys:user:menu	用户管理-页面入口	2	f	1770261629457	2026-03-18 15:00:59.093681+08	1770261629457	2026-03-18 15:00:59.093681+08	用户管理侧边栏菜单是否显示
25	sys:role:menu	角色管理-页面入口	2	f	1770261629457	2026-03-18 15:00:59.093681+08	1770261629457	2026-03-18 15:00:59.093681+08	角色管理侧边栏菜单是否显示
26	sys:menu:menu	菜单管理-页面入口	2	f	1770261629457	2026-03-18 15:00:59.093681+08	1770261629457	2026-03-18 15:00:59.093681+08	菜单管理侧边栏菜单是否显示
27	sys:permission:menu	权限管理-页面入口	2	f	1770261629457	2026-03-18 15:00:59.093681+08	1770261629457	2026-03-18 15:00:59.093681+08	权限管理侧边栏菜单是否显示
28	sys:monitor:menu	服务监控-页面入口	2	f	1770261629457	2026-03-18 15:00:59.093681+08	1770261629457	2026-03-18 15:00:59.093681+08	服务监控侧边栏菜单是否显示
29	sys:changelog:menu	更新日志-页面入口	2	f	1770261629457	2026-03-18 15:04:11.266865+08	1770261629457	2026-03-18 15:04:11.266865+08	更新日志侧边栏菜单是否显示
30	sys:about:menu	关于我们-页面入口	2	f	1770261629457	2026-03-18 15:04:11.266865+08	1770261629457	2026-03-18 15:04:11.266865+08	关于我们侧边栏菜单是否显示
31	sys:project:menu	项目地址-页面入口	2	f	1770261629457	2026-03-18 15:04:11.266865+08	1770261629457	2026-03-18 15:04:11.266865+08	项目地址侧边栏菜单是否显示
32	sys:profile:menu	个人中心-页面入口	2	f	1770261629457	2026-03-18 15:09:17.373656+08	1770261629457	2026-03-18 15:09:17.373656+08	个人中心侧边栏菜单是否显示
21	sys:monitor:server-info	服务监控-查看	1	f	1770261629457	2026-03-18 14:29:46.515304+08	1770261629457	2026-03-18 14:29:46.515304+08	服务器信息监控菜单权限
33	sys:user:update-bind-roles	用户管理-绑定角色	1	f	1770261629457	2026-03-20 14:28:49.676336+08	1770261629457	2026-03-20 14:28:49.676336+08	用户分配角色按钮权限
34	sys:user:list-bind-roles	用户管理-查看绑定角色	1	f	1770261629457	2026-03-20 14:28:49.676336+08	1770261629457	2026-03-20 14:28:49.676336+08	查询用户已绑定角色列表
35	sys:role:update-bind-menus	角色管理-绑定菜单	1	f	1770261629457	2026-03-20 14:28:49.676336+08	1770261629457	2026-03-20 14:28:49.676336+08	角色分配菜单按钮权限
36	sys:role:update-bind-perms	角色管理-绑定权限	1	f	1770261629457	2026-03-20 14:28:49.676336+08	1770261629457	2026-03-20 14:28:49.676336+08	角色分配权限按钮权限
37	sys:role:list-bind-menus	角色管理-查看绑定菜单	1	f	1770261629457	2026-03-20 14:28:49.676336+08	1770261629457	2026-03-20 14:28:49.676336+08	查询角色已绑定菜单列表
38	sys:role:list-bind-perms	角色管理-查看绑定权限	1	f	1770261629457	2026-03-20 14:28:49.676336+08	1770261629457	2026-03-20 14:28:49.676336+08	查询角色已绑定权限列表
39	sys:audit:list	审计日志-列表	1	f	1770261629457	2026-03-25 10:32:16.369336+08	1770261629457	2026-03-25 10:32:16.369336+08	\N
40	sys:audit:menu	审计日志-页面入口	2	f	1770261629457	2026-03-25 10:32:16.369336+08	1770261629457	2026-03-25 10:32:16.369336+08	\N
41	ai:ask:chat	AI对话	1	f	1770261629457	2026-03-26 15:09:13.994767+08	1770261629457	2026-03-26 15:09:13.994767+08	\N
7443214863368851456	sys_user	测试菜单1-页面入口	2	f	7443214863368851456	2026-03-27 16:38:31.671961+08	7443214863368851456	2026-03-27 16:38:31.671961+08	\N
\.


--
-- Data for Name: sys_menu; Type: TABLE DATA; Schema: public; Owner: rulo
--

COPY public.sys_menu (id, parent_id, perm_id, name, menu_type, path, component, icon, sort_order, is_hidden, is_deleted, create_id, create_time, update_id, update_time, remark) FROM stdin;
200	0	\N	系统管理	1	/system	\N	Setting	1000	f	f	1770261629457	2026-03-18 15:06:30.568831+08	1770261629457	2026-03-18 15:06:30.568831+08	\N
201	200	24	用户管理	2	/system/user	view/system/user/index.vue	UserFilled	10	f	f	1770261629457	2026-03-18 15:06:30.568831+08	1770261629457	2026-03-18 15:06:30.568831+08	\N
202	200	25	角色管理	2	/system/role	view/system/role/index.vue	Key	20	f	f	1770261629457	2026-03-18 15:06:30.568831+08	1770261629457	2026-03-18 15:06:30.568831+08	\N
203	200	26	菜单管理	2	/system/menu	view/system/menu/index.vue	Menu	30	f	f	1770261629457	2026-03-18 15:06:30.568831+08	1770261629457	2026-03-18 15:06:30.568831+08	\N
204	200	27	权限管理	2	/system/permission	view/system/permission/index.vue	Lock	40	f	f	1770261629457	2026-03-18 15:06:30.568831+08	1770261629457	2026-03-18 15:06:30.568831+08	\N
301	300	28	服务监控	2	/monitor/server	view/monitor/server/index.vue	DataLine	10	f	f	1770261629457	2026-03-18 15:06:30.568831+08	1770261629457	2026-03-18 15:06:30.568831+08	\N
401	400	30	关于我们	2	/other/about	view/other/about/index.vue	InfoFilled	10	f	f	1770261629457	2026-03-18 15:06:30.568831+08	1770261629457	2026-03-18 15:06:30.568831+08	\N
500	0	29	更新日志	2	/changelog	view/changelog/index.vue	Notebook	1030	f	f	1770261629457	2026-03-18 15:06:30.568831+08	1770261629457	2026-03-18 15:06:30.568831+08	\N
403	400	32	个人中心	2	/profile	view/profile/index.vue	User	30	t	f	1770261629457	2026-03-18 15:09:17.373656+08	1770261629457	2026-03-18 15:09:17.373656+08	\N
300	0	\N	系统监控	1	/monitor	\N	Monitor	1010	f	f	1770261629457	2026-03-18 15:06:30.568831+08	1770261629457	2026-03-18 15:06:30.568831+08	\N
400	0	\N	其他	1	/other	\N	MoreFilled	1020	f	f	1770261629457	2026-03-18 15:06:30.568831+08	1770261629457	2026-03-18 15:06:30.568831+08	\N
402	400	31	项目地址	2	https://github.com/LitchiXian/rulo	\N	Link	20	f	f	1770261629457	2026-03-18 15:06:30.568831+08	1770261629457	2026-03-20 14:49:26.63647+08	\N
302	300	40	审计日志	2	/monitor/audit	view/monitor/audit/index.vue	Tickets	20	f	f	1770261629457	2026-03-25 10:32:16.369336+08	1770261629457	2026-03-25 10:32:16.369336+08	\N
\.


--
-- Data for Name: sys_role; Type: TABLE DATA; Schema: public; Owner: rulo
--

COPY public.sys_role (id, role_name, role_key, is_super, is_active, is_deleted, create_id, create_time, update_id, update_time, remark) FROM stdin;
1	超级管理员	super_admin	t	t	f	1770261629457	2026-03-18 15:13:03.283803+08	1770261629457	2026-03-18 15:13:03.283803+08	拥有所有权限，跳过所有鉴权校验
2	管理员	admin	f	t	f	1770261629457	2026-03-18 15:13:03.283803+08	1770261629457	2026-03-18 15:13:03.283803+08	门户管理员，拥有大部分管理权限
3	普通角色	user	f	t	f	1770261629457	2026-03-18 15:13:03.283803+08	1770261629457	2026-03-20 11:06:11.461463+08	普通用户，仅基础查看权限
4	测试角色	tester	f	t	f	1770261629457	2026-03-18 15:13:03.283803+08	1770261629457	2026-03-20 17:02:47.085602+08	测试专用角色，权限按需分配,我要写得很长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长长
\.


--
-- Data for Name: sys_role_menu; Type: TABLE DATA; Schema: public; Owner: rulo
--

COPY public.sys_role_menu (role_id, menu_id, is_hidden) FROM stdin;
1	302	f
2	302	f
\.


--
-- Data for Name: sys_role_permission; Type: TABLE DATA; Schema: public; Owner: rulo
--

COPY public.sys_role_permission (role_id, perm_id) FROM stdin;
1	1
1	6
1	11
1	16
1	21
1	2
1	3
1	4
1	5
1	7
1	8
1	9
1	10
1	12
1	13
1	14
1	15
1	17
1	18
1	19
1	20
1	22
1	23
1	24
1	25
1	26
1	27
1	28
1	29
1	30
1	31
1	32
3	28
3	29
3	30
3	31
3	32
3	22
3	23
3	21
4	29
4	30
4	31
4	32
4	22
4	23
4	21
1	39
1	40
2	1
2	11
2	16
2	21
2	2
2	4
2	5
2	25
2	31
2	29
2	30
2	40
2	26
2	27
2	28
2	32
2	24
2	12
2	13
2	14
2	15
2	17
2	19
2	20
2	22
2	23
2	10
2	6
2	37
2	38
2	7
2	9
2	35
2	36
2	33
2	39
2	41
\.


--
-- Data for Name: sys_user; Type: TABLE DATA; Schema: public; Owner: rulo
--

COPY public.sys_user (id, user_name, nick_name, password, email, is_active, is_deleted, create_id, create_time, update_id, update_time, remark, avatar_url) FROM stdin;
1773382000194	lzx222	lzx222	$argon2id$v=19$m=19456,t=2,p=1$j9SakD6AwyT9gKD0WvtUgw$TnuynDkut6Pc+Dbl7F3hCdy1iZprOsYwpDfwUcvEgaQ	\N	t	f	1773382000194	2026-03-13 14:06:40.194833+08	1773382000194	2026-03-13 14:06:40.194833+08	\N	\N
1773729892189	lzx333	lzx333	$argon2id$v=19$m=19456,t=2,p=1$3uUS8K6+0jJoanZAdJs9TQ$tBV8MuvA3NxCi+GQR8yKggAuDiDqCt5ZduEeYeqbxeA	333@qq.com	t	f	1773729892189	2026-03-17 14:44:52.189848+08	1773729892189	2026-03-17 14:44:52.189848+08	\N	\N
1770261629457	litchi	litchi	$argon2id$v=19$m=19456,t=2,p=1$mAlYqqnt43F537NA/mydlw$qTrQG3C3T/b+8ycMYlGN5hX3XP3rTJ/COxRL1gliEW4	3468845278@qq.com	t	f	1770261629457	2026-02-05 11:20:29+08	1770261629457	2026-02-05 11:20:29+08	\N	\N
1773301283713	lzx111	lzx111	$argon2id$v=19$m=19456,t=2,p=1$mAlYqqnt43F537NA/mydlw$qTrQG3C3T/b+8ycMYlGN5hX3XP3rTJ/COxRL1gliEW4	\N	t	f	1773301283713	2026-03-12 15:41:23.713735+08	1773301283713	2026-03-25 09:44:14.388621+08	\N	2026/03/25/336dc3b5-1776-4c3b-951d-ebcc2b74a12d.jpg
\.


--
-- Data for Name: sys_user_role; Type: TABLE DATA; Schema: public; Owner: rulo
--

COPY public.sys_user_role (user_id, role_id) FROM stdin;
1770261629457	1
1773382000194	3
1773729892189	4
1773301283713	2
\.


--
-- Name: database_ids; Type: SEQUENCE SET; Schema: _sqlx_test; Owner: rulo
--

SELECT pg_catalog.setval('_sqlx_test.database_ids', 1, false);


--
-- PostgreSQL database dump complete
--

\unrestrict QGHcgyDDBSNh2LntqtuarHiqNN9LwCV6SbLfc4Zn49N7NwMPVaPRuQ6HhFdtMfa

