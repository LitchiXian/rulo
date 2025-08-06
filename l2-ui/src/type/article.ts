export interface Article {
    id: string;
    title: string;
    userId: string;
    userName: string;
    content: string;
    createTime: number;
    tags: string;
}

export interface Tag {
    id: string;
    name: string;
}