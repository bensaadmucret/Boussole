<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { ArrowLeft, Edit2, Trash2, Building2, MapPin, Banknote, Briefcase, Calendar, ExternalLink, Tag } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { getJobListings, deleteJobListing, checkDuplicateCompany, getExistingCompanyListings } from '$lib/utils/tauri';
  import { listings } from '$lib/stores/listings';
  import type { JobListing } from '$lib/types';
  
  let id = $derived(parseInt($page.params.id ?? '0'));
  let listing = $derived($listings.find(l => l.id === id));
  let duplicates = $state<JobListing[]>([]);
  let showDeleteConfirm = $state(false);
  
  onMount(async () => {
    if ($listings.length === 0) {
      const data = await getJobListings();
      listings.set(data);
    }
    
    if (listing) {
      const hasDupes = await checkDuplicateCompany(listing.companyName);
      if (hasDupes) {
        duplicates = await getExistingCompanyListings(listing.companyName);
        duplicates = duplicates.filter(d => d.id !== id);
      }
    }
  });
  
  function goBack() {
    goto('/listings');
  }
  
  function formatSalary(min?: number, max?: number) {
    if (!min && !max) return 'Non précisé';
    if (min && max) return `${min.toLocaleString()} - ${max.toLocaleString()} €`;
    return min ? `À partir de ${min.toLocaleString()} €` : `Jusqu'à ${max?.toLocaleString()} €`;
  }
  
  function getRemoteLabel(value: string) {
    const labels: Record<string, string> = {
      'on_site': 'Sur site',
      'hybrid': 'Hybride',
      'full_remote': 'Full remote'
    };
    return labels[value] || value;
  }
  
  async function handleDelete() {
    try {
      await deleteJobListing(id);
      listings.set($listings.filter(l => l.id !== id));
      goto('/listings');
    } catch (err) {
      console.error('Failed to delete:', err);
    }
  }
</script>

