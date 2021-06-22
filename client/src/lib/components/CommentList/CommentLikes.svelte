<script lang="ts">
	import agent from '$lib/api/agent';

	export let postId: string;
	export let commentId: string;
</script>

{#await agent.Comments.listLikes(postId, commentId)}
	<div>Fetching</div>
{:then comments}
	<div>likes: {comments.length}</div>
{:catch}
	<div>Somehting wenrt wrong</div>
{/await}
