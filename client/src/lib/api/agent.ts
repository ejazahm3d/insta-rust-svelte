import axios, { AxiosResponse } from 'axios';

axios.defaults.baseURL = 'http://localhost:5000/api';
axios.defaults.withCredentials = true;

const responseBody = <T>(response: AxiosResponse<T>): T => response.data;

const requests = {
	get: <T>(url: string) => axios.get<T>(url).then(responseBody),
	post: <T, R>(url: string, body: T) => axios.post<R>(url, body).then(responseBody),
	put: <T, R>(url: string, body: T) => axios.put<R>(url, body).then(responseBody),
	delete: <T>(url: string) => axios.delete<T>(url).then(responseBody)
};

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

const Posts = {
	list: (): Promise<Post[]> => requests.get('/posts'),
	details: (postId: string): Promise<Post> => requests.get(`/posts/${postId}`),
	listLikes: (postId: string): Promise<Like[]> => requests.get(`/posts/${postId}/likes`),
	like: (postId: string): Promise<unknown> => requests.post(`/posts/${postId}/likes`, null),
	createPost: (post: CreatePost): Promise<Post> => requests.post(`/posts`, post),
	deletePost: (postId: string): Promise<unknown> => requests.delete(`/posts/${postId}`)
};

export interface LoginRequest {
	email: string;
	password: string;
}

export interface LoginOrSignUpResponse {
	id: string;
	email: string;
	username: string;
}

export interface SignUpRequest {
	email: string;
	password: string;
	username: string;
}

export interface CurrentUser {
	user: {
		id: string;
	};
}

const Account = {
	current: (): Promise<CurrentUser> => requests.get<CurrentUser>('/auth/current'),
	login: (user: LoginRequest): Promise<LoginOrSignUpResponse> =>
		requests.post<LoginRequest, LoginOrSignUpResponse>('/auth/login', user),
	signup: (user: SignUpRequest): Promise<LoginOrSignUpResponse> =>
		requests.post<LoginRequest, LoginOrSignUpResponse>('/auth/signup', user),
	logout: (): Promise<unknown> => requests.post('/auth/logout', {})
};

export interface Comment {
	id: string;
	createdAt: string;
	updatedAt: string;
	contents: string;
	userId: string;
	postId: string;
}

export interface CreateComment {
	contents: string;
}

const Comments = {
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

export default { Posts, Account, Comments };
