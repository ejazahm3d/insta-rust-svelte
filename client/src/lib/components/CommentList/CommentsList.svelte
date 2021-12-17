<script lang="ts">
	import { accountsStore } from '$lib/auth';
	import { onMount } from 'svelte';
	import CreateComment from './CreateComment.svelte';
	import { store } from '$lib/store';
	import { commentList } from '$lib/store/services/comments';
	import CommentItem from './CommentItem.svelte';

	export let postId: string;

	onMount(async () => {
		store.dispatch(commentList.initiate(postId));
	});

	$: ({ data: comments } = commentList.select(postId)($store));
	$: isLoggedIn = !!$accountsStore.user;
</script>

<div class="w-full">
	{#if isLoggedIn}
		<div class="mx-4">
			<CreateComment {postId} />
		</div>
	{/if}

	{#if comments}
		{#each comments as comment}
			<CommentItem {comment} />
		{/each}
	{/if}
</div>
