export interface JobListing {
  id: number;
  companyName: string;
  title: string;
  location?: string;
  salaryMin?: number;
  salaryMax?: number;
  salaryCurrency: string;
  contractType: string;
  remoteType: string;
  stack: string[];
  sourceSite: string;
  sourceUrl: string;
  description: string;
  status: string;
  datePosted?: string;
  dateSaved: string;
}

export interface Application {
  id: number;
  jobListingId?: number;
  companyName: string;
  position: string;
  status: string;
  appliedDate: string;
  responseDate?: string;
  notes?: string;
  contactEmail?: string;
  contactName?: string;
  cvVersionId?: number;
  coverLetterId?: number;
}

export interface CreateApplicationInput {
  jobListingId?: number | null;
  companyName: string;
  position: string;
  status: string;
  appliedDate: string;
  notes?: string | null;
  contactEmail?: string | null;
  contactName?: string | null;
}

export interface CreateJobListingInput {
  companyName: string;
  title: string;
  location?: string | null;
  salaryMin?: number | null;
  salaryMax?: number | null;
  contractType: string;
  remoteType: string;
  stack: string[];
  sourceSite: string;
  sourceUrl: string;
  description: string;
  datePosted?: string | null;
}

export interface Document {
  id: number;
  docType: string;
  profileName?: string;
  name: string;
  content?: number[];
  version: number;
  createdAt: string;
  updatedAt: string;
}

export interface CreateDocumentInput {
  docType: string;
  profileName?: string;
  name: string;
  content: number[];
}

export interface GenerateLetterInput {
  jobTitle: string;
  companyName: string;
  jobDescription: string;
  cvContent: string;
  tone?: string;
}

export interface GeneratedLetter {
  content: string;
  tokensUsed: number;
}

export interface GoogleCalendarAccount {
  id: string;
  email: string;
  displayName: string;
  avatarColor: string;
  connectedAt: string;
  calendars: GoogleCalendarSource[];
}

export interface GoogleCalendarSource {
  id: string;
  name: string;
  color: string;
  active: boolean;
}

export interface GoogleOAuthConfig {
  clientId: string;
  clientSecret?: string;
}

export interface UnifiedCalendarSettings {
  enabled: boolean;
  showSourceLabels: boolean;
  colorEventsByAccount: boolean;
  syncWindowDays: number;
  refreshIntervalMinutes: number;
}

export interface CalendarEventSource {
  accountEmail: string;
  accountName: string;
  calendarId: string;
  calendarName: string;
  color: string;
}

export interface UnifiedCalendarEvent {
  id: string;
  title: string;
  start: string;
  end: string;
  location?: string;
  source: CalendarEventSource;
}
