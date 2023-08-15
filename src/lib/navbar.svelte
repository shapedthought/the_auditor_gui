<script lang="ts">
	import Swal from 'sweetalert2';
	let orgNameLocal = 'OFFLINE';
	let loginStatus = false;
	import { loggedIn, orgName } from '../stores';
	import { goto } from '$app/navigation';

	loggedIn.subscribe((value) => {
		loginStatus = value;
	});

	orgName.subscribe((value) => {
		orgNameLocal = value;
	});

	function logout() {
		Swal.fire({
			title: 'Are you sure?',
			text: `You will be logged out!`,
			icon: 'warning',
			showCancelButton: true,
			confirmButtonText: 'Yes, log me out!',
			cancelButtonText: 'No, keep me logged in'
		}).then((result) => {
			if (result.isConfirmed) {
				loggedIn.set(false);
				orgName.set('OFFLINE');
				goto('/');
			} else if (result.dismiss === Swal.DismissReason.cancel) {
				Swal.fire('Cancelled', 'Phew!', 'error');
			}
		});
	}
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
			<div class="navbar-item">
				<button class="button is-danger is-small" on:click={logout}>Log out</button>
			</div>
			<p class="navbar-item">{orgNameLocal}</p>
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
