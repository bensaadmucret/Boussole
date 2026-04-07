import { invoke } from '@tauri-apps/api/tauri';

export async function createJobListing(data: any) {
  return await invoke('create_job_listing', { data });
}

export async function getJobListings() {
  return await invoke('get_job_listings');
}

export async function createApplication(data: any) {
  return await invoke('create_application', { data });
}

export async function getApplications() {
  return await invoke('get_applications');
}
