import type { Post } from '$lib/types';
import { baseApi } from './baseApi';

export const postsApi = baseApi.injectEndpoints({
	endpoints: (builder) => ({
		postList: builder.query<Post[], void>({
			query: () => ({
				url: '/posts',
				method: 'GET'
			}),
			providesTags: ['PostList']
		})
	}),
	overrideExisting: false
});
