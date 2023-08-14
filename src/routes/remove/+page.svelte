<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { accessToken, addressSet, orgId } from '../../stores.js';
	import type { AuthItem, User, Group } from '../../types/auth.type.js';
	import { SvelteToast, toast } from '@zerodevx/svelte-toast';
	import Tables from '$lib/tables.svelte';
	import Swal from 'sweetalert2';
	import { onMount } from 'svelte';

	let auditId = '';

	function showErrorAlert(err: string) {
		toast.push('Error!', {
			theme: {
				'--toastColor': 'black',
				'--toastBackground': 'rgba(255,41,50,0.8)',
				'--toastBarBackground': '#800000'
			}
		});
	}

	function handle_message(message: any) {
		auditId = message.detail.text;
		let deleteId = '';
		authItems.forEach((item) => {
			if (item.user != undefined) {
				if (item.user.id == auditId) {
					deleteId = item.id;
				}
			} else if (item.group != undefined) {
				if (item.group.id == auditId) {
					deleteId = item.id;
				}
			}
		});

		show_confirmation(auditId, deleteId);
		console.log(`Audit ID: ${auditId}`);
		console.log(`Delete ID: ${deleteId}`);
	}

	function remove_item(id: string) {
		invoke('delete_audit_item', {
			address: address,
			token: accessTokenLocal,
			id: id
		})
			.then((data) => {
				let response = data as string;
				Swal.fire('Deleted!', 'Audit item has been deleted.', 'success');
				console.log(response);
				get_audit();
			})
			.catch((err) => {
				showErrorAlert(err);
				console.log(err);
			});
	}

	let address = '';
	let accessTokenLocal = '';
	let orgIdLocal = '';
	let authItems: AuthItem[] = [];
	let users: User[] = [];
	let groups: Group[] = [];

	function show_confirmation(userId: string, auditId: string) {
		Swal.fire({
			title: 'Are you sure?',
			text: `${userId} will be deleted!`,
			icon: 'warning',
			showCancelButton: true,
			confirmButtonText: 'Yes, delete it!',
			cancelButtonText: 'No, keep it'
		}).then((result) => {
			if (result.isConfirmed) {
				remove_item(auditId);
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

	orgId.subscribe((value) => {
		orgIdLocal = value;
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
			token: accessTokenLocal,
			orgId: orgIdLocal
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

	onMount(() => {
		get_audit();
	});
</script>

<div class="columns is-centered">
	<div class="column">
		<h1 class="title">Remove</h1>
		<Tables {users} {groups} on:message={handle_message} />
	</div>
</div>
<SvelteToast />
