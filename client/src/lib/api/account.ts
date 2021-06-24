import { requests } from './agent';

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

export const Account = {
	current: (): Promise<CurrentUser> => requests.get<CurrentUser>('/auth/current'),
	login: (user: LoginRequest): Promise<LoginOrSignUpResponse> =>
		requests.post<LoginRequest, LoginOrSignUpResponse>('/auth/login', user),
	signup: (user: SignUpRequest): Promise<LoginOrSignUpResponse> =>
		requests.post<LoginRequest, LoginOrSignUpResponse>('/auth/signup', user),
	logout: (): Promise<unknown> => requests.post('/auth/logout', {})
};
