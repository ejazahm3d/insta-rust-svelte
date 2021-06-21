<script lang="ts">
	import agent from '$lib/api/agent';

	async function fetchPosts() {
		return await agent.Posts.list();
	}
</script>

<div class="container">
	<h1 class="text-center mt-5 mb-2">Welcome to SvelteKit</h1>

	{#await fetchPosts()}
		<p>Fetching posts</p>
	{:then posts}
		<div class="d-flex flex-column align-items-center">
			{#each posts as post}
				<div class="card m-5">
					<img class="card-img-top" src={post.url} alt={post.caption} />
					<div class="card-body">
						<p class="card-text">
							{post.caption}
						</p>
						<a class="btn btn-primary" href={`/posts/${post.id}`}>Details</a>
						<p>Likes: {post.likes}</p>
						<p>Comments: {post.comments}</p>
					</div>
				</div>
			{/each}
		</div>
	{:catch error}
		<div>{error.response.data}</div>
	{/await}
</div>
