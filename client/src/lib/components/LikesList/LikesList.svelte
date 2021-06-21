<script lang="ts">
	import agent from '$lib/api/agent';

	export let postId: string;

	async function fetchPostLikes() {
		return await agent.Posts.listLikes(postId);
	}
</script>

<h2>Likes List</h2>

{#await fetchPostLikes()}
	<div>Fetching comments</div>
{:then likes}
	<div>
		{#each likes as like}
			<div class="card mt-3">
				<div class="card-body">
					Liked By: {like.userId}
				</div>
			</div>
		{/each}
	</div>
{/await}
