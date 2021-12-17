import type { CreateComment, Comment } from '$lib/types';
import { baseApi } from './baseApi';

export const commentsApi = baseApi.injectEndpoints({
	endpoints: (builder) => ({
		commentList: builder.query<Comment[], string>({
			query: (postId) => ({
				url: `/posts/${postId}/comments`,
				method: 'GET'
			}),
			providesTags: (_result, _error, arg) => [{ type: 'CommentList', id: arg }]
		}),
		commentLikeList: builder.query<Comment[], { postId: string; commentId: string }>({
			query: ({ postId, commentId }) => ({
				url: `/posts/${postId}/comments/${commentId}/likes`,
				method: 'GET'
			}),
			providesTags: ['CommentLikeList']
		}),
		hasLikedComment: builder.query<boolean, { postId: string; commentId: string }>({
			query: ({ postId, commentId }) => ({
				url: `/posts/${postId}/comments/${commentId}/hasLiked`,
				method: 'GET'
			}),
			providesTags: ['CommentLikeList']
		}),
		commentLike: builder.mutation<unknown, { postId: string; commentId: string }>({
			query: ({ postId, commentId }) => ({
				url: `/posts/${postId}/comments/${commentId}/likes`,
				method: 'POST'
			}),
			invalidatesTags: (_res, _err, arg) => [
				{ type: 'CommentList', id: arg.postId },
				'CommentLikeList'
			]
		}),
		commentCreate: builder.mutation<Comment, { body: CreateComment; postId: string }>({
			query: ({ body, postId }) => ({
				url: `/posts/${postId}/comments`,
				method: 'POST',
				body
			}),
			invalidatesTags: (_res, _err, arg) => [{ type: 'CommentList', id: arg.postId }, 'PostList']
		}),
		commentDelete: builder.mutation<Comment, { postId: string; commentId: string }>({
			query: ({ postId, commentId }) => ({
				url: `/posts/${postId}/comments/${commentId}`,
				method: 'DELETE'
			}),
			invalidatesTags: (_res, _err, arg) => [{ type: 'CommentList', id: arg.postId }, 'PostList']
		})
	}),
	overrideExisting: false
});

export const {
	commentList,
	commentCreate,
	commentDelete,
	commentLike,
	commentLikeList,
	hasLikedComment
} = commentsApi.endpoints;
