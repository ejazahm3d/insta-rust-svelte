<script lang="ts">
	import { postsStore } from '$lib/stores';

	const post = {
		caption: '',
		url: ''
	};

	let loading = false;

	async function onSubmit() {
		loading = true;
		try {
			await postsStore.createPost(post);
			loading = false;
			post.caption = '';
			post.url = '';
		} catch (error) {
			console.error(error.response.data);
			loading = false;
		}
	}
</script>

<h2>Create Post</h2>

<form on:submit|preventDefault={onSubmit}>
	<div class="mb-2">
		<label for="caption" class="form-label"> Caption </label>
		<textarea
			name="caption"
			class="form-control"
			rows="5"
			cols="100"
			bind:value={post.caption}
			required
		/>
	</div>

	<div>
		<label for="url" class="form-label"> Url </label>
		<input name="url" class="form-control" rows="5" cols="100" bind:value={post.url} required />
	</div>

	<button type="submit" class="btn btn-primary mt-2 float-end">Submit</button>
</form>
