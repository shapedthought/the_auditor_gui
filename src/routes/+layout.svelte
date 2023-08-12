<script>
	import Menu from '$lib/menu.svelte';
</script>

<div class="container mt-5">
	<div class="columns">
		<div class="column is-one-quarter" id="menu-column">
			<Menu />
		</div>
		<div class="column">
			<slot />
		</div>
	</div>
</div>
