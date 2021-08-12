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
				<div
					class="mb-5 rounded overflow-hidden border w-full lg:w-6/12 md:w-6/12 bg-white mx-3 md:mx-0 lg:mx-0"
				>
					<div class="w-full flex justify-between p-3">
						<div class="flex">
							<a href="/profiles/{post.userId}" class="avatar">
								<div
									class="rounded-full w-10 h-10 ring ring-primary ring-offset-base-100 ring-offset-2"
								>
									<img
										alt="avatar"
										src="https://www.pngitem.com/pimgs/m/30-307416_profile-icon-png-image-free-download-searchpng-employee.png"
									/>
								</div>
							</a>
							<span class="pt-1 ml-2 font-bold text-sm">{post.username}</span>
						</div>
					</div>
					<img
						class="w-full bg-cover"
						src={`http://localhost:5000${post.url}`}
						alt={post.caption}
					/>
					<div class="px-3 pb-2 flex flex-col">
						<div class="pt-2">
							<i class="far fa-heart cursor-pointer" />
							<span class="text-sm font-medium">{post.likes} likes</span>
						</div>

						<div class="pt-1">
							<div class="mb-2 text-sm">
								<span class="font-medium mr-2">
									<a href="/profiles/{post.userId}">
										{post.username}
									</a>
								</span>
								{post.caption}
							</div>
						</div>
						<div class="self-end">
							{#if isLoggedIn}
								<button class="btn btn-primary btn-sm" on:click={() => postsStore.likePost(post.id)}
									>Like</button
								>
							{/if}

							{#if $accountsStore.user?.id === post.userId}
								<button
									class="btn btn-secondary btn-sm mb-3"
									on:click={() => postsStore.deletePost(post.id)}>Delete</button
								>
							{/if}
						</div>
						<a
							href="/posts/{post.id}"
							class="text-sm mb-2 text-gray-400 cursor-pointer font-medium"
						>
							View all {post.comments} comments
						</a>
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
