import type { AxiosResponse } from 'axios';
import axios from 'axios';
import { requests } from './agent';

export interface Post {
	id: string;
	createdAt: Date;
	updatedAt: Date;
	url: string;
	caption: string;
	lat?: number;
	lng?: number;
	userId: string;
	username: string;
	likes: number;
	comments: number;
}

interface Like {
	id: string;
	createdAt: string;
	userId: string;
	username: string;
	postId: string;
}

export interface CreatePost {
	url: string;
	caption: string;
}

export const Posts = {
	list: (): Promise<Post[]> => requests.get('/posts'),
	details: (postId: string): Promise<Post> => requests.get(`/posts/${postId}`),
	listLikes: (postId: string): Promise<Like[]> => requests.get(`/posts/${postId}/likes`),
	hasLiked: (postId: string): Promise<Like[]> => requests.get(`/posts/${postId}/hasLiked`),
	like: (postId: string): Promise<unknown> => requests.post(`/posts/${postId}/likes`, null),
	createPost: (post: CreatePost): Promise<Post> => requests.post(`/posts`, post),
	deletePost: (postId: string): Promise<unknown> => requests.delete(`/posts/${postId}`),
	uploadPhoto: (image: Blob): Promise<AxiosResponse<{ filepath?: string }>> => {
		const formData = new FormData();

		formData.append('file', image);

		return axios.post(`/posts/upload`, formData, {
			headers: {
				'Content-type': 'multipart/form-data'
			}
		});
	}
};
