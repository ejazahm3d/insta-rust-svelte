import { configureStore } from '@reduxjs/toolkit';
import type { EnhancedStore } from '@reduxjs/toolkit';
import type { Readable } from 'svelte/store';
import { readable } from 'svelte/store';
import { baseApi } from './services/baseApi';

const reduxStore = configureStore({
	reducer: {
		api: baseApi.reducer
	},
	middleware: (getDefaultMiddleware) => getDefaultMiddleware().concat(baseApi.middleware)
});

// Wrap the redux store with Svelte's readable store
const bindReduxStore = <T extends EnhancedStore<any, any, any>>(
	store: T
): {
	subscribe: Readable<ReturnType<T['getState']>>['subscribe'];
	dispatch: T['dispatch'];
} => {
	const state = readable(store.getState(), (set) => {
		const unsubscribe = store.subscribe(() => {
			set(store.getState());
		});
		return unsubscribe;
	});

	return {
		subscribe: state.subscribe,
		dispatch: store.dispatch
	};
};

export const store = bindReduxStore(reduxStore);
