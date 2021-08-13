<script lang="ts">
	import { page } from '$app/stores';

	import { accountsStore, profilesStore } from '$lib/stores/index';

	import { onMount } from 'svelte';

	$: userId = $page.params.profileId;
	$: profile = $profilesStore.profile;
	$: posts = $profilesStore.posts;
	$: followers = $profilesStore.followers;
	$: following = $profilesStore.following;
	$: isFollowing = $profilesStore.isFollowing;
	$: currentUser = $accountsStore.user;

	onMount(async () => {
		await profilesStore.details(userId);
		await profilesStore.posts(userId);
		await profilesStore.isFollowing(userId);
		await profilesStore.followers(userId);
		await profilesStore.following(userId);
	});

	async function followOrUnfollow() {
		await profilesStore.followOrUnfollow(userId);
	}
</script>

{#if profile}
	<div class="bg-gray-100 bg-opacity-25">
		<div class="lg:w-8/12 lg:mx-auto mb-8">
			<header class="flex flex-wrap items-center p-4 md:py-8">
				<div class="md:w-3/12 md:ml-16">
					<!-- profile image -->
					<img
						class="w-20 h-20 md:w-40 md:h-40 object-cover rounded-full
					   border-2 border-primary p-1"
						src="https://images.unsplash.com/photo-1502791451862-7bd8c1df43a7?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=700&q=80"
						alt="profile"
					/>
				</div>

				<!-- profile meta -->
				<div class="w-8/12 md:w-7/12 ml-4">
					<div class="md:flex md:flex-wrap md:items-center mb-4">
						<h2 class="text-3xl inline-block font-light md:mr-2 mb-2 sm:mb-0">
							{profile.username}
						</h2>

						<!-- badge -->
						<span
							class="inline-block fas fa-certificate fa-lg text-blue-500 
								 relative mr-6 text-xl transform -translate-y-2"
							aria-hidden="true"
						>
							<i
								class="fas fa-check text-white text-xs absolute inset-x-0
								 ml-1 mt-px"
							/>
						</span>

						<!-- follow button -->
						{#if currentUser.id !== userId && !isFollowing}
							<button on:click={followOrUnfollow} class="btn btn-primary btn-sm">Follow</button>
						{/if}
						{#if currentUser.id !== userId && isFollowing}
							<button on:click={followOrUnfollow} class="btn btn-primary btn-sm">unfollow</button>
						{/if}
					</div>

					<!-- post, following, followers list for medium screens -->
					<ul class="hidden md:flex space-x-8 mb-4">
						<li>
							<span class="font-semibold">{posts.length}</span>
							posts
						</li>

						<li>
							<span class="font-semibold">{followers}</span>
							followers
						</li>
						<li>
							<span class="font-semibold">{following}</span>
							following
						</li>
					</ul>

					<!-- user meta form medium screens -->
					<div class="hidden md:block">
						<p>{profile.bio ?? ''}</p>
					</div>
				</div>

				<!-- user meta form small screens -->
				<div class="md:hidden text-sm my-2">
					<p>{profile.bio ?? ''}</p>
				</div>
			</header>

			<!-- posts -->
			<div class="px-px md:px-3">
				<!-- user following for mobile only -->
				<ul
					class="flex md:hidden justify-around space-x-8 border-t 
			text-center p-2 text-gray-600 leading-snug text-sm"
				>
					<li>
						<span class="font-semibold text-gray-800 block">{posts.length}</span>
						posts
					</li>

					<li>
						<span class="font-semibold text-gray-800 block">{followers}</span>
						followers
					</li>
					<li>
						<span class="font-semibold text-gray-800 block">{following}</span>
						following
					</li>
				</ul>

				<!-- insta freatures -->
				<ul
					class="flex items-center justify-around md:justify-center space-x-12  
		uppercase tracking-widest font-semibold text-xs text-gray-600
		border-t"
				>
					<!-- posts tab is active -->
					<li class="md:border-t md:border-gray-700 md:-mt-px md:text-gray-700">
						<a class="inline-block p-3" href="#post">
							<i class="fas fa-th-large text-xl md:text-xs" />
							<span class="hidden md:inline">post</span>
						</a>
					</li>
				</ul>

				{#if posts.length > 0}
					<!-- flexbox grid -->
					<div class="flex flex-wrap -mx-px md:-mx-3">
						{#each posts as post}
							<!-- column -->
							<div class="w-1/3 p-px md:px-3">
								<!-- post 1-->
								<a href={`/posts/${post.id}`}>
									<article class="post bg-gray-100 text-white relative pb-full md:mb-6">
										<!-- post image-->
										<img
											class="w-full h-full absolute left-0 top-0 object-cover"
											src={`http://localhost:5000${post.url}`}
											alt="post"
										/>

										<i class="fas fa-square absolute right-0 top-0 m-1" />
										<!-- overlay-->
										<div
											class="overlay bg-gray-800 bg-opacity-25 w-full h-full absolute 
							left-0 top-0 hidden"
										>
											<div
												class="flex justify-center items-center 
							space-x-4 h-full"
											>
												<span class="p-2">
													<i class="fas fa-heart" />
													{post.likes}
												</span>

												<span class="p-2">
													<i class="fas fa-comment" />
													{post.comments}
												</span>
											</div>
										</div>
									</article>
								</a>
							</div>
						{/each}
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}

<style>
	.pb-full {
		padding-bottom: 100%;
	}

	@media screen and (min-width: 768px) {
		.post:hover .overlay {
			display: block;
		}
	}
</style>
