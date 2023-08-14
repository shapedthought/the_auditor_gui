<script lang="ts">
	let orgNameLocal = 'OFFLINE';
	let loginStatus = false;
	import { loggedIn, orgName } from '../stores';

	loggedIn.subscribe((value) => {
		loginStatus = value;
	});

	orgName.subscribe((value) => {
		orgNameLocal = value;
	});
</script>

<nav class="navbar is-fixed-top">
	<div class="navbar-start">
		<h2 class="navbar-item" id="logo">The Auditor</h2>
		{#if loginStatus}
			<a href="/setup" class="navbar-item">Set Up</a>
			<a href="/add" class="navbar-item">Add</a>
			<a href="/remove" class="navbar-item">Remove</a>
		{/if}
	</div>
	{#if loginStatus}
		<div class="navbar-end">
			<a href="/" class="navbar-item">Login</a>
			<p class="navbar-item">Logged in org: {orgNameLocal}</p>
		</div>
	{/if}
</nav>

<style>
	.navbar {
		border-bottom: 1px solid rgb(119, 118, 118);
	}

	#logo {
		font-family: 'Garramond';
		font-size: x-large;
		background-color: black;
		color: white;
	}
</style>
