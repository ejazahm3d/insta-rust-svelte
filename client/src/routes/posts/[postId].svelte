<script>
	import { page } from '$app/stores';
	import agent from '$lib/api/agent';
	import CommentsList from '$lib/components/CommentList/CommentsList.svelte';
	import CreateComment from '$lib/components/CreateComment/CreateComment.svelte';
	import LikesList from '$lib/components/LikesList/LikesList.svelte';
	import { accountsStore } from '$lib/stores';
	import { onMount } from 'svelte';

	let postId = $page.params.postId;

	function fetchPost() {
		return agent.Posts.details(postId);
	}

	onMount(() => {
		console.log('hello' + postId);
	});

	$: isLoggedIn = $accountsStore.user;
</script>

<div class="container d-flex flex-column">
	{#await fetchPost()}
		<div>Fetching single post</div>
	{:then post}
		<div>
			{post.caption}
		</div>
		<img src={post.url} alt={post.url} />
		<LikesList postId={post.id} />

		{#if isLoggedIn}
			<CreateComment postId={post.id} />
		{/if}
		<CommentsList postId={post.id} />
	{:catch}
		<div>Error loading post</div>
	{/await}
</div>
