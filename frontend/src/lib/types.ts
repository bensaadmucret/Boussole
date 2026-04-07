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
