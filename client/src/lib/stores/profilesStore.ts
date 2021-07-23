import agent from '$lib/api';
import type { Profile } from '$lib/api/profile';
import { writable } from 'svelte/store';

function createAccountStore() {
	const { subscribe, set } = writable<{
		profile: Profile;
	}>({
		profile: null
	});

	return {
		subscribe,
		details: async (userId: string) => {
			const data = await agent.Profiles.details(userId);
			set({ profile: data });
		}
	};
}

export const profilesStore = createAccountStore();
