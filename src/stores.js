import { writable } from 'svelte/store';

export const loggedIn = writable(false);

export const accessToken = writable('');

export const addressSet = writable('');
