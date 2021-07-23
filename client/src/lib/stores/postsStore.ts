import agent from '$lib/api';
import type { CreatePost, Post } from '$lib/api/post';
import { writable } from 'svelte/store';

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
		},
		uploadPhoto: async (image: Blob) => {
			const res = await agent.Posts.uploadPhoto(image);
			return res.data;
		}
	};
}

export const postsStore = createPostsStore();