<div class="px-8 py-6 max-w-4xl">
  {#if !listing}
    <div class="text-center py-12">
      <p class="text-gray-500">Annonce non trouvée</p>
      <button onclick={goBack} class="btn-primary mt-4">Retour aux annonces</button>
    </div>
  {:else}
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <div class="flex items-center gap-4">
        <button 
          onclick={goBack}
          class="p-2 text-gray-500 hover:text-primary-600 hover:bg-primary-50 rounded-xl transition"
          aria-label="Retour"
        >
          <ArrowLeft class="w-6 h-6" />
        </button>
        <div>
          <h1 class="text-3xl font-bold text-gray-900">{listing.title}</h1>
          <div class="flex items-center gap-2 text-gray-500 mt-1">
            <Building2 class="w-4 h-4" />
            <span>{listing.companyName}</span>
          </div>
        </div>
      </div>
      
      <div class="flex items-center gap-2">
        <button class="btn-secondary flex items-center gap-2">
          <Edit2 class="w-4 h-4" />
          <span>Modifier</span>
        </button>
        <button 
          onclick={() => showDeleteConfirm = true}
          class="px-4 py-2 bg-red-100 text-red-700 rounded-xl font-medium hover:bg-red-200 transition flex items-center gap-2"
        >
          <Trash2 class="w-4 h-4" />
          <span>Supprimer</span>
        </button>
      </div>
    </div>
    
    <!-- Alert doublons -->
    {#if duplicates.length > 0}
      <div class="mb-6 p-4 bg-amber-50 border border-amber-200 rounded-2xl">
        <div class="flex items-start gap-3">
          <div class="w-8 h-8 bg-amber-100 rounded-xl flex items-center justify-center text-amber-600 shrink-0">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/></svg>
          </div>
          <div>
            <h3 class="font-semibold text-amber-900">Doublon détecté</h3>
            <p class="text-amber-700 text-sm mt-1">{duplicates.length} autre(s) annonce(s) de {listing.companyName} existent déjà</p>
            <div class="mt-3 space-y-2">
              {#each duplicates as dup}
                <a 
                  href={`/listings/${dup.id}`}
                  class="block p-3 bg-white rounded-xl hover:bg-amber-100 transition text-sm"
                >
                  <span class="font-medium">{dup.title}</span>
                  <span class="text-gray-500 ml-2">- {new Date(dup.dateSaved).toLocaleDateString('fr-FR')}</span>
                </a>
              {/each}
            </div>
          </div>
        </div>
      </div>
    {/if}
    
    <!-- Détails -->
    <div class="grid grid-cols-2 gap-6">
      <div class="card">
        <h2 class="font-bold text-lg text-gray-900 mb-4">Informations</h2>
        <div class="space-y-4">
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 bg-primary-100 rounded-xl flex items-center justify-center">
              <Briefcase class="w-5 h-5 text-primary-600" />
            </div>
            <div>
              <p class="text-sm text-gray-500">Type de contrat</p>
              <p class="font-medium">{listing.contractType}</p>
            </div>
          </div>
          
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 bg-primary-100 rounded-xl flex items-center justify-center">
              <MapPin class="w-5 h-5 text-primary-600" />
            </div>
            <div>
              <p class="text-sm text-gray-500">Localisation</p>
              <p class="font-medium">{listing.location || 'Non précisée'} • {getRemoteLabel(listing.remoteType)}</p>
            </div>
          </div>
          
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 bg-primary-100 rounded-xl flex items-center justify-center">
              <Banknote class="w-5 h-5 text-primary-600" />
            </div>
            <div>
              <p class="text-sm text-gray-500">Salaire</p>
              <p class="font-medium">{formatSalary(listing.salaryMin, listing.salaryMax)}</p>
            </div>
          </div>
          
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 bg-primary-100 rounded-xl flex items-center justify-center">
              <Calendar class="w-5 h-5 text-primary-600" />
            </div>
            <div>
              <p class="text-sm text-gray-500">Date de publication</p>
              <p class="font-medium">{listing.datePosted ? new Date(listing.datePosted).toLocaleDateString('fr-FR') : 'Non précisée'}</p>
            </div>
          </div>
        </div>
      </div>
      
      <div class="space-y-6">
        <div class="card">
          <h2 class="font-bold text-lg text-gray-900 mb-4">Source</h2>
          <div class="space-y-3">
            <div>
              <p class="text-sm text-gray-500">Site de parution</p>
              <p class="font-medium">{listing.sourceSite}</p>
            </div>
            {#if listing.sourceUrl}
              <a 
                href={listing.sourceUrl}
                target="_blank"
                rel="noopener"
                class="inline-flex items-center gap-2 text-primary-600 hover:text-primary-700 font-medium"
              >
                <span>Voir l'offre originale</span>
                <ExternalLink class="w-4 h-4" />
              </a>
            {/if}
          </div>
        </div>
        
        {#if listing.stack && listing.stack.length > 0}
          <div class="card">
            <h2 class="font-bold text-lg text-gray-900 mb-4 flex items-center gap-2">
              <Tag class="w-5 h-5" />
              Stack technique
            </h2>
            <div class="flex flex-wrap gap-2">
              {#each listing.stack as tech}
                <span class="px-3 py-1 bg-primary-100 text-primary-800 rounded-xl text-sm font-medium">{tech}</span>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    </div>
    
    <!-- Description -->
    {#if listing.description}
      <div class="card mt-6">
        <h2 class="font-bold text-lg text-gray-900 mb-4">Description</h2>
        <div class="prose prose-sm max-w-none text-gray-700 whitespace-pre-wrap">{listing.description}</div>
      </div>
    {/if}
    
    <!-- Modal confirmation suppression -->
    {#if showDeleteConfirm}
      <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
        <div class="bg-white rounded-2xl p-6 max-w-md w-full mx-4">
          <h3 class="text-lg font-bold text-gray-900 mb-2">Confirmer la suppression</h3>
          <p class="text-gray-500 mb-6">Êtes-vous sûr de vouloir supprimer cette annonce ? Cette action est irréversible.</p>
          <div class="flex justify-end gap-3">
            <button 
              onclick={() => showDeleteConfirm = false}
              class="px-4 py-2 text-gray-600 font-medium hover:text-gray-800 transition"
            >
              Annuler
            </button>
            <button 
              onclick={handleDelete}
              class="px-4 py-2 bg-red-600 text-white rounded-xl font-medium hover:bg-red-700 transition"
            >
              Supprimer
            </button>
          </div>
        </div>
      </div>
    {/if}
  {/if}
</div>
