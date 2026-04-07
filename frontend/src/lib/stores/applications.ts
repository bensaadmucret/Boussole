import { writable } from 'svelte/store';
import type { Application } from '../types';

export const applications = writable<Application[]>([]);
export const applicationFilter = writable<string>('all');
