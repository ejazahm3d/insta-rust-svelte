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

export interface Like {
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

export interface Profile {
	id: string;
	email: string;
	username: string;
	created_at: string;
	updated_at: string;
	avatar: null | string;
	bio: null | string;
}

export interface Follower {
	id: string;
	createdAt: Date;
	leaderId: string;
	followerId: string;
}
