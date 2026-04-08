<script lang="ts">
  import { Plus, Calendar, Building2, MoreHorizontal, Trash2, XCircle } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { getApplications, createApplication } from '$lib/utils/tauri';
  import { applications } from '$lib/stores/applications';
  import type { Application } from '$lib/types';
  
  let showNewModal = $state(false);
  let creating = $state(false);
  let newApp = $state({
    companyName: '',
    position: '',
    status: 'applied',
    appliedDate: new Date().toISOString().split('T')[0],
    notes: '',
    contactEmail: '',
    contactName: ''
  });
  
  const columns = [
    { id: 'applied', label: 'Candidatures envoyées', color: 'bg-blue-100 text-blue-700' },
    { id: 'phone', label: 'Entretien téléphonique', color: 'bg-purple-100 text-purple-700' },
    { id: 'interview', label: 'Entretien technique', color: 'bg-orange-100 text-orange-700' },
    { id: 'final', label: 'Entretien final', color: 'bg-pink-100 text-pink-700' },
    { id: 'offer', label: 'Offre reçue', color: 'bg-green-100 text-green-700' },
    { id: 'rejected', label: 'Refusé', color: 'bg-gray-100 text-gray-700' },
    { id: 'withdrawn', label: 'Retiré', color: 'bg-red-100 text-red-700' }
  ];
  
  let draggedApp = $state<Application | null>(null);
  let draggedFrom = $state<string>('');
  let openMenuId = $state<number | null>(null);
  
  function toggleMenu(appId: number) {
    openMenuId = openMenuId === appId ? null : appId;
  }
  
  function closeMenu() {
    openMenuId = null;
  }
  
  function moveToStatus(app: Application, newStatus: string) {
    applications.update(apps => 
      apps.map(a => a.id === app.id ? { ...a, status: newStatus } : a)
    );
    closeMenu();
  }
  
  function deleteApplication(app: Application) {
    applications.update(apps => apps.filter(a => a.id !== app.id));
    closeMenu();
  }
  
  onMount(async () => {
    // Always refresh applications to ensure newly created ones appear
    const data = await getApplications();
    applications.set(data);
  });
  
  function getApplicationsByStatus(status: string) {
    return $applications.filter(a => a.status === status);
  }
  
  function handleDragStart(e: DragEvent, app: Application, fromColumn: string) {
    draggedApp = app;
    draggedFrom = fromColumn;
    e.dataTransfer?.setData('text/plain', String(app.id));
    e.dataTransfer!.effectAllowed = 'move';
  }
  
  function handleDragOver(e: DragEvent) {
    e.preventDefault();
    e.dataTransfer!.dropEffect = 'move';
  }
  
  async function handleDrop(e: DragEvent, toColumn: string) {
    e.preventDefault();
    if (!draggedApp || draggedFrom === toColumn) return;
    
    applications.update(apps => 
      apps.map(a => a.id === draggedApp!.id ? { ...a, status: toColumn } : a)
    );
    
    draggedApp = null;
    draggedFrom = '';
  }
  
  function getDaysSince(dateStr: string) {
    const days = Math.floor((Date.now() - new Date(dateStr).getTime()) / (1000 * 60 * 60 * 24));
    return days === 0 ? 'Aujourd\'hui' : days === 1 ? 'Hier' : `Il y a ${days} jours`;
  }
  async function handleCreate() {
    if (!newApp.companyName || !newApp.position) return;
    
    creating = true;
    try {
      const created = await createApplication({
        ...newApp,
        jobListingId: null
      });
      applications.update(apps => [created, ...apps]);
      showNewModal = false;
      newApp = {
        companyName: '',
        position: '',
        status: 'applied',
        appliedDate: new Date().toISOString().split('T')[0],
        notes: '',
        contactEmail: '',
        contactName: ''
      };
    } catch (err) {
      console.error('Failed to create application:', err);
    } finally {
      creating = false;
    }
  }
</script>

