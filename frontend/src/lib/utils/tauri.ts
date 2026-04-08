import { invoke } from '@tauri-apps/api/core';
import type {
  Application,
  CreateApplicationInput,
  CreateDocumentInput,
  CreateJobListingInput,
  Document,
  GeneratedLetter,
  GenerateLetterInput,
  GeminiConfig,
  GoogleCalendarAccount,
  GoogleOAuthConfig,
  JobListing,
  UnifiedCalendarEvent,
  UnifiedCalendarSettings
} from '../types';

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

export async function updateApplicationStatus(id: number, status: string): Promise<Application> {
  return await invoke('update_application_status', { id, status });
}

// Document commands
export async function createDocument(data: CreateDocumentInput): Promise<Document> {
  return await invoke('create_document', { data });
}

export async function getDocuments(docType?: string): Promise<Document[]> {
  return await invoke('get_documents', { docType });
}

export async function getDocumentById(id: number): Promise<Document> {
  return await invoke('get_document_by_id', { id });
}

export async function deleteDocument(id: number): Promise<void> {
  return await invoke('delete_document', { id });
}

export async function getDocumentsByProfile(profileName: string): Promise<Document[]> {
  return await invoke('get_documents_by_profile', { profileName });
}

export async function getCvProfiles(): Promise<string[]> {
  return await invoke('get_cv_profiles');
}

export async function getGoogleCalendarAccounts(): Promise<GoogleCalendarAccount[]> {
  return await invoke('get_google_calendar_accounts');
}

export async function saveGoogleCalendarAccounts(accounts: GoogleCalendarAccount[]): Promise<void> {
  return await invoke('save_google_calendar_accounts', { accounts });
}

export async function getGoogleOauthConfig(): Promise<GoogleOAuthConfig> {
  return await invoke('get_google_oauth_config');
}

export async function saveGoogleOauthConfig(config: GoogleOAuthConfig): Promise<void> {
  return await invoke('save_google_oauth_config', { config });
}

export async function connectGoogleCalendarAccount(): Promise<GoogleCalendarAccount> {
  return await invoke('connect_google_calendar_account');
}

export async function deleteGoogleCalendarAccount(email: string): Promise<void> {
  return await invoke('delete_google_calendar_account', { email });
}

export async function syncUnifiedCalendarEvents(): Promise<UnifiedCalendarEvent[]> {
  return await invoke('sync_unified_calendar_events');
}

export async function getUnifiedCalendarSettings(): Promise<UnifiedCalendarSettings> {
  return await invoke('get_unified_calendar_settings');
}

export async function saveUnifiedCalendarSettings(settings: UnifiedCalendarSettings): Promise<void> {
  return await invoke('save_unified_calendar_settings', { settings });
}

export async function getUnifiedCalendarEvents(): Promise<UnifiedCalendarEvent[]> {
  return await invoke('get_unified_calendar_events');
}

export async function saveUnifiedCalendarEvents(events: UnifiedCalendarEvent[]): Promise<void> {
  return await invoke('save_unified_calendar_events', { events });
}

export async function openExternalUrl(url: string): Promise<void> {
  return await invoke('open_external_url', { url });
}

export async function generateCoverLetter(data: GenerateLetterInput): Promise<GeneratedLetter> {
  return await invoke('generate_cover_letter', { data });
}

export async function getGeminiConfig(): Promise<GeminiConfig> {
  return await invoke('get_gemini_config');
}

export async function saveGeminiConfig(config: GeminiConfig): Promise<void> {
  return await invoke('save_gemini_config', { config });
}

// Browser API
