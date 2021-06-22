<script lang="ts">
	import { accountsStore, commentsStore } from '$lib/stores';
	import { onMount } from 'svelte';
	import CommentLikes from '$lib/components/CommentList/CommentLikes.svelte';

	export let postId: string;

	onMount(async () => {
		await commentsStore.fetchComments(postId);
	});

	$: comments = $commentsStore.comments;
	$: currentUser = $accountsStore.user;
	$: isLoggedIn = !!$accountsStore.user;
</script>

<h2 class="mb-5">Comment List</h2>

<div>
	{#each comments as comment}
		<div class="card mt-3">
			<div class="card-body">
				{comment.contents}
			</div>

			<CommentLikes {postId} commentId={comment.id} />
			<div class="d-flex">
				{#if isLoggedIn}
					<button
						class="btn btn-primary"
						on:click={async () => await commentsStore.likeComment(postId, comment.id)}
					>
						Like
					</button>
				{/if}

				{#if currentUser?.id === comment.userId}
					<button
						class="btn btn-danger"
						on:click={async () => await commentsStore.deleteComment(postId, comment.id)}
					>
						Delete
					</button>
				{/if}
			</div>
		</div>
	{/each}
</div>
