import {defineStore} from 'pinia';
import {computed, ref} from 'vue';
import router from '@/router/router.ts';
import authApi from '@/api/web/auth.ts';
import type {LoginDto, UserInfo} from '@/type/user';

export const useUserStore = defineStore('user', () => {
    // 状态
    const userInfo = ref<UserInfo | null>(null);
    const token = ref<string>('');
    const loading = ref(false);
    const error = ref<string | null>(null);

    // 计算属性
    const isLoggedIn = computed(() => !!token.value);
    const userName = computed(() => userInfo.value?.nickName || '');
    const userAvatar = computed(() => userInfo.value?.avatar || '');
    // const isAdmin = computed(() => userInfo.value?.role === 'admin');

    // 登录方法
    const login = async (credentials: LoginDto) => {

        try {
            loading.value = true;
            error.value = null;

            token.value = ((await authApi.login(credentials)) as unknown) as string;

            // 持久化存储
            if (credentials.remember) {
                localStorage.setItem('satoken', token.value);
            } else {
                sessionStorage.setItem('satoken', token.value);
            }

            router.push(credentials.redirect || '/');
        } catch (err: any) {
            error.value = err.response?.data?.msg || '登录失败';
            throw err;
        } finally {
            loading.value = false;
        }
    };

    // 登出方法
    const logout = async () => {
        try {
            if (isLoggedIn.value) {
                await authApi.logout();
            }
        } finally {
            clearUserData();
            router.push('/login');
        }
    };

    // 清除用户数据
    const clearUserData = () => {
        token.value = '';
        userInfo.value = null;
        localStorage.removeItem('satoken');
        sessionStorage.removeItem('satoken');
    };

    // 初始化用户数据
    const initUser = async () => {
        const savedToken = sessionStorage.getItem('satoken') || localStorage.getItem('satoken');
        if (savedToken) {
            token.value = savedToken;
            try {
                userInfo.value = await authApi.getLoginInfo();
                console.log("用户信息",userInfo.value)
            } catch (err) {
                // token失效时清理数据
                clearUserData();
            }
        }
    };

    return {
        userInfo,
        token,
        loading,
        error,
        isLoggedIn,
        userName,
        userAvatar,
        login,
        logout,
        initUser
    };
});