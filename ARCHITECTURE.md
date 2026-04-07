# Plan d'Implémentation - Boussole

## 📋 Vue d'ensemble

Application desktop Tauri + Svelte pour la gestion de recherche d'emploi avec stockage local SQLite.

---

## 🗄️ Schéma Base de Données

### Table `job_listings` (Annonces)

| Champ | Type | Description |
|-------|------|-------------|
| `id` | INTEGER PK | Identifiant unique |
| `company_name` | TEXT NOT NULL | Nom de la société |
| `title` | TEXT NOT NULL | Titre du poste |
| `location` | TEXT | Ville / région |
| `salary_min` | INTEGER | Salaire minimum |
| `salary_max` | INTEGER | Salaire maximum |
| `salary_currency` | TEXT | Devise (EUR par défaut) |
| `contract_type` | TEXT | CDI, CDD, freelance, stage, alternance |
| `remote_type` | TEXT | on_site, hybrid, full_remote |
| `stack` | TEXT | Technologies (JSON array) |
| `source_site` | TEXT | Site de parution (LinkedIn, Indeed, etc.) |
| `source_url` | TEXT | Lien original |
| `description` | TEXT | Description complète |
| `date_posted` | DATE | Date de publication de l'offre |
| `date_saved` | DATE | Date d'enregistrement |
| `status` | TEXT | saved, applied, expired, rejected |
| `search_vector` | TEXT | Index de recherche full-text |

### Table `applications` (Candidatures)

| Champ | Type | Description |
|-------|------|-------------|
| `id` | INTEGER PK | Identifiant unique |
| `job_listing_id` | INTEGER FK | Lien vers l'annonce (nullable) |
| `company_name` | TEXT NOT NULL | Nom de la société |
| `position` | TEXT NOT NULL | Intitulé du poste |
| `status` | TEXT | wishlist, applied, interview, offer, rejected, ghosted |
| `applied_date` | DATE | Date de candidature |
| `response_date` | DATE | Date de réponse |
| `notes` | TEXT | Notes personnelles |
| `contact_email` | TEXT | Email du recruteur |
| `contact_name` | TEXT | Nom du contact |
| `cv_version_id` | INTEGER FK | Version du CV utilisée |
| `cover_letter_id` | INTEGER FK | Lettre de motivation générée |

### Table `documents` (Documents)

| Champ | Type | Description |
|-------|------|-------------|
| `id` | INTEGER PK | Identifiant unique |
| `type` | TEXT | cv, cover_letter, other |
| `name` | TEXT | Nom du fichier |
| `content` | BLOB | Contenu binaire |
| `version` | INTEGER | Numéro de version |
| `created_at` | DATETIME | Date de création |
| `updated_at` | DATETIME | Date de modification |
| `is_template` | BOOLEAN | Modèle réutilisable |

### Table `interviews` (Entretiens)

| Champ | Type | Description |
|-------|------|-------------|
| `id` | INTEGER PK | Identifiant unique |
| `application_id` | INTEGER FK | Lien vers la candidature |
| `scheduled_at` | DATETIME | Date et heure |
| `type` | TEXT | phone, video, onsite |
| `location` | TEXT | Adresse ou lien visio |
| `duration_minutes` | INTEGER | Durée estimée |
| `preparation_notes` | TEXT | Notes de préparation |
| `google_event_id` | TEXT | ID de l'événement Google Calendar |
| `status` | TEXT | scheduled, completed, cancelled |

### Table `ai_generations` (Générations IA)

| Champ | Type | Description |
|-------|------|-------------|
| `id` | INTEGER PK | Identifiant unique |
| `application_id` | INTEGER FK | Lien vers la candidature |
| `type` | TEXT | cover_letter, interview_prep, offer_analysis |
| `prompt` | TEXT | Prompt utilisé |
| `result` | TEXT | Résultat généré |
| `model` | TEXT | Modèle Gemini utilisé |
| `created_at` | DATETIME | Date de génération |
| `tokens_used` | INTEGER | Tokens consommés |

### Table `activity_log` (Journal France Travail)

