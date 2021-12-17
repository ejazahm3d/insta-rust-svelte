<script lang="ts">
	import type { Post } from '$lib/types';
	import PostItem from './PostItem.svelte';

	export let posts: Post[];
	export let isLoggedIn: boolean;
</script>

<div class="flex flex-col items-center">
	{#each posts as post}
		<PostItem {isLoggedIn} {post} />
	{/each}
</div>
