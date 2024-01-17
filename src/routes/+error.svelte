<script>
	import { page } from '$app/stores';
	import { Skull } from 'lucide-svelte';
</script>

<div class="flex min-h-screen min-w-screen items-center justify-center">
	<Skull size="108" />
	<div class="flex flex-col">
		<h1 class="text-6xl">{$page.error?.code ?? 404}</h1>
		<p class="text-3xl text-gray-700">{$page.error?.message}</p>
	</div>
</div>
