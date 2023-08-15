<script>
	import Menu from '$lib/menu.svelte';
	import Navbar from '$lib/navbar.svelte';
</script>

<div class="columns">
	<div class="column">
		<Navbar />
	</div>
</div>
<div class="container">
	<slot />
</div>
