import agent from '$lib/api';
import type { CreatePost, Post } from '$lib/api/post';
import { writable } from 'svelte/store';

function createPostsStore() {
	const { subscribe, update } = writable<{
		posts: Post[];
		postDetails: Post;
	}>({
		posts: [],
		postDetails: null
	});

	return {
		subscribe,
		fetchPosts: async () => {
			const posts = await agent.Posts.list();
			update((prevState) => ({ ...prevState, posts }));
			return posts;
		},
		likePostForList: async (postId: string) => {
			await agent.Posts.like(postId);
			const posts = await agent.Posts.list();

			update((prevState) => ({ ...prevState, posts }));
		},
		likePostForDetails: async (postId: string) => {
			await agent.Posts.like(postId);
			const postDetails = await agent.Posts.details(postId);

			update((prevState) => ({ ...prevState, postDetails }));
		},
		createPost: async (post: CreatePost) => {
			await agent.Posts.createPost(post);
			const posts = await agent.Posts.list();
			update((prevState) => ({ ...prevState, posts }));
		},
		deletePost: async (postId: string) => {
			await agent.Posts.deletePost(postId);
			const posts = await agent.Posts.list();
			update((prevState) => ({ ...prevState, posts }));
		},
		fetchPost: async (postId: string) => {
			const postDetails = await agent.Posts.details(postId);
			update((prevState) => ({ ...prevState, postDetails }));
		},
		uploadPhoto: async (image: Blob) => {
			const res = await agent.Posts.uploadPhoto(image);
			return res.data;
		}
	};
}

export const postsStore = createPostsStore();
