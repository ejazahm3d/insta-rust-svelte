<script lang="ts">
	import { store } from '$lib/store';

	import { commentCreate } from '$lib/store/services/comments';

	export let postId: string;

	let comment = {
		contents: ''
	};

	let loading = false;

	async function onSubmit() {
		loading = true;
		try {
			await store.dispatch(commentCreate.initiate({ postId, body: comment }));
			loading = false;
			comment.contents = '';
		} catch (error) {
			console.error(error);
			loading = false;
		}
	}
</script>

<form class="flex flex-col " on:submit|preventDefault={onSubmit}>
	<div class="mb-2 form-control">
		<label for="contents" class="label label-text"> Comment </label>
		<textarea
			name="contents"
			class="textarea textarea-primary w-full"
			bind:value={comment.contents}
			required
		/>
	</div>
	<button type="submit" class="btn btn-primary mt-2 self-end">Submit</button>
</form>
