<script lang="ts">
	import agent from '$lib/api/agent';
	import { writable } from 'svelte/store';

	const user = writable({
		email: '',
		username: '',
		password: ''
	});

	async function login() {
		try {
			await agent.Account.signup($user);
		} catch (error) {
			console.log(error.response.data);
		}
	}
</script>

<h1>Signup page</h1>

<form on:submit|preventDefault={login}>
	<div>
		<label for="email">Email</label>
		<input name="email" type="email" bind:value={$user.email} required />
	</div>

	<div>
		<label for="username">Username</label>
		<input name="username" type="text" bind:value={$user.username} required />
	</div>
	<div>
		<label for="password">Password</label>
		<input name="password" type="password" bind:value={$user.password} required />
	</div>

	<button type="submit">Submit</button>
</form>
