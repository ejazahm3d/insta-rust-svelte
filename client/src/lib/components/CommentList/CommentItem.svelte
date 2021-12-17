<script lang="ts">
	import type { Comment } from '$lib/types';
	import { formatDistance } from 'date-fns';
	import { accountsStore } from '$lib/auth';
	import { onMount } from 'svelte';
	import { store } from '$lib/store';
	import { commentDelete, commentLike, hasLikedComment } from '$lib/store/services/comments';

	export let comment: Comment;

	onMount(() => {
		store.dispatch(hasLikedComment.initiate({ postId: comment.postId, commentId: comment.id }));
	});

	$: ({
		data: hasLiked,
		isError,
		isLoading
	} = hasLikedComment.select({
		postId: comment.postId,
		commentId: comment.id
	})($store));

	$: currentUser = $accountsStore.user;
	$: isLoggedIn = !!$accountsStore.user;
	console.log(hasLiked);
</script>

<div class="flex mt-5 mx-4">
	<a href="/profiles/{comment.userId}" class="avatar mx-2 mt-2">
		<div class="rounded-full w-10 h-10 ring ring-primary ring-offset-base-100 ring-offset-2">
			<img
				src="https://www.pngitem.com/pimgs/m/30-307416_profile-icon-png-image-free-download-searchpng-employee.png"
				alt=""
			/>
		</div>
	</a>
	<div class="flex-1 border rounded-lg px-4 py-2 sm:px-6 sm:py-4 leading-relaxed">
		<strong>{comment.username}</strong>
		<span class="text-xs text-gray-400"
			>{formatDistance(new Date(comment.createdAt), new Date(), { addSuffix: true })}</span
		>
		<p class="text-sm">
			{comment.contents}
		</p>

		<div class="text-sm text-gray-500 font-semibold">{comment.likes} likes</div>
		<div class="flex justify-end">
			{#if isLoggedIn}
				{#if hasLiked && !isLoading && !isError}
					<button
						class="btn btn-primary btn-sm mr-2"
						on:click={() =>
							store.dispatch(
								commentLike.initiate({ postId: comment.postId, commentId: comment.id })
							)}
					>
						Dislike
					</button>
				{:else}
					<button
						class="btn btn-primary btn-sm mr-2"
						on:click={async () =>
							store.dispatch(
								commentLike.initiate({ postId: comment.postId, commentId: comment.id })
							)}
					>
						Like
					</button>
				{/if}
			{/if}

			{#if currentUser?.id === comment.userId}
				<button
					class="btn btn-secondary btn-sm"
					on:click={() =>
						store.dispatch(
							commentDelete.initiate({ postId: comment.postId, commentId: comment.id })
						)}
				>
					Delete
				</button>
			{/if}
		</div>
	</div>
</div>
