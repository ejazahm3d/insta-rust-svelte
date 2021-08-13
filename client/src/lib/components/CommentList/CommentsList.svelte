<script lang="ts">
	import { accountsStore, commentsStore } from '$lib/stores';
	import { onMount } from 'svelte';
	import CreateComment from '../CreateComment/CreateComment.svelte';
	import agent from '$lib/api/index';
	import { formatDistance } from 'date-fns';

	export let postId: string;

	onMount(async () => {
		await commentsStore.fetchComments(postId);
	});

	$: comments = $commentsStore.comments;
	$: currentUser = $accountsStore.user;
	$: isLoggedIn = !!$accountsStore.user;
</script>

<div class="w-full">
	{#if isLoggedIn}
		<div class="mx-4">
			<CreateComment {postId} />
		</div>
	{/if}
	{#each comments as comment}
		<div class="flex mt-5 mx-4">
			<div class="avatar mx-2 mt-2">
				<div class="rounded-full w-10 h-10 ring ring-primary ring-offset-base-100 ring-offset-2">
					<img
						src="https://www.pngitem.com/pimgs/m/30-307416_profile-icon-png-image-free-download-searchpng-employee.png"
						alt=""
					/>
				</div>
			</div>
			<div class="flex-1 border rounded-lg px-4 py-2 sm:px-6 sm:py-4 leading-relaxed">
				<strong>{comment.username}</strong>
				<span class="text-xs text-gray-400"
					>{formatDistance(new Date(comment.createdAt), new Date(), { addSuffix: true })}</span
				>
				<p class="text-sm">
					{comment.contents}
				</p>

				<div class="text-sm text-gray-500 font-semibold">{comment.likes} likes</div>
				<div class="flex justify-end">
					{#if isLoggedIn}
						{#await agent.Comments.hasLiked(postId, comment.id) then hasLiked}
							{#if hasLiked}
								<button
									class="btn btn-primary btn-sm mr-2"
									on:click={async () => await commentsStore.likeComment(postId, comment.id)}
								>
									Dislike
								</button>
							{:else}
								<button
									class="btn btn-primary btn-sm mr-2"
									on:click={async () => await commentsStore.likeComment(postId, comment.id)}
								>
									Like
								</button>
							{/if}
						{/await}
					{/if}

					{#if currentUser?.id === comment.userId}
						<button
							class="btn btn-secondary btn-sm"
							on:click={async () => await commentsStore.deleteComment(postId, comment.id)}
						>
							Delete
						</button>
					{/if}
				</div>
			</div>
		</div>
	{/each}
</div>
