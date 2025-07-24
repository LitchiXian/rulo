import {defineStore} from 'pinia';
import {computed, ref} from 'vue';
import router from '@/router';
import {getLoginInfo, login as loginApi, logout as logoutApi} from '@/api/web/login';
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

            token.value = ((await loginApi(credentials)) as unknown) as string;

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
            console.log('login', isLoggedIn.value)
            console.log('login', token.value)
            if (isLoggedIn.value) {
                await logoutApi();
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
        const savedToken = localStorage.getItem('satoken') || sessionStorage.getItem('satoken');
        if (savedToken) {
            token.value = savedToken;
            try {
                const response = await getLoginInfo();
                console.log(response)
                userInfo.value = response.data;
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