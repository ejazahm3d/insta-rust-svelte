<script lang="ts">
	import { store } from '$lib/store';
	import { hasLikedPost, postDelete, postLike } from '$lib/store/services/posts';
	import { accountsStore } from '$lib/stores';

	import type { Post } from '$lib/types';
	import { onMount } from 'svelte';

	export let post: Post;
	export let isLoggedIn: boolean;

	$: ({ data: hasLiked, isError, isLoading } = hasLikedPost.select(post.id)($store));

	onMount(() => {
		store.dispatch(hasLikedPost.initiate(post.id));
	});
</script>

<div class="mb-5 rounded overflow-hidden border w-full lg:w-6/12 md:w-6/12 mx-3 md:mx-0 lg:mx-0">
	<div class="w-full flex justify-between p-3">
		<div class="flex">
			<a href="/profiles/{post.userId}" class="avatar">
				<div class="rounded-full w-10 h-10 ring ring-primary ring-offset-base-100 ring-offset-2">
					<img
						alt="avatar"
						src="https://www.pngitem.com/pimgs/m/30-307416_profile-icon-png-image-free-download-searchpng-employee.png"
					/>
				</div>
			</a>
			<span class="pt-1 ml-2 font-bold text-sm">{post.username}</span>
		</div>
	</div>
	<img class="w-full bg-cover" src={`http://localhost:5000${post.url}`} alt={post.caption} />
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
		<div class="self-end h-8">
			{#if isLoggedIn}
				{#if hasLiked && !isLoading && !isError}
					<button
						class="btn btn-primary btn-sm"
						on:click={() => store.dispatch(postLike.initiate(post.id))}>Dislike</button
					>
				{:else}
					<button
						class="btn btn-primary btn-sm"
						on:click={() => store.dispatch(postLike.initiate(post.id))}>Like</button
					>
				{/if}
			{/if}

			{#if $accountsStore.user?.id === post.userId}
				<button
					class="btn btn-secondary btn-sm mb-3"
					on:click={() => store.dispatch(postDelete.initiate(post.id))}>Delete</button
				>
			{/if}
		</div>
		<a href="/posts/{post.id}" class="text-sm mb-2 text-gray-400 cursor-pointer font-medium">
			View all {post.comments} comments
		</a>
	</div>
</div>
