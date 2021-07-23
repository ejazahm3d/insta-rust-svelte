import type { LoginRequest, SignUpRequest } from '$lib/api/account';
import agent from '$lib/api';
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
