<script lang="ts">
	import { accessToken, addressSet } from '../../stores.js';
	import type { User } from '../../types/user.type.js';
	import type { Group } from '../../types/group.type.js';
	import { invoke } from '@tauri-apps/api/tauri';
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

	function handleFileInput(event: any) {
		filePath = event.target.files[0];
		fileName = event.target.files[0].name;

		console.log(filePath);
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
			fs.writeFile(filePath, JSON.stringify(jsonData));
		} else {
			showErrorAlert('No file path selected!');
		}
	}

	async function get_users() {
		invoke('get_users', {
			address: address,
			token: accessTokenLocal
		})
			.then((data) => {
				users = data as User;
				showAlert('Got users!');
				save_json(users, 'users.json');
				console.log(data);
			})
			.catch((err) => {
				showErrorAlert(err);
				console.log(err);
			});
	}

	async function get_groups() {
		invoke('get_groups', {
			address: address,
			token: accessTokenLocal
		})
			.then((data) => {
				users = data as Group;
				showAlert('Got Groups!');
				save_json(users, 'groups.json');
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
			<div class="file">
				<label class="file-label">
					<input
						class="file-input"
						type="file"
						accept=".json"
						name="resume"
						on:input={handleFileInput}
					/>
					<span class="file-cta">
						<span class="file-label"> Choose a file… </span>
					</span>
				</label>
			</div>
			<p>{fileName}</p>
		</div>
	</div>
</div>
{#if fileName.length > 0}
	<div class="column">
		<div class="card">
			<div class="card-header">
				<p class="card-header-title">Select if file has Users or Groups</p>
			</div>
			<div class="card-body">
				<div class="column">
					<div class="select mr-2">
						<select bind:value={selectedType}>
							<option value="Users">Users</option>
							<option value="Groups">Groups</option>
						</select>
					</div>
					<button class="button">Add {selectedType}</button>
				</div>
			</div>
		</div>
	</div>
{/if}
<SvelteToast />
