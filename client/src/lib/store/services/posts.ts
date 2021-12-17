import type { Like, Post, CreatePost } from '$lib/types';
import { baseApi } from './baseApi';

export const postsApi = baseApi.injectEndpoints({
	endpoints: (builder) => ({
		postList: builder.query<Post[], void>({
			query: () => ({
				url: '/posts',
				method: 'GET'
			}),
			providesTags: ['PostList']
		}),
		postDetails: builder.query<Post, string>({
			query: (postId) => ({
				url: `/posts/${postId}`,
				method: 'GET'
			}),
			providesTags: ['PostDetails']
		}),
		postLikeList: builder.query<Like[], string>({
			query: (postId) => ({
				url: `/posts/${postId}/likes`,
				method: 'GET'
			}),
			providesTags: ['PostLikeList']
		}),
		hasLikedPost: builder.query<boolean, string>({
			query: (postId) => ({
				url: `/posts/${postId}/hasLiked`,
				method: 'GET'
			}),
			providesTags: ['PostLikeList']
		}),
		postLike: builder.mutation<unknown, string>({
			query: (postId) => ({
				url: `/posts/${postId}/likes`,
				method: 'POST'
			}),
			invalidatesTags: ['PostDetails', 'PostLikeList', 'PostList', 'PostsByProfile']
		}),
		postCreate: builder.mutation<Post, CreatePost>({
			query: (body) => ({
				url: `/posts`,
				method: 'POST',
				body
			}),
			invalidatesTags: (res) => ['PostList', { type: 'PostsByProfile', id: res.userId }]
		}),
		postDelete: builder.mutation<Post, string>({
			query: (postId) => ({
				url: `/posts/${postId}`,
				method: 'DELETE'
			}),
			invalidatesTags: (res) => ['PostList', { type: 'PostsByProfile', id: res.userId }]
		}),
		uploadPhoto: builder.mutation<Post, Blob>({
			query: (imageBlob) => {
				const formData = new FormData();

				formData.append('file', imageBlob);

				return {
					url: `/posts/upload`,
					method: 'POST',
					body: formData
				};
			}
		})
	}),
	overrideExisting: false
});

export const {
	postList,
	hasLikedPost,
	postLike,
	postDetails,
	postLikeList,
	postCreate,
	postDelete,
	uploadPhoto
} = postsApi.endpoints;
