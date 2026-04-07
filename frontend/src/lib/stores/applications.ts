import { writable } from 'svelte/store';

export const applications = writable([]);
export const applicationFilter = writable('all');
