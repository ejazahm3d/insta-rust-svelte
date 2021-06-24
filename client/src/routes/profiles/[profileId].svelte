<script lang="ts">
	import { page } from '$app/stores';

	import { profilesStore } from '$lib/stores';

	import { onMount } from 'svelte';

	onMount(async () => {
		let userId = $page.params.profileId;
		await profilesStore.details(userId);
	});

	$: profile = $profilesStore.profile;
</script>

{#if profile?.id}
	<h1>this is profile {profile?.id}</h1>
{/if}