| Champ | Type | Description |
|-------|------|-------------|
| `id` | INTEGER PK | Identifiant unique |
| `date` | DATE | Date de l'activité |
| `type` | TEXT | search, apply, interview, training, other |
| `description` | TEXT | Description détaillée |
| `application_id` | INTEGER FK | Lien vers candidature si applicable |
| `duration_minutes` | INTEGER | Durée (pour les formations) |
| `exported` | BOOLEAN | Déjà inclus dans un rapport |

---

## 🔧 Architecture Tauri

### Commandes Rust (Backend)

```rust
// job_listings
#[tauri::command]
async fn create_job_listing(data: JobListingInput) -> Result<JobListing, Error>

#[tauri::command]
async fn search_job_listings(query: String, filters: JobFilters) -> Result<Vec<JobListing>, Error>

#[tauri::command]
async fn update_job_listing(id: i64, data: JobListingInput) -> Result<JobListing, Error>

#[tauri::command]
async fn delete_job_listing(id: i64) -> Result<(), Error>

#[tauri::command]
async fn check_duplicate_company(company_name: String) -> Result<Vec<Application>, Error>

// applications
#[tauri::command]
async fn create_application(data: ApplicationInput) -> Result<Application, Error>

#[tauri::command]
async fn link_application_to_listing(app_id: i64, listing_id: i64) -> Result<(), Error>

// documents
#[tauri::command]
async fn save_document(data: DocumentInput) -> Result<Document, Error>

#[tauri::command]
async fn get_cv_versions() -> Result<Vec<Document>, Error>

// ai
#[tauri::command]
async fn generate_cover_letter(application_id: i64, tone: String) -> Result<AIGeneration, Error>

#[tauri::command]
async fn analyze_job_match(listing_id: i64, cv_id: i64) -> Result<MatchScore, Error>

// calendar
#[tauri::command]
async fn sync_google_calendar() -> Result<Vec<CalendarEvent>, Error>

#[tauri::command]
async fn create_interview_event(interview: InterviewInput) -> Result<Interview, Error>

// exports
#[tauri::command]
async fn generate_ft_report(start_date: Date, end_date: Date) -> Result<Vec<u8>, Error>

#[tauri::command]
async fn export_all_data() -> Result<Vec<u8>, Error>
```

### Structure des Modèles

```rust
// src/models/job_listing.rs
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobListing {
    pub id: i64,
    pub company_name: String,
    pub title: String,
    pub location: Option<String>,
    pub salary_min: Option<i32>,
    pub salary_max: Option<i32>,
    pub contract_type: ContractType,
    pub remote_type: RemoteType,
    pub stack: Vec<String>,
    pub source_site: String,
    pub source_url: String,
    pub description: String,
    pub status: ListingStatus,
    pub date_posted: Option<NaiveDate>,
    pub date_saved: NaiveDateTime,
}

// src/models/application.rs  
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    pub id: i64,
    pub job_listing_id: Option<i64>,
    pub company_name: String,
    pub position: String,
    pub status: ApplicationStatus,
    pub applied_date: NaiveDate,
    pub notes: Option<String>,
    pub has_existing_application: bool, // calculé
}
```

---

## 🎨 Composants Svelte (Frontend)

### Structure des routes

```
src/
├── routes/
│   ├── +layout.svelte          # Navigation principale
│   ├── +page.svelte            # Dashboard
│   ├── listings/
│   │   ├── +page.svelte        # Liste des annonces
│   │   ├── +page.ts            # Load data
│   │   ├── [id]/
│   │   │   ├── +page.svelte    # Détail annonce
│   │   │   └── +page.ts
│   │   └── new/
│   │       └── +page.svelte    # Nouvelle annonce
│   ├── applications/
│   │   ├── +page.svelte        # Kanban des candidatures
│   │   └── [id]/
│   │       └── +page.svelte    # Détail candidature
│   ├── calendar/
│   │   └── +page.svelte        # Vue agenda
│   ├── documents/
│   │   └── +page.svelte        # Gestion documents
│   ├── ai/
│   │   └── +page.svelte        # Assistant IA
│   └── settings/
│       └── +page.svelte        # Paramètres
├── lib/
│   ├── components/
│   │   ├── kanban/
│   │   ├── forms/
│   │   ├── search/
│   │   └── calendar/
│   ├── stores/
│   │   ├── listings.ts         # Store des annonces
│   │   ├── applications.ts     # Store des candidatures
│   │   └── settings.ts
│   └── utils/
│       ├── tauri.ts            # Invokes Tauri
│       └── validators.ts
```

