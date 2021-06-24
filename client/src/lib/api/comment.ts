import { requests } from './agent';

export interface Comment {
	id: string;
	createdAt: string;
	updatedAt: string;
	contents: string;
	userId: string;
	username: string;
	postId: string;
	likes: number;
}

export interface CreateComment {
	contents: string;
}

export const Comments = {
	list: (postId: string): Promise<Comment[]> => requests.get(`/posts/${postId}/comments`),

	listLikes: (postId: string, commentId: string): Promise<Comment[]> =>
		requests.get(`/posts/${postId}/comments/${commentId}/likes`),

	create: (postId: string, comment: CreateComment): Promise<Comment> =>
		requests.post(`/posts/${postId}/comments`, comment),

	like: (postId: string, commentId: string): Promise<unknown> =>
		requests.post(`/posts/${postId}/comments/${commentId}/likes`, {}),

	delete: (postId: string, commentId: string): Promise<Comment> =>
		requests.delete(`/posts/${postId}/comments/${commentId}`)
};
