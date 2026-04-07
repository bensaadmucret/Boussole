import { writable } from 'svelte/store';
import type { JobListing } from '../types';

export const listings = writable<JobListing[]>([]);

export interface ListingFilters {
  company?: string;
  contractTypes?: string[];
  remoteTypes?: string[];
  stack?: string[];
  salaryMin?: number;
  hasApplication?: boolean;
}

export interface ListingSearchState {
  query: string;
  filters: ListingFilters;
  results: JobListing[];
  isLoading: boolean;
}

export const listingSearch = writable<ListingSearchState>({
  query: '',
  filters: {},
  results: [],
  isLoading: false
});
