import type { Post } from '$lib/types';
import { requests } from './agent';

export interface Profile {
	id: string;
	email: string;
	username: string;
	created_at: string;
	updated_at: string;
	avatar: null | string;
	bio: null | string;
}

export const Profiles = {
	details: (userId: string): Promise<Profile> => requests.get(`/profiles/${userId}`),
	posts: (userId: string): Promise<Post[]> => requests.get(`/profiles/${userId}/posts`)
};
