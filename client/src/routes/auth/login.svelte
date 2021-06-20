<script lang="ts">
	import agent from '$lib/api/agent';
	import { writable } from 'svelte/store';
	import { goto } from '$app/navigation';
	import { accountsStore } from '$lib/stores/index';

	const user = writable({
		email: '',
		password: ''
	});

	async function login() {
		try {
			const data = await agent.Account.login($user);

			accountsStore.set({ user: data.id });

			await goto('/');
		} catch (error) {
			console.log(error.response.data);
		}
	}
</script>

<h1>Login page</h1>

<form on:submit|preventDefault={login}>
	<div>
		<label for="email">Email</label>
		<input name="email" type="email" bind:value={$user.email} required />
	</div>

	<div>
		<label for="password">Password</label>
		<input name="password" type="password" bind:value={$user.password} required />
	</div>

	<button type="submit">Submit</button>
</form>
