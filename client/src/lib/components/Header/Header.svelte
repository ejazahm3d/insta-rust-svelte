<script lang="ts">
	import { accountsStore } from '$lib/stores';

	export let isLoggedIn: boolean;
	export let userId: string;

	const logout = async () => {
		await accountsStore.logout();
	};
</script>

<nav class="navbar mb-2 shadow-lg bg-neutral text-neutral-content rounded-box">
	<div class="flex-none px-2 mx-2">
		<a href="/" class="text-lg font-bold"> Insta </a>
	</div>
	<div class="flex-1 px-2 mx-2">
		<ul class="items-stretch ">
			<li class="btn btn-ghost btn-sm rounded-btn">
				<a href="/">Home </a>
			</li>

			{#if !isLoggedIn}
				<li class="btn btn-ghost btn-sm rounded-btn">
					<a href="/auth/signup">Sign Up</a>
				</li>
				<li class="btn btn-ghost btn-sm rounded-btn">
					<a href="/auth/login">Login</a>
				</li>
			{/if}
		</ul>
	</div>
	{#if isLoggedIn}
		<li class="nav-item">
			<button class="btn btn-error btn-sm rounded-btn " on:click={logout}>Logout</button>
		</li>
		<div class="flex-none">
			<a href={`/profiles/${userId}`}>
				<div class="avatar">
					<div class="rounded-full w-10 h-10 m-1">
						<img
							src="https://www.pngitem.com/pimgs/m/30-307416_profile-icon-png-image-free-download-searchpng-employee.png"
							alt="avatar"
						/>
					</div>
				</div>
			</a>
		</div>
		<li class="nav-item" />
	{/if}
</nav>
