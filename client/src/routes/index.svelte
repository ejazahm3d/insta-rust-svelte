<script lang="ts">
	import agent from '$lib/api/agent';

	async function fetchPosts() {
		return await agent.Posts.list();
	}
</script>

<h1>Welcome to SvelteKit</h1>

{#await fetchPosts()}
	<p>Fetching posts</p>
{:then posts}
	<div>
		{#each posts as post}
			<div>
				{post.id}
				<p>
					{post.caption}
				</p>
				<img src={post.url} alt={post.caption} />
			</div>
		{/each}
	</div>
{:catch error}
	<div>{error.response.data}</div>
{/await}
