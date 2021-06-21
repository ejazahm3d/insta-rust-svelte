<script lang="ts">
	import { goto } from '$app/navigation';

	import agent from '$lib/api/agent';
	import { writable } from 'svelte/store';

	const user = writable({
		email: '',
		username: '',
		password: ''
	});

	let loading = false;

	async function signup() {
		loading = true;
		try {
			await agent.Account.signup($user);
			loading = false;
			await goto('/');
		} catch (error) {
			loading = false;
			console.log(error.response.data);
		}
	}
</script>

<div class="container">
	<h1 class="text-center mt-5">Signup page</h1>

	<form on:submit|preventDefault={signup}>
		<div class="mb-3">
			<label class="form-label" for="email">Email</label>
			<input class="form-control" name="email" type="email" bind:value={$user.email} required />
		</div>

		<div class="mb-3">
			<label class="form-label" for="username">Username</label>
			<input
				class="form-control"
				name="username"
				type="text"
				bind:value={$user.username}
				required
			/>
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

		<button class="btn btn-primary" type="submit" disabled={loading}>
			{#if loading}
				<span class="spinner-border spinner-border-sm" role="status" aria-hidden="true" />
			{/if}
			Submit</button
		>
	</form>
</div>
