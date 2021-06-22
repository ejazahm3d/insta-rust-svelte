import type { CreatePost, LoginRequest, Post, SignUpRequest } from '$lib/api/agent';
import agent from '$lib/api/agent';
import { writable } from 'svelte/store';

function createAccountStore() {
	const { subscribe, set } = writable({
		user: null
	});

	return {
		subscribe,
		fetchCurrentUser: async () => {
			const data = await agent.Account.current();
			set({ user: data?.user });
		},
		login: async (user: LoginRequest) => {
			const data = await agent.Account.login(user);

			set({ user: data.id });
		},
		logout: async () => {
			await agent.Account.logout();
			set({ user: null });
		},
		signup: async (user: SignUpRequest) => {
			const data = await agent.Account.signup(user);
			set({ user: data.id });
		}
	};
}

export const accountsStore = createAccountStore();

function createPostsStore() {
	const { set, subscribe } = writable<{
		posts: Post[];
	}>({
		posts: []
	});

	return {
		subscribe,
		fetchPosts: async () => {
			const posts = await agent.Posts.list();
			set({ posts });
			return posts;
		},
		likePost: async (postId: string) => {
			await agent.Posts.like(postId);
			const posts = await agent.Posts.list();

			set({ posts });
		},
		createPost: async (post: CreatePost) => {
			await agent.Posts.createPost(post);
			const posts = await agent.Posts.list();
			set({ posts });
		}
	};
}

export const postsStore = createPostsStore();
