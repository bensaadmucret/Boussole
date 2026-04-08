<script lang="ts">
  import { onMount } from 'svelte';
  import { Plus, FileText, Trash2, Upload, User, File as FileIcon } from 'lucide-svelte';
  import { getDocuments, getCvProfiles, deleteDocument, createDocument } from '$lib/utils/tauri';
  import { documents, cvProfiles } from '$lib/stores/documents';
  import type { Document } from '$lib/types';
  
  let activeTab = $state<'cv' | 'letters'>('cv');
  let showUploadModal = $state(false);
  let uploading = $state(false);
  let newProfileName = $state('');
  let selectedFile: globalThis.File | null = $state(null);
  
  let cvDocuments = $derived($documents.filter(d => d.docType === 'cv'));
  let letterDocuments = $derived($documents.filter(d => d.docType === 'cover_letter'));
  
  onMount(async () => {
    await loadData();
  });
  
  async function loadData() {
    try {
      const [docs, profiles] = await Promise.all([
        getDocuments(),
        getCvProfiles()
      ]);
      documents.set(docs);
      cvProfiles.set(profiles);
    } catch (err) {
      console.error('Failed to load documents:', err);
    }
  }
  
  async function handleFileSelect(e: Event) {
    const input = e.target as HTMLInputElement;
    if (input.files && input.files[0]) {
      selectedFile = input.files[0];
    }
  }
  
  async function handleUpload() {
    console.log('handleUpload called');
    console.log('selectedFile:', selectedFile);
    console.log('newProfileName:', newProfileName);
    console.log('activeTab:', activeTab);
    
    if (!selectedFile) {
      console.error('No file selected');
      alert('Veuillez sélectionner un fichier');
      return;
    }
    
    if (activeTab === 'cv' && !newProfileName.trim()) {
      console.error('No profile name for CV');
      alert('Veuillez entrer un nom de profil');
      return;
    }
    
    console.log('Starting upload...', selectedFile.name, 'Size:', selectedFile.size);
    uploading = true;
    
    try {
      console.log('Reading file as ArrayBuffer...');
      const arrayBuffer = await selectedFile.arrayBuffer();
      console.log('ArrayBuffer loaded, size:', arrayBuffer.byteLength);
      
      console.log('Converting to Uint8Array...');
      const uint8Array = new Uint8Array(arrayBuffer);
      console.log('Uint8Array created, length:', uint8Array.length);
      
      console.log('Converting to regular array...');
      const content = Array.from(uint8Array);
      console.log('Content array created, length:', content.length);
      
      const docData = {
        docType: activeTab === 'cv' ? 'cv' : 'cover_letter',
        profileName: activeTab === 'cv' ? newProfileName.trim() : undefined,
        name: selectedFile.name,
        content
      };
      console.log('Sending document data:', { ...docData, content: '[...' + content.length + ' bytes]' });
      
      console.log('Calling createDocument...');
      const result = await createDocument(docData);
      console.log('Document created successfully:', result);
      
      console.log('Reloading data...');
      await loadData();
      
      console.log('Closing modal...');
      showUploadModal = false;
      selectedFile = null;
      newProfileName = '';
      
      alert('Document uploadé avec succès !');
    } catch (err) {
      console.error('Failed to upload:', err);
      console.error('Error type:', typeof err);
      console.error('Error message:', err instanceof Error ? err.message : String(err));
      console.error('Error stack:', err instanceof Error ? err.stack : 'No stack');
      alert('Erreur lors de l\'upload: ' + (err instanceof Error ? err.message : String(err)));
    } finally {
      console.log('Upload finished, uploading = false');
      uploading = false;
    }
  }
  
  async function handleDelete(doc: Document) {
    if (!confirm(`Supprimer ${doc.name} ?`)) return;
    
    try {
      await deleteDocument(doc.id);
      await loadData();
    } catch (err) {
      console.error('Failed to delete:', err);
    }
  }
  
  function formatSize(bytes: number): string {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
    return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
  }
  
  function formatDate(dateStr: string): string {
    return new Date(dateStr).toLocaleDateString('fr-FR', { day: 'numeric', month: 'short', year: 'numeric' });
  }
