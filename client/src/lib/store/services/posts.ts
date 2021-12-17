import type { CreatePost } from '$lib/api/post';
import type { Like, Post } from '$lib/types';
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
				url: `/posts/${postId}/hasLiked`,
				method: 'POST'
			}),
			invalidatesTags: ['PostDetails', 'PostLikeList', 'PostList']
		}),
		postCreate: builder.mutation<Post, CreatePost>({
			query: (body) => ({
				url: `/posts`,
				method: 'POST',
				body
			}),
			invalidatesTags: ['PostList']
		}),
		postDelete: builder.mutation<Post, string>({
			query: (postId) => ({
				url: `/posts/${postId}`,
				method: 'DELETE'
			}),
			invalidatesTags: ['PostList']
		}),
		uploadPhoto: builder.mutation<Post, Blob>({
			query: (imageBlob) => {
				const formData = new FormData();

				formData.append('file', imageBlob);

				return {
					url: `/posts/upload`,
					method: 'POST',
					headers: {
						'Content-type': 'multipart/form-data'
					},
					body: formData
				};
			},
			invalidatesTags: ['PostList']
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
