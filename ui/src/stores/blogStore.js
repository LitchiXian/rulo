import { defineStore } from 'pinia';
import posts from '@/data/posts.json';

interface Post {
    id: number;
    title: string;
    date: string;
    tags: string[];
    content: string;
}

export const useBlogStore = defineStore('blog', {
    state: () => ({
        posts: posts as Post[]
    }),
    actions: {
        getPostById(id: number): Post | undefined {
            return this.posts.find(post => post.id === id);
        }
    }
});