<script lang="ts">
	import agent from '$lib/api/agent';
	import { postsStore } from '$lib/stores';

	async function fetchPosts() {
		const posts = await agent.Posts.list();
		postsStore.set({ posts });
		return posts;
	}

	$: posts = $postsStore.posts;
</script>

<div class="container">
	<h1 class="text-center mt-5 mb-2">Welcome to SvelteKit</h1>

	{#await fetchPosts()}
		<p>Fetching posts</p>
	{:then _}
		<div class="d-flex flex-column align-items-center">
			{#each posts as post}
				<div class="card m-5">
					<img class="card-img-top" src={post.url} alt={post.caption} />
					<div class="card-body">
						<p class="card-text">
							{post.caption}
						</p>
						<p>Created by: {post.username}</p>
						<p>Likes: {post.likes}</p>
						<p>Comments: {post.comments}</p>
						<a class="btn btn-primary mb-3" href={`/posts/${post.id}`}>Details</a>
					</div>
				</div>
			{/each}
		</div>
	{:catch error}
		<div>{error.response.data}</div>
	{/await}
</div>
