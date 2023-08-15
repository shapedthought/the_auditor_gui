<script>
	import { loggedIn } from '../stores.js';

	let loggedInLocal = false;

	loggedIn.subscribe((value) => {
		loggedInLocal = value;
	});
</script>

<aside class="menu">
	<ul class="menu-list">
		<li><a href="/login">Login</a></li>
		{#if loggedInLocal}
			<li><a href="/setup">Set Up</a></li>
			<li><a href="/add">Add</a></li>
			<li><a href="/remove">Remove</a></li>
		{/if}
	</ul>
</aside>

<style>
	.menu {
		border-right: 1px solid rgb(119, 118, 118);
	}
</style>
