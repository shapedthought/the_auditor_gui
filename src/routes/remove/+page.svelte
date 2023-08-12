<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { accessToken, addressSet } from '../../stores.js';
	import type { AuthItem, User, Group } from '../../types/auth.type.js';
	import Tables from '$lib/tables.svelte';
	import Swal from 'sweetalert2';

	function handle_message(message: any) {
		show_confirmation(message.detail.text);
		console.log(message.detail.text);
	}

	let address = '';
	let accessTokenLocal = '';
	let authItems: AuthItem[] = [];
	let users: User[] = [];
	let groups: Group[] = [];

	function show_confirmation(name: string) {
		Swal.fire({
			title: 'Are you sure?',
			text: `${name} will be deleted!`,
			icon: 'warning',
			showCancelButton: true,
			confirmButtonText: 'Yes, delete it!',
			cancelButtonText: 'No, keep it'
		}).then((result) => {
			if (result.isConfirmed) {
				Swal.fire('Deleted!', 'Audit it has been deleted.', 'success');
			} else if (result.dismiss === Swal.DismissReason.cancel) {
				Swal.fire('Cancelled', 'Audit item has been saved :)', 'error');
			}
		});
	}

	accessToken.subscribe((value) => {
		accessTokenLocal = value;
	});

	addressSet.subscribe((value) => {
		address = value;
	});

	function sort_auth() {
		users = [];
		groups = [];
		authItems.forEach((item) => {
			if (item.type == 'User') {
				if (item.user != null) {
					users.push(item.user);
				}
			} else if (item.type == 'Group') {
				if (item.group != null) {
					groups.push(item.group);
				}
			}
		});
		users = users;
		groups = groups;
	}

	async function get_audit() {
		invoke('get_audit', {
			address: address,
			token: accessTokenLocal
		})
			.then((data) => {
				authItems = data as AuthItem[];
				sort_auth();
				console.log(data);
			})
			.catch((err) => {
				console.log(err);
			});
	}
</script>

<button class="button is-small" on:click={get_audit}>Refresh</button>
<h1 class="title">Remove</h1>
<Tables {users} {groups} on:message={handle_message} />
