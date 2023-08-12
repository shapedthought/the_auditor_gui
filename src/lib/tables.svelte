<script lang="ts">
	import type { AuthItem, User, Group } from '../types/auth.type';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	function sayHello(name: string) {
		dispatch('message', {
			text: name
		});
	}

	export let users: User[] = [];
	export let groups: Group[] = [];
</script>

{#if users.length > 0}
	<div class="column">
		<h4 class="title is-4">Users</h4>
		<table class="table is-fullwidth">
			<thead>
				<tr>
					<th>Name</th>
					<th>Display Name</th>
					<th>Type</th>
					<th>Remove</th>
				</tr>
			</thead>
			<tbody>
				{#each users as user}
					<tr>
						<td>{user.name}</td>
						<td>{user.displayName}</td>
						<td>{user.locationType}</td>
						<td
							><button class="button is-small is-danger" on:click={() => sayHello(user.name)}
								>X</button
							></td
						>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
{:else}
	<div class="column">
		<h4 class="title is-4">Groups</h4>
		<p>No users found.</p>
	</div>
{/if}

{#if groups.length > 0}
	<div class="column">
		<h4 class="title is-4">Groups</h4>
		<table class="table is-fullwidth">
			<thead>
				<tr>
					<th>Name</th>
					<th>Display Name</th>
					<th>Type</th>
					<th>Remove</th>
				</tr>
			</thead>
			<tbody>
				{#each groups as group}
					<tr>
						<td>{group.name}</td>
						<td>{group.displayName}</td>
						<td>{group.locationType}</td>
						<td><button class="button is-small is-danger">X</button></td>
					</tr>
				{/each}
			</tbody>
		</table>
		%$
	</div>
{:else}
	<div class="column">
		<h4 class="title is-4">Groups</h4>
		<p>No groups found.</p>
	</div>
{/if}
