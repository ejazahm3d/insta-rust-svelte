<script>
	import Header from '$lib/components/Header/Header.svelte';
	import agent from '$lib/api/agent';
	import { accountsStore } from '$lib/stores/index';
	import { onMount } from 'svelte';
	import '../styles/global.css';

	onMount(async () => {
		const data = await agent.Account.current();
		accountsStore.set({ user: data?.user });
	});

	$: isLoggedIn = !!$accountsStore?.user;
</script>

<Header {isLoggedIn} />

<main>
	<slot />
</main>
