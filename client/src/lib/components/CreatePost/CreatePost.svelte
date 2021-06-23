<script lang="ts">
	import { postsStore } from '$lib/stores';

	let files: FileList,
		avatar: string,
		caption = '';

	const onFileSelected = (
		e: Event & {
			currentTarget: EventTarget & HTMLInputElement;
		}
	) => {
		let image = e.currentTarget.files[0];
		let reader = new FileReader();
		reader.readAsDataURL(image);
		reader.onload = (e) => {
			avatar = e.target.result as string;
		};
	};

	let loading = false;

	async function onSubmit() {
		loading = true;
		try {
			postsStore.uploadPhoto(files[0]).then(async (data) => {
				await postsStore.createPost({ caption: caption, url: data.filepath }).catch(() => {
					console.log('something went wrong');
				});
				files = null;
			});
			loading = false;
			caption = '';
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
			bind:value={caption}
			required
		/>
	</div>

	<div>
		{#if avatar}
			<img src={avatar} alt="post uplaod" />
		{/if}

		<input
			type="file"
			name="img"
			id="img"
			accept=".jpg, .jpeg, .png"
			on:change={onFileSelected}
			bind:files
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
