<script lang="ts">
	import { accessToken, addressSet, orgId } from '../../stores.js';
	import type { User } from '../../types/user.type.js';
	import type { Group } from '../../types/group.type.js';
	import { invoke } from '@tauri-apps/api/tauri';
	import { save, open } from '@tauri-apps/api/dialog';
	import { SvelteToast, toast } from '@zerodevx/svelte-toast';
	import { dialog, fs } from '@tauri-apps/api';

	let address = '';
	let accessTokenLocal = '';
	let orgIdLocal = '';
	let users: User;
	let itemsInBasket = 12;
	let isActive = true;

	let activeTabValue = 1;
	let tabs = [
		{
			id: 1,
			label: 'Users'
		},
		{
			id: 2,
			label: 'Groups'
		}
	];

	const handleClick = (tab: { id: number; label: string }) => {
		activeTabValue = tab.id;
	};

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

	accessToken.subscribe((value) => {
		accessTokenLocal = value;
	});

	addressSet.subscribe((value) => {
		address = value;
	});

	orgId.subscribe((value) => {
		orgIdLocal = value;
	});

	function switchActive() {
		isActive = !isActive;
	}

	async function add_users() {
		let readPath = await open({
			filters: [
				{
					name: 'excel',
					extensions: ['xlsx']
				}
			]
		});
		invoke('add_users', {
			address: address,
			token: accessTokenLocal,
			path: readPath,
			orgId: orgIdLocal
		})
			.then((data) => {
				const response = data as String;
				showAlert('Added users!');
				// save_json(users, 'users.json');
				console.log(response);
			})
			.catch((err) => {
				showErrorAlert(err);
				console.log(err);
			});
	}

	async function add_groups() {
		let readPath = await open({
			filters: [
				{
					name: 'excel',
					extensions: ['xlsx']
				}
			]
		});
		invoke('add_groups', {
			address: address,
			token: accessTokenLocal,
			path: readPath,
			orgId: orgIdLocal
		})
			.then((data) => {
				const response = data as String;
				showAlert('Added users!');
				// save_json(users, 'users.json');
				console.log(response);
			})
			.catch((err) => {
				showErrorAlert(err);
				console.log(err);
			});
	}

	async function get_users() {
		const savePath = await save({
			defaultPath: 'users.xlsx',
			filters: [
				{
					name: 'excel',
					extensions: ['xlsx']
				}
			]
		});
		invoke('get_users', {
			address: address,
			token: accessTokenLocal,
			path: savePath,
			orgId: orgIdLocal
		})
			.then((data) => {
				users = data as User;
				showAlert('Got users!');
				// save_json(users, 'users.json');
				console.log(data);
			})
			.catch((err) => {
				showErrorAlert(err);
				console.log(err);
			});
	}

	async function get_groups() {
		const savePath = await save({
			defaultPath: 'groups.xlsx',
			filters: [
				{
					name: 'excel',
					extensions: ['xlsx']
				}
			]
		});
		invoke('get_groups', {
			address: address,
			token: accessTokenLocal,
			path: savePath,
			orgId: orgIdLocal
		})
			.then((data) => {
				users = data as Group;
				showAlert('Got Groups!');
				console.log(data);
			})
			.catch((err) => {
				showErrorAlert(err);
				console.log(err);
			});
	}
</script>

<div class="columns is-centered">
	<div class="column">
		<div class="columns">
			<div class="column">
				<h1 class="title">Add</h1>
			</div>
			<div class="column is-2">
				<i class="fa-solid fa-basket-shopping" />
				<a class="tag is-primary" href="/basket">{itemsInBasket}</a>
			</div>
		</div>
		<div class="column">
			<div class="tabs">
				<ul>
					{#each tabs as tab}
						<li class={tab.id === activeTabValue ? 'is-active' : ''}>
							<!-- svelte-ignore a11y-click-events-have-key-events -->
							<!-- svelte-ignore a11y-no-static-element-interactions -->
							<!-- svelte-ignore a11y-missing-attribute -->
							<a on:click={() => handleClick(tab)}>{tab.label}</a>
						</li>
					{/each}
				</ul>
			</div>
		</div>

		<div class="column">
			<div class="card card-content">
				<p>
					To add users or groups first click on the "Get" button, this will save Users or Groups to
					an Excel file
				</p>
				<p>Then you can edit the Excel file remove users or groups you do not want to Audit.</p>
				<p>
					Then click on either the "Add Users" or "Add Groups" buttons to add them to the Audit.
				</p>
				<p>Note: Do not change the sheet name or the structure of the columns!</p>
			</div>
		</div>
		<div class="column">
			<div class="card">
				<div class="card-header">
					<p class="card-header-title">Get Users or Groups</p>
				</div>
				<div class="card-content">
					<button class="button mr-2" on:click={get_users}>Get Users</button>
					<button class="button" on:click={get_groups}>Get Groups</button>
				</div>
			</div>
		</div>

		<div class="column">
			<div class="card">
				<div class="card-header">
					<p class="card-header-title">Add Users or Groups</p>
				</div>
				<div class="card-content">
					<button class="button mr-2" on:click={add_users}>Add Users</button>
					<button class="button" on:click={add_groups}>Add Groups</button>
				</div>
			</div>
		</div>
	</div>
</div>
<SvelteToast />
