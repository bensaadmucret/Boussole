import { writable } from 'svelte/store';

export const listings = writable([]);
export const listingSearch = writable({
  query: '',
  filters: {},
  results: [],
  isLoading: false
});
