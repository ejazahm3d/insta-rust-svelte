<script lang="ts">
	import CreatePost from '$lib/components/CreatePost/CreatePost.svelte';

	import { postsStore, accountsStore } from '$lib/stores';

	$: posts = $postsStore.posts;
	$: isLoggedIn = $accountsStore.user;
</script>

<div class="container">
	<h1 class="text-center mt-5 mb-2">Welcome to SvelteKit</h1>

	{#if isLoggedIn}
		<div class="d-flex flex-column align-items-center">
			<CreatePost />
		</div>
	{/if}

	{#await postsStore.fetchPosts()}
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

						{#if isLoggedIn}
							<button class="btn btn-primary mb-3" on:click={() => postsStore.likePost(post.id)}
								>Like</button
							>
						{/if}

						{#if $accountsStore.user?.id === post.userId}
							<button class="btn btn-danger mb-3" on:click={() => postsStore.deletePost(post.id)}
								>Delete</button
							>
						{/if}
					</div>
				</div>
			{/each}
		</div>
	{:catch error}
		<div>{error.response.data}</div>
	{/await}
</div>
