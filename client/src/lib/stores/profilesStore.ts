import agent from '$lib/api';
import type { Post } from '$lib/api/post';
import type { Profile } from '$lib/api/profile';
import { writable } from 'svelte/store';

function createAccountStore() {
	const { subscribe, update } = writable<{
		profile: Profile;
		posts: Post[];
		followers: number;
		following: number;
	}>({
		profile: null,
		posts: [],
		followers: 0,
		following: 0
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
		},
		followers: async (userId: string) => {
			const data = await agent.Followers.followersByLeaderId(userId);
			update((prevState) => ({ ...prevState, followers: data.length }));
		},

		following: async (userId: string) => {
			const data = await agent.Leaders.leadersByFollowerId(userId);
			update((prevState) => ({ ...prevState, following: data.length }));
		}
	};
}

export const profilesStore = createAccountStore();
