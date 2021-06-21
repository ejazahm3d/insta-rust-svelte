<script lang="ts">
	import { accountsStore } from '$lib/stores';

	export let isLoggedIn: boolean;

	const logout = async () => {
		await accountsStore.logout();
	};
</script>

<nav class="navbar navbar-expand-lg navbar-light bg-light">
	<a class="navbar-brand" href="/">Navbar</a>
	<button
		class="navbar-toggler"
		type="button"
		data-toggle="collapse"
		data-target="#navbarNav"
		aria-controls="navbarNav"
		aria-expanded="false"
		aria-label="Toggle navigation"
	>
		<span class="navbar-toggler-icon" />
	</button>
	<div class="collapse navbar-collapse" id="navbarNav">
		<ul class="navbar-nav">
			<li class="nav-item active">
				<a class="nav-link" href="/">Home </a>
			</li>
			{#if isLoggedIn}
				<li class="nav-item">
					<button class="btn btn-danger " on:click={logout}>Logout</button>
				</li>
			{:else}
				<li class="nav-item">
					<a class="nav-link" href="/auth/signup">Sign Up</a>
				</li>
				<li class="nav-item">
					<a class="nav-link" href="/auth/login">Login</a>
				</li>
			{/if}
		</ul>
	</div>
</nav>
