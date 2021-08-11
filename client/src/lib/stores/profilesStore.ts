import agent from '$lib/api';
import type { Post } from '$lib/api/post';
import type { Profile } from '$lib/api/profile';
import { writable } from 'svelte/store';

function createAccountStore() {
	const { subscribe, update } = writable<{
		profile: Profile;
		posts: Post[];
	}>({
		profile: null,
		posts: []
	});

	return {
		subscribe,
		details: async (userId: string) => {
			const data = await agent.Profiles.details(userId);
			update((prevState) => ({ ...prevState, profile: data }));
		},

		posts: async (userId: string) => {
			const data = await agent.Profiles.posts(userId);
			update((prevState) => ({ ...prevState, posts: data }));
		}
	};
}

export const profilesStore = createAccountStore();
