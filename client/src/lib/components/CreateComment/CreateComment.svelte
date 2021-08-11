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
	<div class="mb-2 form-control">
		<label for="contents" class="label label-text"> Comment </label>
		<textarea
			name="contents"
			class="textarea textarea-primary h-30 w-full"
			bind:value={comment.contents}
			required
		/>
	</div>
	<button type="submit" class="btn btn-primary mt-2 float-end">Submit</button>
</form>
