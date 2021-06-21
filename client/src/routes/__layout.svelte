<script>
	import Header from '$lib/components/Header/Header.svelte';
	import { accountsStore } from '$lib/stores/index';
	import { onMount } from 'svelte';
	import '../styles/global.css';

	onMount(async () => {
		accountsStore.fetchCurrentUser();
	});

	$: isLoggedIn = !!$accountsStore?.user;
</script>

<Header {isLoggedIn} />

<main>
	<slot />
</main>
