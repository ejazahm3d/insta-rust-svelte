<script lang="ts">
	import { accountsStore, commentsStore } from '$lib/stores';
	import { onMount } from 'svelte';
	import CreateComment from '../CreateComment/CreateComment.svelte';

	export let postId: string;

	onMount(async () => {
		await commentsStore.fetchComments(postId);
	});

	$: comments = $commentsStore.comments;
	$: currentUser = $accountsStore.user;
	$: isLoggedIn = !!$accountsStore.user;
</script>

<div class="w-full bg-base-300">
	{#if isLoggedIn}
		<div class="mx-4">
			<CreateComment {postId} />
		</div>
	{/if}
	{#each comments as comment}
		<div class="card mx-4 my-5 bg-base-100 rounded-box">
			<div class="p-5">
				<div class="card-title">
					{comment.contents}
				</div>

				<div class="my-3">likes: {comment.likes}</div>

				<div class="flex justify-end">
					{#if isLoggedIn}
						<button
							class="btn btn-primary btn-sm mr-2"
							on:click={async () => await commentsStore.likeComment(postId, comment.id)}
						>
							Like
						</button>
					{/if}

					{#if currentUser?.id === comment.userId}
						<button
							class="btn btn-secondary btn-sm"
							on:click={async () => await commentsStore.deleteComment(postId, comment.id)}
						>
							Delete
						</button>
					{/if}
				</div>
			</div>
		</div>
	{/each}
</div>
