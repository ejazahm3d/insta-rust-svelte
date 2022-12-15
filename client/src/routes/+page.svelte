<script lang="ts">
	import CreatePost from '$lib/components/CreatePost/CreatePost.svelte';
	import { accountsStore } from '$lib/auth';
	import { postList } from '$lib/store/services/posts';
	import { onMount } from 'svelte';
	import { store } from '$lib/store';
	import PostList from '$lib/components/PostList/PostList.svelte';

	$: ({ data: posts, isError, error } = postList.select()($store));
	$: isLoggedIn = !!$accountsStore.user;

	onMount(() => {
		store.dispatch(postList.initiate());
	});
</script>

<div>
	<h1 class="text-center mt-10 md:mt-20 mb-5 mx-5 text-4xl font-semibold">Welcome to InstaClone</h1>

	{#if isLoggedIn}
		<div class="flex m-5 flex-col items-center bg-base-200 max-w-lg mx-auto p-10 card">
			<CreatePost />
		</div>
	{/if}

	{#if isError}
		<div>{JSON.stringify(error)}</div>
	{/if}

	{#if posts}
		<PostList {isLoggedIn} {posts} />
	{/if}
</div>
