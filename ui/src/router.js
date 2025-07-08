import { createRouter, createWebHistory } from 'vue-router';
import Home from '@/pages/Home.vue';
import Post from '@/pages/Post.vue';
import NotFound from '@/pages/NotFound.vue';

const router = createRouter({
    history: createWebHistory(),
    routes: [
        { path: '/', component: Home },
        { path: '/post/:id', component: Post, props: true },
        { path: '/:pathMatch(.*)*', component: NotFound }
    ]
});

export default router;