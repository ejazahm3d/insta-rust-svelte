<script lang="ts">
	import { accountsStore } from '$lib/stores';

	export let isLoggedIn: boolean;
	export let userId: string;

	const logout = async () => {
		await accountsStore.logout();
	};
</script>

<nav class="border-b px-4 py-2 bg-white">
	<div class="flex flex-wrap items-center justify-between md:justify-around">
		<!-- logo -->
		<a href="/">
			<img
				class="h-10"
				src="https://upload.wikimedia.org/wikipedia/commons/thumb/2/2a/Instagram_logo.svg/150px-Instagram_logo.svg.png"
				alt="instagram"
			/>
		</a>

		<!-- search-->
		<div class="relative hidden sm:block text-gray-500">
			<input
				class="search-bar max-w-xs border rounded bg-gray-200 px-4
			  text-center outline-none focus:border-gray-400"
				type="search"
				placeholder="Search"
			/>
			<i class="fa fa-search absolute top-0 left-0 ml-12 mt-1" />
		</div>

		<div class="space-x-4">
			{#if !isLoggedIn}
				<a href="/auth/login" class="btn btn-primary btn-sm">Log In</a>
				<a href="/auth/signup" class="btn btn-primary btn-sm btn-outline">Sign Up</a>
			{/if}
			{#if isLoggedIn}
				<div class="flex items-center">
					<div class="dropdown dropdown-end">
						<div tabindex="0" class="dropdown dropdown-end">
							<div class="avatar">
								<div class="rounded-full w-10 h-10 m-1">
									<img
										src="https://www.pngitem.com/pimgs/m/30-307416_profile-icon-png-image-free-download-searchpng-employee.png"
										alt="avatar"
									/>
								</div>
							</div>
							<!-- <a href={`/profiles/${userId}`}>
						</a> -->
							<ul
								tabindex="0"
								class="p-2 shadow menu dropdown-content bg-neutral text-neutral-content rounded-box w-52"
							>
								<li>
									<a href="/profiles/{userId}">Profile</a>
								</li>
								<li>
									<button class="btn btn-secondary btn-sm " on:click={logout}>Logout</button>
								</li>
							</ul>
						</div>
					</div>
				</div>
			{/if}
		</div>
	</div>
</nav>
