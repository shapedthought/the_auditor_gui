<script lang="ts">
	import { accessToken, addressSet } from '../../stores.js';
	import type { User } from '../../types/user.type.js';
	import type { Group } from '../../types/group.type.js';
	import { invoke } from '@tauri-apps/api/tauri';
	import { save, open } from '@tauri-apps/api/dialog';
	import { SvelteToast, toast } from '@zerodevx/svelte-toast';
	import { dialog, fs } from '@tauri-apps/api';

	let filePath: object = {};
	let fileName = '';
	let selectedType = 'Users';
	let address = '';
	let accessTokenLocal = '';
	let users: User;

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
			path: readPath
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
			path: readPath
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

	async function save_json(jsonData: User | Group, fileName: string) {
		const filePath = await dialog.save({
			defaultPath: fileName,
			filters: [
				{
					name: 'JSON',
					extensions: ['json']
				}
			]
		});
		if (filePath != null) {
			// need to change this
			fs.writeFile(filePath, JSON.stringify(jsonData));
		} else {
			showErrorAlert('No file path selected!');
		}
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
			path: savePath
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
			path: savePath
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

<h1 class="title">Add</h1>

<div class="column">
	<div class="card card-content">
		<p>
			To add users or groups first click on the "Get" button, this will save Users or Groups to an
			Excel file
		</p>
		<p>Then you can edit the Excel file remove users or groups you do not want to Audit.</p>
		<p>Then click on either the "Add Users" or "Add Groups" buttons to add them to the Audit.</p>
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

<SvelteToast />
