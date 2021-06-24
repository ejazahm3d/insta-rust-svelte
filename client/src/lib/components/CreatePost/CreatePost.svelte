<script lang="ts">
	import { postsStore } from '$lib/stores';
	import { writable } from 'svelte/store';

	const store = writable<{
		files: FileList;
		avatar: string;
		caption: string;
	}>({
		files: null,
		avatar: '',
		caption: ''
	});

	const onFileSelected = (
		e: Event & {
			currentTarget: EventTarget & HTMLInputElement;
		}
	) => {
		let image = e.currentTarget.files[0];
		let reader = new FileReader();
		reader.readAsDataURL(image);
		reader.onload = (e) => {
			$store.avatar = e.target.result as string;
		};
	};

	let loading = false;

	async function onSubmit() {
		loading = true;
		try {
			const data = await postsStore.uploadPhoto($store.files[0]);
			await postsStore.createPost({ caption: $store.caption, url: data.filepath });
		} catch (error) {
			console.error(error.response.data);
			loading = false;
		} finally {
			store.set({ files: null, avatar: '', caption: '' });
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
			bind:value={$store.caption}
			required
		/>
	</div>

	<div>
		{#if $store.avatar}
			<img src={$store.avatar} alt="post uplaod" />
		{/if}

		<input
			type="file"
			name="img"
			id="img"
			accept=".jpg, .jpeg, .png"
			on:change={onFileSelected}
			bind:files={$store.files}
			required
		/>
	</div>

	<button type="submit" class="btn btn-primary mt-2 float-end">Submit</button>
</form>

<style>
	img {
		max-width: 400px;
	}
</style>
