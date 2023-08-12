<script lang="ts">
	import { accessToken, addressSet } from '../../stores.js';
	import { invoke } from '@tauri-apps/api';
	import Swal from 'sweetalert2';
	let useSmtp = false;
	let useExisting = true;

	let userId = '';
	let from = '';
	let to = '';
	let subject =
		'VBO Audit - %StartTime% — %OrganizationName% - %DisplayName% - %Action% - %InitiatedByUserName%';
	let accessTokenLocal = '';
	let address = '';
	let buttonDisabled = true;

	function enableButton() {
		if (userId.length > 0 && from.length > 0 && to.length > 0 && subject.length > 0) {
			buttonDisabled = false;
		} else {
			buttonDisabled = true;
		}
	}

	accessToken.subscribe((value) => {
		accessTokenLocal = value;
	});

	addressSet.subscribe((value) => {
		address = value;
	});

	async function setUpNotifications() {
		invoke('setup_notifications', {
			address: address,
			token: accessTokenLocal,
			userId: userId,
			from: from,
			to: to,
			subject: subject
		})
			.then((data) => {
				console.log(data);
				Swal.fire({
					icon: 'success',
					title: 'Success',
					text: 'Notifications set up successfully'
				});
			})
			.catch((err) => {
				console.log(err);
				Swal.fire({
					icon: 'error',
					title: 'Oops...',
					text: err
				});
			});
	}
</script>

<h1 class="title">Set up</h1>
<p>Current implementation only works with Microsoft with an existing Azure application.</p>

<div class="column" on:change={enableButton}>
	<div class="field">
		<label class="label" for="userId">User ID</label>
		<div class="control">
			<input class="input" type="text" id="userId" bind:value={userId} />
		</div>
	</div>
	<div class="field">
		<label class="label" for="from">From</label>
		<div class="control">
			<input class="input" type="text" id="from" bind:value={from} />
		</div>
	</div>
	<div class="field">
		<label class="label" for="to">To</label>
		<div class="control">
			<input class="input" type="text" id="to" bind:value={to} />
		</div>
	</div>
	<div class="field">
		<label class="label" for="subject">Subject</label>
		<div class="control">
			<input class="input" type="text" id="subject" bind:value={subject} />
		</div>
	</div>
	<button class="button" disabled={buttonDisabled} on:click={setUpNotifications}>Run Setup</button>
</div>
