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
	url: string;
	caption: string;
	userId: string;
}

const Posts = {
	list: (): Promise<Post[]> => requests.get<Post[]>('/posts')
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

export default { Posts, Account };
