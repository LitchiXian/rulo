export interface Article {
    id: string;
    title: string;
    userId: string;
    userName: string;
    content: string;
    createTime: number;
}

export interface Tag {
    id: string;
    name: string;
}