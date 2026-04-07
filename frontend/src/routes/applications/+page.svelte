<script lang="ts">
  import { Plus, Calendar, Building2, MoreHorizontal } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { getApplications } from '$lib/utils/tauri';
  import { applications } from '$lib/stores/applications';
  import type { Application } from '$lib/types';
  
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
  
  onMount(async () => {
    if ($applications.length === 0) {
      const data = await getApplications();
      applications.set(data);
    }
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
</script>

<div class="px-8 py-6 h-screen flex flex-col">
  <!-- Header -->
  <div class="flex justify-between items-center mb-6">
    <div>
      <h1 class="text-3xl font-bold text-gray-900">Candidatures</h1>
      <p class="text-gray-500 mt-1">{$applications.length} candidature{$applications.length > 1 ? 's' : ''}</p>
    </div>
    <button class="btn-primary flex items-center gap-2">
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
                  <button class="opacity-0 group-hover:opacity-100 transition text-gray-400 hover:text-gray-600">
                    <MoreHorizontal class="w-4 h-4" />
                  </button>
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
