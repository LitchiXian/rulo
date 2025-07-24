import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import router from '@/router';
import { login as loginApi, logout as logoutApi, getLoginInfo } from '@/api/web/login';
import type { UserInfo, LoginDto } from '@/type/user';
import {useRoute} from "vue-router";

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

            const response = await loginApi(credentials);
            token.value = response.data;

            // 持久化存储
            if (credentials.remember) {
                localStorage.setItem('satoken', token.value);
            } else {
                sessionStorage.setItem('satoken', token.value);
            }
            const route = useRoute();
            const redirectPath = route.query.redirect?.toString() || '/';
            router.push(redirectPath);
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
                console.log( response)
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