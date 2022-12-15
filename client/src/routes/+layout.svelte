<script>
	import '../app.postcss';
	import Header from '$lib/components/Header/Header.svelte';
	import { accountsStore } from '$lib/auth/index';
	import { onMount } from 'svelte';

	onMount(async () => {
		accountsStore.fetchCurrentUser();
	});

	$: isLoggedIn = !!$accountsStore?.user;
</script>

<Header {isLoggedIn} userId={$accountsStore.user?.id} />
<main>
	<slot />
</main>
