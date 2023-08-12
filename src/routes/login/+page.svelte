<script lang="ts">
	import { loggedIn, accessToken, addressSet } from '../../stores.js';
	import { invoke } from '@tauri-apps/api/tauri';
	import { SvelteToast, toast } from '@zerodevx/svelte-toast';

	let address = '';
	let username = '';
	let password = '';

	function showAlert(message: string) {
		toast.push(message, {
			theme: {
				'--toastColor': 'mintcream',
				'--toastBackground': 'rgba(72,187,120,0.9)',
				'--toastBarBackground': '#2F855A'
			}
		});
	}

	function showErrorAlert(err: string) {
		toast.push('Error!', {
			theme: {
				'--toastColor': 'black',
				'--toastBackground': 'rgba(255,41,50,0.8)',
				'--toastBarBackground': '#800000'
			}
		});
	}

	async function login() {
		invoke('login', {
			username: username,
			password: password,
			address: address
		})
			.then((data) => {
				console.log(data);
				loggedIn.set(true);
				addressSet.set(address);
				accessToken.set(data as string);
				showAlert('Logged in!');
			})
			.catch((err) => {
				console.log(err);
				showErrorAlert(err);
			});
	}
</script>

<h1 class="title">Log In</h1>
<div class="field">
	<label class="label" for="vbaddress">VB365 Address</label>
	<div class="control">
		<input
			class="input"
			type="text"
			placeholder="Address input"
			id="vbaddress"
			bind:value={address}
		/>
	</div>
</div>
<div class="field">
	<label class="label" for="email">Email</label>
	<div class="control">
		<input class="input" type="email" placeholder="Email input" id="email" bind:value={username} />
	</div>
</div>
<div class="field">
	<label class="label" for="password">Password</label>
	<div class="control">
		<input
			class="input"
			type="password"
			placeholder="Password input"
			id="password"
			bind:value={password}
		/>
	</div>
</div>
<button class="button" on:click={login}>Submit</button>
<SvelteToast />
