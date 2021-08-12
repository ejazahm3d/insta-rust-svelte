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
		isFollowing: boolean;
	}>({
		profile: null,
		posts: [],
		followers: 0,
		following: 0,
		isFollowing: false
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
			const data = await agent.Followers.followersCount(userId);
			update((prevState) => ({ ...prevState, followers: data }));
		},
		following: async (userId: string) => {
			const data = await agent.Leaders.leadersCount(userId);
			update((prevState) => ({ ...prevState, following: data }));
		},
		isFollowing: async (userId: string) => {
			const isFollowing = await agent.Leaders.isFollowing(userId);
			update((prevState) => ({ ...prevState, isFollowing }));
		},
		followOrUnfollow: async (userId: string) => {
			await agent.Leaders.followOrUnfollow(userId);

			const following = await agent.Leaders.leadersCount(userId);
			const followers = await agent.Followers.followersCount(userId);
			const posts = await agent.Profiles.posts(userId);
			const isFollowing = await agent.Leaders.isFollowing(userId);

			update((prevState) => ({ ...prevState, followers, following, posts, isFollowing }));
		}
	};
}

export const profilesStore = createAccountStore();
