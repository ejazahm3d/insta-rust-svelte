import type {
	Comment,
	CreateComment,
	CreatePost,
	LoginRequest,
	Post,
	SignUpRequest
} from '$lib/api/agent';
import agent from '$lib/api/agent';
import { writable } from 'svelte/store';

function createAccountStore() {
	const { subscribe, set } = writable<{
		user: {
			id: string;
		};
	}>({
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

			set({ user: { id: data.id } });
		},
		logout: async () => {
			await agent.Account.logout();
			set({ user: null });
		},
		signup: async (user: SignUpRequest) => {
			const data = await agent.Account.signup(user);
			set({ user: { id: data.id } });
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
		},
		deletePost: async (postId: string) => {
			await agent.Posts.deletePost(postId);
			const posts = await agent.Posts.list();
			set({ posts });
		}
	};
}

export const postsStore = createPostsStore();

function createCommentsStore() {
	const { set, subscribe, update } = writable<{
		currentPost: Post;
		comments: Comment[];
	}>({
		currentPost: null,
		comments: []
	});

	return {
		subscribe,
		fetchComments: async (postId: string) => {
			const comments = await agent.Comments.list(postId);
			update((v) => ({ ...v, comments }));
		},
		likeComment: async (postId: string, commentId: string) => {
			await agent.Comments.like(postId, commentId);
			const comments = await agent.Comments.list(postId);
			update((v) => ({ ...v, comments }));
		},
		createComment: async (postId: string, comment: CreateComment) => {
			await agent.Comments.create(postId, comment);
			const comments = await agent.Comments.list(postId);
			update((v) => ({ ...v, comments }));
		},
		deleteComment: async (postId: string, commentId: string) => {
			await agent.Comments.delete(postId, commentId);
			const comments = await agent.Comments.list(postId);
			update((v) => ({ ...v, comments }));
		}
	};
}

export const commentsStore = createCommentsStore();
