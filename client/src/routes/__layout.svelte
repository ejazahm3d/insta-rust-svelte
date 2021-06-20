<script>
	import agent from '$lib/api/agent';
	import { accountsStore } from '$lib/stores/index';
	import { onDestroy, onMount } from 'svelte';

	onMount(async () => {
		const data = await agent.Account.current();
		accountsStore.set({ user: data?.user });
	});

	$: isLoggedIn = !!$accountsStore?.user;
</script>

<ul>
	<li>
		<a href="/"> Home </a>
	</li>

	{#if isLoggedIn}
		<li>
			<button
				on:click={() => {
					agent.Account.logout();
					accountsStore.set({ user: null });
				}}
			>
				Logout
			</button>
		</li>
	{:else}
		<li>
			<a href="/auth/login"> Login </a>
		</li>

		<li>
			<a href="/auth/signup"> Signup </a>
		</li>
	{/if}
</ul>
<main>
	<slot />
</main>
