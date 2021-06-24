<script>
	import { page } from '$app/stores';
	import agent from '$lib/api/agent';
	import CommentsList from '$lib/components/CommentList/CommentsList.svelte';
	import CreateComment from '$lib/components/CreateComment/CreateComment.svelte';
	import LikesList from '$lib/components/LikesList/LikesList.svelte';
	import { accountsStore } from '$lib/stores';

	let postId = $page.params.postId;

	function fetchPost() {
		return agent.Posts.details(postId);
	}

	$: isLoggedIn = $accountsStore.user;
</script>

<div class="container d-flex flex-column">
	{#await fetchPost()}
		<div>Fetching single post</div>
	{:then post}
		<div>
			{post.caption}
		</div>
		<img src={`http://localhost:5000${post.url}`} alt={post.url} />
		<LikesList postId={post.id} />

		{#if isLoggedIn}
			<CreateComment postId={post.id} />
		{/if}
		<CommentsList postId={post.id} />
	{:catch}
		<div>Error loading post</div>
	{/await}
</div>

<style>
	img {
		max-width: 40rem;
	}
</style>
