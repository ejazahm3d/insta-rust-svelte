<script lang="ts">
	import agent from '$lib/api/agent';
	import { writable } from 'svelte/store';
	import { goto } from '$app/navigation';
	import { accountsStore } from '$lib/stores/index';

	const user = writable({
		email: '',
		password: ''
	});

	let loading: boolean = false;

	async function login() {
		loading = true;
		try {
			const data = await agent.Account.login($user);

			accountsStore.set({ user: data.id });
			loading = false;

			await goto('/');
		} catch (error) {
			loading = false;
			console.log(error.response.data);
		}
	}
</script>

<div class="container">
	<h1 class="text-center mt-5">Login page</h1>

	<form on:submit|preventDefault={login}>
		<div class="mb-3">
			<label class="form-label" for="email">Email</label>
			<input class="form-control" name="email" type="email" bind:value={$user.email} required />
		</div>

		<div class="mb-3">
			<label class="form-label" for="password">Password</label>
			<input
				class="form-control"
				name="password"
				type="password"
				bind:value={$user.password}
				required
			/>
		</div>

		<button type="submit" class={`btn btn-primary `} disabled={loading}>
			{#if loading}
				<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
			{/if}
			Submit</button
		>
	</form>
</div>
