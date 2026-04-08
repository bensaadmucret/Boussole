import { writable } from 'svelte/store';
import type { Document, GoogleCalendarAccount, UnifiedCalendarEvent, UnifiedCalendarSettings } from '../types';

export const documents = writable<Document[]>([]);
export const cvProfiles = writable<string[]>([]);

export const selectedCvProfile = writable<string>('');
export const selectedCoverLetter = writable<Document | null>(null);

export const googleCalendarAccounts = writable<GoogleCalendarAccount[]>([]);

export const unifiedCalendarSettings = writable<UnifiedCalendarSettings>({
  enabled: true,
  showSourceLabels: true,
  colorEventsByAccount: true,
  syncWindowDays: 30,
  refreshIntervalMinutes: 15
});

export const unifiedCalendarEvents = writable<UnifiedCalendarEvent[]>([]);
