<script lang="ts">
	import agent from '$lib/api';
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
			await accountsStore.login($user);

			loading = false;
			await goto('/');
		} catch (error) {
			loading = false;
			console.log(error.response.data);
		}
	}
</script>

<div class="p-10 card bg-base-200 max-w-md mx-auto mt-10 md:mt-20">
	<h1 class="text-center mt-5 text-3xl mb-3 font-semibold">Login page</h1>

	<form on:submit|preventDefault={login}>
		<div class="mb-3 form-control">
			<label class="label" for="email"> Email </label>
			<input
				class="input input-bordered"
				name="email"
				type="email"
				bind:value={$user.email}
				required
			/>
		</div>

		<div class="mb-3 form-control">
			<label class="label label-text" for="password">Password</label>
			<input
				class="input input-bordered"
				name="password"
				type="password"
				bind:value={$user.password}
				required
			/>
		</div>

		<button
			type="submit"
			class={`btn btn-primary w-full mt-3 ${loading ? 'loading' : ''}`}
			disabled={loading}
		>
			Submit</button
		>
	</form>
</div>
