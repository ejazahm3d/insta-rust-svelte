<script>
	import { page } from '$app/stores';
	import agent from '$lib/api/agent';
	import CommentsList from '$lib/components/CommentList/CommentsList.svelte';
	import LikesList from '$lib/components/LikesList/LikesList.svelte';

	let postId = $page.params.postId;

	function fetchPost() {
		return agent.Posts.details(postId);
	}
</script>

<h1>
	Welcome {postId}
</h1>

{#await fetchPost()}
	<div>Fetching single post</div>
{:then post}
	<div>
		{post.caption}
	</div>
	<img src={post.url} alt={post.url} />
	<LikesList postId={post.id} />
	<CommentsList postId={post.id} />
{:catch}
	<div>Error loading post</div>
{/await}
