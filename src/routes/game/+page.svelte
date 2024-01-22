<script lang="ts">
	import VTT from '$lib/vtt';

	(async () => {
		await VTT.init();
		VTT.attach_canvas('#bevy-canvas');
	})();
</script>

<div class="w-screen h-screen">
	<div class="w-full h-full absolute -z-10 flex justify-center items-center flex-col">
		<div class="loading loading-infinity loading-lg"></div>
		<div>loading canvas</div>
	</div>
	<canvas class="w-screen h-screen" id="bevy-canvas"></canvas>
</div>