</script>

<div class="px-8 py-6">
  <!-- Header -->
  <div class="flex justify-between items-center mb-6">
    <div>
      <h1 class="text-3xl font-bold text-gray-900">Documents</h1>
      <p class="text-gray-500 mt-1">Gérez vos CV et lettres de motivation</p>
    </div>
    <button 
      onclick={() => showUploadModal = true}
      class="btn-primary flex items-center gap-2"
    >
      <Plus class="w-5 h-5" />
      <span>Ajouter un document</span>
    </button>
  </div>
  
  <!-- Tabs -->
  <div class="flex gap-2 mb-6">
    <button 
      onclick={() => activeTab = 'cv'}
      class="px-4 py-2 rounded-xl font-medium transition {activeTab === 'cv' ? 'bg-primary-100 text-primary-700' : 'text-gray-500 hover:text-gray-700'}"
    >
      <span class="flex items-center gap-2">
        <User class="w-4 h-4" />
        CV par profil
        <span class="px-2 py-0.5 bg-gray-200 text-gray-600 rounded-lg text-xs">{cvDocuments.length}</span>
      </span>
    </button>
    <button 
      onclick={() => activeTab = 'letters'}
      class="px-4 py-2 rounded-xl font-medium transition {activeTab === 'letters' ? 'bg-primary-100 text-primary-700' : 'text-gray-500 hover:text-gray-700'}"
    >
      <span class="flex items-center gap-2">
        <FileText class="w-4 h-4" />
        Lettres de motivation
        <span class="px-2 py-0.5 bg-gray-200 text-gray-600 rounded-lg text-xs">{letterDocuments.length}</span>
      </span>
    </button>
  </div>
  
  <!-- Content -->
  {#if activeTab === 'cv'}
    <!-- CV Profiles -->
    <div class="space-y-6">
      {#if $cvProfiles.length === 0}
        <div class="card p-12 text-center">
          <div class="w-16 h-16 bg-primary-100 rounded-2xl flex items-center justify-center mx-auto mb-4">
            <User class="w-8 h-8 text-primary-600" />
          </div>
          <h3 class="text-lg font-semibold text-gray-900 mb-2">Aucun CV</h3>
          <p class="text-gray-500 mb-4">Ajoutez votre premier CV pour un profil spécifique</p>
          <button 
            onclick={() => { activeTab = 'cv'; showUploadModal = true; }}
            class="btn-primary inline-flex items-center gap-2"
          >
            <Upload class="w-4 h-4" />
            <span>Uploader un CV</span>
          </button>
        </div>
      {:else}
        {#each $cvProfiles as profile}
          {@const profileDocs = cvDocuments.filter(d => d.profileName === profile)}
          <div class="card">
            <div class="flex items-center justify-between mb-4">
              <h2 class="font-bold text-xl text-gray-900 flex items-center gap-2">
                <div class="w-10 h-10 bg-primary-200 rounded-xl flex items-center justify-center">
                  <User class="w-5 h-5 text-primary-700" />
                </div>
                {profile}
              </h2>
              <span class="text-sm text-gray-500">{profileDocs.length} document{profileDocs.length > 1 ? 's' : ''}</span>
            </div>
            
            <div class="space-y-2">
              {#each profileDocs as doc}
                <div class="flex items-center justify-between p-3 bg-gray-50 rounded-xl group">
                  <div class="flex items-center gap-3">
                    <FileIcon class="w-8 h-8 text-primary-600" />
                    <div>
                      <p class="font-medium text-gray-900">{doc.name}</p>
                      <p class="text-xs text-gray-500">v{doc.version} • {formatDate(doc.updatedAt)}</p>
                    </div>
                  </div>
                  <button 
                    onclick={() => handleDelete(doc)}
                    class="opacity-0 group-hover:opacity-100 p-2 text-gray-400 hover:text-red-600 transition"
                    aria-label="Supprimer"
                  >
                    <Trash2 class="w-4 h-4" />
                  </button>
                </div>
              {/each}
            </div>
          </div>
        {/each}
      {/if}
    </div>
  {:else}
    <!-- Cover Letters -->
    <div class="card">
      <div class="flex items-center justify-between mb-4">
        <h2 class="font-bold text-xl text-gray-900">Lettres de motivation</h2>
        <span class="text-sm text-gray-500">{letterDocuments.length} document{letterDocuments.length > 1 ? 's' : ''}</span>
      </div>
      
      {#if letterDocuments.length === 0}
        <div class="text-center py-12">
          <FileText class="w-12 h-12 mx-auto mb-3 text-gray-300" />
          <p class="text-gray-500">Aucune lettre de motivation</p>
          <p class="text-sm text-gray-400 mt-1">Générez des lettres depuis le détail d'une annonce</p>
        </div>
      {:else}
        <div class="space-y-2">
          {#each letterDocuments as doc}
            <div class="flex items-center justify-between p-3 bg-gray-50 rounded-xl group">
              <div class="flex items-center gap-3">
                <FileText class="w-8 h-8 text-primary-600" />
                <div>
                  <p class="font-medium text-gray-900">{doc.name}</p>
                  <p class="text-xs text-gray-500">{formatDate(doc.updatedAt)}</p>
                </div>
              </div>
              <button 
                onclick={() => handleDelete(doc)}
                class="opacity-0 group-hover:opacity-100 p-2 text-gray-400 hover:text-red-600 transition"
                aria-label="Supprimer"
              >
                <Trash2 class="w-4 h-4" />
              </button>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
  
  <!-- Upload Modal -->
  {#if showUploadModal}
    <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div class="bg-white rounded-2xl p-6 max-w-md w-full mx-4">
        <h3 class="text-lg font-bold text-gray-900 mb-4">
          Ajouter {activeTab === 'cv' ? 'un CV' : 'une lettre de motivation'}
        </h3>
        
        {#if activeTab === 'cv'}
          <div class="mb-4">
            <label for="document-profile-name" class="block text-sm font-medium text-gray-700 mb-1">Nom du profil</label>
            <input
              id="document-profile-name"
              type="text"
              bind:value={newProfileName}
              placeholder="Ex: Lead Dev, Chef de projet..."
              class="input-field"
            />
          </div>
        {/if}
        
        <div class="mb-6">
          <label for="document-file-input" class="block text-sm font-medium text-gray-700 mb-1">Fichier PDF</label>
          <input
            id="document-file-input"
            type="file"
            accept=".pdf,.doc,.docx"
            onchange={handleFileSelect}
            class="block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-xl file:border-0 file:bg-primary-100 file:text-primary-700 file:font-medium hover:file:bg-primary-200"
          />
          {#if activeTab === 'cv'}
            <p class="mt-2 text-sm text-amber-600">⚠️ Le nom du profil est obligatoire pour les CV</p>
          {/if}
        </div>
        
        <div class="flex justify-end gap-3">
          <button 
            onclick={() => { showUploadModal = false; selectedFile = null; newProfileName = ''; }}
            class="px-4 py-2 text-gray-600 font-medium hover:text-gray-800 transition"
          >
            Annuler
          </button>
          <button 
            onclick={handleUpload}
            disabled={!selectedFile || uploading || (activeTab === 'cv' && !newProfileName)}
            class="btn-primary flex items-center gap-2 disabled:opacity-50"
          >
            {#if uploading}
              <svg class="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
              <span>Upload...</span>
            {:else}
              <Upload class="w-4 h-4" />
              <span>Uploader</span>
            {/if}
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>
