<script lang="ts">
	import agent from '$lib/api/agent';

	import { onMount } from 'svelte';

	let profile;

	onMount(async () => {
		const data = await agent.Profiles.details();
		profile = data;
	});
</script>

<!-- <h1>this is dashboard {profile.id}</h1> -->
