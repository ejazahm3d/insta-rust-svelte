<script lang="ts">
	import CreatePost from '$lib/components/CreatePost/CreatePost.svelte';

	import { postsStore, accountsStore } from '$lib/stores';

	$: posts = $postsStore.posts;
	$: isLoggedIn = $accountsStore.user;
</script>

<div>
	<h1 class="text-center mt-10 md:mt-20 mb-5 mx-5 text-4xl font-semibold">Welcome to InstaClone</h1>

	{#if isLoggedIn}
		<div class="flex m-5 flex-col items-center bg-base-200 max-w-lg mx-auto p-10 card">
			<CreatePost />
		</div>
	{/if}

	{#await postsStore.fetchPosts()}
		<p>Fetching posts</p>
	{:then _}
		<div class="flex flex-col items-center">
			{#each posts as post}
				<div class="card m-5 bordered bg-base-200">
					<figure class="px-10 pt-10">
						<img class="rounded-xl" src={`http://localhost:5000${post.url}`} alt={post.caption} />
					</figure>
					<div class="card-body">
						<p class="card-title">
							{post.caption}
						</p>
						<p>Created by: {post.username}</p>
						<p>Likes: {post.likes}</p>
						<p>Comments: {post.comments}</p>

						<div class="justify-end card-actions">
							<a class="btn btn-primary btn-sm mb-3" href={`/posts/${post.id}`}>Details</a>

							{#if isLoggedIn}
								<button
									class="btn btn-primary btn-sm mb-3"
									on:click={() => postsStore.likePost(post.id)}>Like</button
								>
							{/if}

							{#if $accountsStore.user?.id === post.userId}
								<button
									class="btn btn-secondary btn-sm mb-3"
									on:click={() => postsStore.deletePost(post.id)}>Delete</button
								>
							{/if}
						</div>
					</div>
				</div>
			{/each}
		</div>
	{:catch error}
		<div>{error.response.data}</div>
	{/await}
</div>

<style>
	img {
		max-width: 45rem;
	}
</style>
