import { invoke } from '@tauri-apps/api/tauri';
import type { JobListing, CreateJobListingInput, Application } from '../types';

export async function createJobListing(data: CreateJobListingInput): Promise<JobListing> {
  return await invoke('create_job_listing', { data });
}

export async function getJobListings(): Promise<JobListing[]> {
  return await invoke('get_job_listings');
}

export async function updateJobListing(id: number, data: CreateJobListingInput): Promise<JobListing> {
  return await invoke('update_job_listing', { id, data });
}

export async function deleteJobListing(id: number): Promise<void> {
  return await invoke('delete_job_listing', { id });
}

export async function checkDuplicateCompany(companyName: string): Promise<boolean> {
  return await invoke('check_duplicate_company', { companyName });
}

export async function getExistingCompanyListings(companyName: string): Promise<JobListing[]> {
  return await invoke('get_existing_company_listings', { companyName });
}

export async function createApplication(data: any): Promise<Application> {
  return await invoke('create_application', { data });
}

export async function getApplications(): Promise<Application[]> {
  return await invoke('get_applications');
}
