import type { Post } from '$lib/api/agent';
import { writable } from 'svelte/store';

export const accountsStore = writable({
	user: null
});

export const postsStore = writable<{
	posts: Post[];
}>({
	posts: []
});
