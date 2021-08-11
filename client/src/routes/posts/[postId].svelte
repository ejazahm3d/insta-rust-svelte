<script>
	import { page } from '$app/stores';
	import agent from '$lib/api';
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

<div class="flex flex-col md:flex-row ">
	{#await fetchPost()}
		<div>Fetching single post</div>
	{:then post}
		<div class="card p-10 bg-base-200 mt-10 md:mt-0">
			<figure>
				<img class="rounded-xl" src={`http://localhost:5000${post.url}`} alt={post.url} />
			</figure>
			<div class="card-body">
				<div class="card-title mt-5">
					{post.caption}
				</div>

				<LikesList postId={post.id} />
			</div>
		</div>
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