<div class="px-8 py-6 h-screen flex flex-col">
  <!-- Header -->
  <div class="flex justify-between items-center mb-6">
    <div>
      <h1 class="text-3xl font-bold text-gray-900">Candidatures</h1>
      <p class="text-gray-500 mt-1">{$applications.length} candidature{$applications.length > 1 ? 's' : ''}</p>
    </div>
    <button onclick={() => showNewModal = true} class="btn-primary flex items-center gap-2 cursor-pointer">
      <Plus class="w-5 h-5" />
      <span>Nouvelle candidature</span>
    </button>
  </div>
  
  <!-- Kanban Board -->
  <div class="flex-1 overflow-x-auto">
    <div class="flex gap-4 min-w-max h-full">
      {#each columns as column}
        <div 
          class="w-80 flex flex-col"
          ondragover={handleDragOver}
          ondrop={(e) => handleDrop(e, column.id)}
        >
          <!-- Column Header -->
          <div class="flex items-center justify-between mb-3">
            <div class="flex items-center gap-2">
              <span class="px-3 py-1 rounded-xl text-sm font-semibold {column.color}">
                {column.label}
              </span>
              <span class="text-sm text-gray-400 font-medium">
                {getApplicationsByStatus(column.id).length}
              </span>
            </div>
          </div>
          
          <!-- Cards -->
          <div class="flex-1 bg-gray-50/50 rounded-2xl p-3 space-y-3 overflow-y-auto">
            {#each getApplicationsByStatus(column.id) as app (app.id)}
              <div
                draggable="true"
                ondragstart={(e) => handleDragStart(e, app, column.id)}
                class="card p-4 cursor-move hover:shadow-md transition group"
              >
                <div class="flex items-start justify-between mb-3">
                  <div class="flex items-center gap-2">
                    <div class="w-10 h-10 bg-primary-100 rounded-xl flex items-center justify-center text-primary-700 font-bold text-sm">
                      {app.companyName[0]}
                    </div>
                    <div>
                      <h3 class="font-semibold text-gray-900 text-sm">{app.position}</h3>
                      <p class="text-xs text-gray-500 flex items-center gap-1">
                        <Building2 class="w-3 h-3" />
                        {app.companyName}
                      </p>
                    </div>
                  </div>
                  <div class="relative">
                    <button 
                      onclick={() => toggleMenu(app.id)}
                      class="opacity-0 group-hover:opacity-100 transition text-gray-400 hover:text-gray-600 p-1 hover:bg-gray-100 rounded-lg"
                    >
                      <MoreHorizontal class="w-4 h-4" />
                    </button>
                    
                    {#if openMenuId === app.id}
                      <div 
                        class="absolute right-0 top-8 bg-white rounded-xl shadow-lg border border-gray-100 py-1 min-w-[160px] z-20"
                        onclick={(e) => e.stopPropagation()}
                      >
                        <button 
                          onclick={() => moveToStatus(app, 'rejected')}
                          class="w-full px-3 py-2 text-left text-sm text-red-600 hover:bg-red-50 flex items-center gap-2"
                        >
                          <XCircle class="w-4 h-4" />
                          <span>Marquer refusé</span>
                        </button>
                        <div class="border-t border-gray-100 my-1"></div>
                        <button 
                          onclick={() => deleteApplication(app)}
                          class="w-full px-3 py-2 text-left text-sm text-gray-600 hover:bg-gray-50 flex items-center gap-2"
                        >
                          <Trash2 class="w-4 h-4" />
                          <span>Supprimer</span>
                        </button>
                      </div>
                    {/if}
                  </div>
                </div>
                
                {#if app.contactName}
                  <div class="flex items-center gap-2 text-xs text-gray-500 mb-2">
                    <span class="px-2 py-1 bg-primary-50 rounded-lg">{app.contactName}</span>
                  </div>
                {/if}
                
                <div class="flex items-center justify-between text-xs text-gray-400">
                  <span class="flex items-center gap-1">
                    <Calendar class="w-3 h-3" />
                    {getDaysSince(app.appliedDate)}
                  </span>
                  {#if app.responseDate}
                    <span class="text-green-600">Réponse le {new Date(app.responseDate).toLocaleDateString('fr-FR', { day: 'numeric', month: 'short' })}</span>
                  {/if}
                </div>
              </div>
            {/each}
            
            {#if getApplicationsByStatus(column.id).length === 0}
              <div class="h-24 border-2 border-dashed border-gray-200 rounded-xl flex items-center justify-center text-gray-400 text-sm">
                Glisser une candidature ici
              </div>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  </div>
</div>

<!-- Modal Nouvelle candidature -->
{#if showNewModal}
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
    <div class="bg-white rounded-2xl p-6 max-w-md w-full mx-4">
      <h3 class="text-lg font-bold text-gray-900 mb-4">Nouvelle candidature</h3>
      
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Entreprise *</label>
          <input 
            bind:value={newApp.companyName}
            class="input-field w-full"
            placeholder="Nom de l'entreprise"
          />
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Poste *</label>
          <input 
            bind:value={newApp.position}
            class="input-field w-full"
            placeholder="Titre du poste"
          />
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Statut</label>
          <select bind:value={newApp.status} class="input-field w-full">
            {#each columns as col}
              <option value={col.id}>{col.label}</option>
            {/each}
          </select>
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Date de candidature</label>
          <input 
            type="date"
            bind:value={newApp.appliedDate}
            class="input-field w-full"
          />
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Contact</label>
          <input 
            bind:value={newApp.contactName}
            class="input-field w-full mb-2"
            placeholder="Nom du contact"
          />
          <input 
            type="email"
            bind:value={newApp.contactEmail}
            class="input-field w-full"
            placeholder="Email du contact"
          />
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Notes</label>
          <textarea 
            bind:value={newApp.notes}
            class="input-field w-full h-20"
            placeholder="Notes personnelles..."
          />
        </div>
      </div>
      
      <div class="flex justify-end gap-3 mt-6">
        <button 
          onclick={() => showNewModal = false}
          class="px-4 py-2 text-gray-600 font-medium hover:text-gray-800 transition"
        >
          Annuler
        </button>
        <button 
          onclick={handleCreate}
          disabled={creating || !newApp.companyName || !newApp.position}
          class="btn-primary flex items-center gap-2 disabled:opacity-50"
        >
          {#if creating}
            <svg class="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
            <span>Création...</span>
          {:else}
            <Plus class="w-4 h-4" />
            <span>Créer</span>
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}
