<script lang="ts">
	import { goto } from '$app/navigation';
	import { loggedIn, accessToken, addressSet, orgId, orgName } from '../stores.js';
	import type { OrgSummary } from '../types/org.type.js';
	import { invoke } from '@tauri-apps/api/tauri';
	import { SvelteToast, toast } from '@zerodevx/svelte-toast';

	let chooseOrgs = false;
	let address = '';
	let username = '';
	let password = '';
	let orgs: OrgSummary[] = [];
	let selectedOrgId = '';
	let accessTokenLocal = '';

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

	function setOrg() {
		orgId.set(selectedOrgId);
		let orgSelectName = orgs.find((org) => org.id === selectedOrgId)?.name as string;
		orgName.set(orgSelectName);
		chooseOrgs = false;
		goto('/setup');
	}

	async function login() {
		try {
			let response: string = await invoke('login', {
				username: username,
				password: password,
				address: address
			});
			console.log(response);
			loggedIn.set(true);
			addressSet.set(address);
			accessToken.set(response);
			accessTokenLocal = response;
		} catch (err) {
			console.log(err);
			showErrorAlert(err as string);
		}

		try {
			orgs = await invoke('get_orgs', {
				address: address,
				token: accessTokenLocal
			});
			console.log(orgs);
			if (orgs.length > 1) {
				chooseOrgs = true;
			} else {
				orgId.set(orgs[0].id);
				orgName.set(orgs[0].name);
				goto('/setup');
			}
		} catch (err) {
			console.log(err);
			showErrorAlert(err as string);
		}
	}
</script>

<div class="columns is-centered">
	<div class="column is-half">
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
				<input
					class="input"
					type="email"
					placeholder="Email input"
					id="email"
					bind:value={username}
				/>
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
	</div>
</div>

<div id="modal-js-example" class="modal" class:is-active={chooseOrgs}>
	<div class="modal-background" />

	<div class="modal-content">
		<div class="box">
			<div class="select">
				<select bind:value={selectedOrgId}>
					{#each orgs as org}
						<option value={org.id}>{org.name}</option>
					{/each}
				</select>
			</div>
			<button on:click={setOrg}>Set Org</button>
		</div>
	</div>
</div>

<SvelteToast />
