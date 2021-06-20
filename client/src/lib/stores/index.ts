import { writable } from 'svelte/store';

export const accountsStore = writable({
	user: null
});
