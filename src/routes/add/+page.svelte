<script lang="ts">
	import { accessToken, addressSet, orgId } from '../../stores.js';
	import type { User } from '../../types/user.type.js';
	import type { Group } from '../../types/group.type.js';
	import { invoke } from '@tauri-apps/api/tauri';
	import { save, open } from '@tauri-apps/api/dialog';
	import { showAlert, showErrorAlert } from '../../lib/alerts.svelte';

	let address = '';
	let accessTokenLocal = '';
	let orgIdLocal = '';
	let users: User;
	let isActive = true;

	let buttonsLoading: { [key: string]: boolean } = {
		addUsers: false,
		addGroups: false,
		getUsers: false,
		getGroups: false
	};

	accessToken.subscribe((value) => {
		accessTokenLocal = value;
	});

	addressSet.subscribe((value) => {
		address = value;
	});

	orgId.subscribe((value) => {
		orgIdLocal = value;
	});

	async function addUsers() {
		buttonsLoading.addUsers = true;
		let readPath = await open({
			filters: [
				{
					name: 'excel',
					extensions: ['xlsx']
				}
			]
		});
		if (readPath == null) {
			buttonsLoading.addUsers = false;
			return;
		}
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
				showAlert(err);
				console.log(err);
			});
		buttonsLoading.addUsers = false;
	}

	async function addGroups() {
		buttonsLoading.addGroups = true;
		let readPath = await open({
			filters: [
				{
					name: 'excel',
					extensions: ['xlsx']
				}
			]
		});
		if (readPath == null) {
			buttonsLoading.addGroups = false;
			return;
		}
		invoke('add_groups', {
			address: address,
			token: accessTokenLocal,
			path: readPath,
			orgId: orgIdLocal
		})
			.then((data) => {
				const response = data as String;
				showAlert('Added groups!');
				console.log(response);
			})
			.catch((err) => {
				showErrorAlert(err);
				console.log(err);
			});
		buttonsLoading.addGroups = false;
	}

	async function getUsers() {
		buttonsLoading.getUsers = true;
		const savePath = await save({
			defaultPath: 'users.xlsx',
			filters: [
				{
					name: 'excel',
					extensions: ['xlsx']
				}
			]
		});
		if (savePath == null) {
			buttonsLoading.getUsers = false;
			return;
		}
		invoke('get_users', {
			address: address,
			token: accessTokenLocal,
			path: savePath,
			orgId: orgIdLocal
		})
			.then((data) => {
				users = data as User;
				showAlert('Got users!');
				console.log(data);
			})
			.catch((err) => {
				showErrorAlert(err);
				console.log(err);
			});
		buttonsLoading.getUsers = false;
	}

	async function getGroups() {
		buttonsLoading.getGroups = true;
		const savePath = await save({
			defaultPath: 'groups.xlsx',
			filters: [
				{
					name: 'excel',
					extensions: ['xlsx']
				}
			]
		});
		if (savePath == null) {
			buttonsLoading.getGroups = false;
			return;
		}
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
		buttonsLoading.getGroups = false;
	}
</script>

<div class="columns is-centered">
	<div class="column">
		<div class="columns">
			<div class="column">
				<h1 class="title">Add</h1>
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
				<div class="card-content has-text-centered">
					<button
						class="button is-primary mr-2"
						class:is-loading={buttonsLoading.getUsers}
						on:click={getUsers}>Get Users</button
					>
					<button
						class="button is-primary"
						class:is-loading={buttonsLoading.getGroups}
						on:click={getGroups}>Get Groups</button
					>
				</div>
			</div>
		</div>

		<div class="column">
			<div class="card">
				<div class="card-header">
					<p class="card-header-title">Add Users or Groups</p>
				</div>
				<div class="card-content has-text-centered">
					<button
						class="button is-info mr-2"
						on:click={addUsers}
						class:is-loading={buttonsLoading.addUsers}>Add Users</button
					>
					<button
						class="button is-info"
						on:click={addGroups}
						class:is-loading={buttonsLoading.addGroups}>Add Groups</button
					>
				</div>
			</div>
		</div>
	</div>
</div>
