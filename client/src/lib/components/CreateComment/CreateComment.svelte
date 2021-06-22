<script lang="ts">
	import { commentsStore } from '$lib/stores';

	export let postId: string;

	let comment = {
		contents: ''
	};

	let loading = false;

	async function onSubmit() {
		loading = true;
		try {
			await commentsStore.createComment(postId, comment);
			loading = false;
			comment.contents = '';
		} catch (error) {
			console.error(error.response.data);
			loading = false;
		}
	}
</script>

<form on:submit|preventDefault={onSubmit}>
	<div class="mb-2">
		<label for="contents" class="form-label"> Content </label>
		<textarea
			name="contents"
			class="form-control"
			rows="5"
			cols="60"
			bind:value={comment.contents}
			required
		/>
	</div>
	<button type="submit" class="btn btn-primary mt-2 float-end">Submit</button>
</form>
