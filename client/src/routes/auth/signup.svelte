<script lang="ts">
	import { goto } from '$app/navigation';

	import { accountsStore } from '$lib/auth';
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
			await accountsStore.signup($user);
			loading = false;
			await goto('/');
		} catch (error) {
			loading = false;
			console.log(error.response.data);
		}
	}
</script>

<div class="p-10 card bg-base-200 max-w-md mx-auto mt-10 md:mt-20">
	<h1 class="text-center mt-5 text-3xl font-semibold mb-3 ">Signup</h1>

	<form on:submit|preventDefault={signup}>
		<div class="mb-3 form-control">
			<label class="label label-text" for="email">Email</label>
			<input
				class="input input-bordered"
				name="email"
				type="email"
				bind:value={$user.email}
				required
			/>
		</div>

		<div class="mb-3 form-control">
			<label class="label label-text" for="username">Username</label>
			<input
				class="input input-bordered"
				name="username"
				type="text"
				bind:value={$user.username}
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
			class={`btn btn-primary w-full mt-3 ${loading && 'loading'}`}
			type="submit"
			disabled={loading}
		>
			Submit</button
		>
	</form>
</div>
