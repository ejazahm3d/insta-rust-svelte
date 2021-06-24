import agent from '$lib/api/agent';
import type { CreateComment, Comment } from '$lib/api/comment';
import type { Post } from '$lib/api/post';
import { writable } from 'svelte/store';

function createCommentsStore() {
	const { subscribe, update } = writable<{
		currentPost: Post;
		comments: Comment[];
	}>({
		currentPost: null,
		comments: []
	});

	return {
		subscribe,
		fetchComments: async (postId: string) => {
			const comments = await agent.Comments.list(postId);
			update((v) => ({ ...v, comments }));
		},
		likeComment: async (postId: string, commentId: string) => {
			await agent.Comments.like(postId, commentId);
			const comments = await agent.Comments.list(postId);
			update((v) => ({ ...v, comments }));
		},
		createComment: async (postId: string, comment: CreateComment) => {
			await agent.Comments.create(postId, comment);
			const comments = await agent.Comments.list(postId);
			update((v) => ({ ...v, comments }));
		},
		deleteComment: async (postId: string, commentId: string) => {
			await agent.Comments.delete(postId, commentId);
			const comments = await agent.Comments.list(postId);
			update((v) => ({ ...v, comments }));
		},
		fetchLikesForComment: async (postId: string, commentId: string) => {
			await agent.Comments.listLikes(postId, commentId);
			const comments = await agent.Comments.list(postId);
			update((v) => ({ ...v, comments }));
		}
	};
}

export const commentsStore = createCommentsStore();
