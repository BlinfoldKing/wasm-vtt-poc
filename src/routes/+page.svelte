<script lang="ts">
	import { onMount } from 'svelte';
	import VTT from '$lib/vtt';

	onMount(async () => {
		await VTT.init();
	});
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