### Stores clés

```typescript
// src/lib/stores/listings.ts
interface ListingFilters {
  company?: string;
  contractTypes?: ContractType[];
  remoteTypes?: RemoteType[];
  stack?: string[];
  salaryMin?: number;
  hasApplication?: boolean;
}

interface ListingSearch {
  query: string;
  filters: ListingFilters;
  results: JobListing[];
  isLoading: boolean;
}

export const listingSearch = writable<ListingSearch>({
  query: '',
  filters: {},
  results: [],
  isLoading: false
});

// Search avec debounce
export async function searchListings(query: string, filters: ListingFilters) {
  listingSearch.update(s => ({ ...s, isLoading: true }));
  const results = await invoke('search_job_listings', { 
    query, 
    filters: camelCaseKeys(filters) 
  });
  listingSearch.update(s => ({ ...s, results, isLoading: false }));
}
```

---

## 🚀 Phases de Développement

### Phase 1 : Fondation (Semaine 1)
- [ ] Setup projet Tauri + Svelte
- [ ] Configuration base SQLite + migrations sqlx
- [ ] Modèles de base (job_listings, applications)
- [ ] Structure UI (layout, navigation)

### Phase 2 : Gestion des Annonces (Semaine 2)
- [ ] CRUD annonces complet
- [ ] Formulaire avec tous les champs
- [ ] Recherche full-text avec filtres
- [ ] Détection doublons société
- [ ] Import basique (copier-coller URL)

### Phase 3 : Kanban Candidatures (Semaine 3)
- [ ] Tableau Kanban drag & drop
- [ ] Liaison annonces → candidatures
- [ ] Alertes doublons société
- [ ] Historique des actions

### Phase 4 : Documents & IA (Semaine 4)
- [ ] Upload/versioning CV
- [ ] Intégration Gemini API
- [ ] Génération lettres de motivation
- [ ] Analyse matching offre/CV

### Phase 5 : Calendar & Export (Semaine 5)
- [ ] OAuth Google Calendar
- [ ] Synchronisation entretiens
- [ ] Génération rapports PDF France Travail
- [ ] Export données complètes

### Phase 6 : Polish & Distribution (Semaine 6)
- [ ] Tests E2E
- [ ] Optimisations performances
- [ ] Builds Windows/macOS/Linux
- [ ] Documentation utilisateur

---

## 📡 API Externes

### Google Gemini
- **Endpoint** : `https://generativelanguage.googleapis.com/v1beta`
- **Usage** : Génération lettres, analyse offres, préparation entretiens
- **Clé** : Stockée chiffrée dans les settings utilisateur

### Google Calendar
- **OAuth 2.0** : Scopes `calendar.events.readonly`, `calendar.events`
- **Usage** : Lire/synchroniser les entretiens

---

## 🔒 Sécurité & RGPD

### Chiffrement
- Base SQLite chiffrée via SQLCipher
- Clé dérivée du mot de passe utilisateur (PBKDF2)

### Données locales uniquement
- Aucun serveur backend propre
- API externes appelées directement depuis le client
- Cache offline complet

### Export/Suppression
- Export JSON complet de toutes les données
- Suppression irréversible (shred des fichiers)
- Historique des accès (audit log)

---

## 📝 Notes Techniques

### Conventions de code

**Frontend (Svelte)** :
- Props camelCase : `companyName`
- Stores fichiers séparés
- Validation Zod pour les formulaires

**Backend (Rust)** :
- Commands camelCase via serde
- Erreurs centralisées dans `src/error.rs`
- Transactions SQLx pour opérations multiples

**Base de données** :
- snake_case pour les colonnes
- Timestamps UTC stockés, affichage local
- Full-text search via FTS5 SQLite

### Performance
- Pagination 50 items par défaut
- Virtual scrolling pour les listes longues
- Index sur `company_name`, `date_saved`, `status`
- Lazy loading des descriptions longues
