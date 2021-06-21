<script lang="ts">
	import agent from '$lib/api/agent';

	export let postId: string;
	async function fetchComments() {
		return await agent.Comments.list(postId);
	}
</script>

<h2 class="mb-5">Comment List</h2>

{#await fetchComments()}
	<div>Fetching comments</div>
{:then comments}
	<div>
		{#each comments as comment}
			<div class="card mt-3">
				<div class="card-body">
					{comment.contents}
				</div>
			</div>
		{/each}
	</div>
{/await}
