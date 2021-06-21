import type { Post } from '$lib/api/agent';
import agent from '$lib/api/agent';
import { writable } from 'svelte/store';

export const accountsStore = writable({
	user: null
});

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
		}
	};
}

export const postsStore = createPostsStore();
