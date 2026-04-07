<script lang="ts">
  import { Plus, Search, Filter, Building2, MapPin, Banknote, Briefcase } from 'lucide-svelte';
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import { getJobListings } from '$lib/utils/tauri';
  import { listings } from '$lib/stores/listings';
  import type { JobListing } from '$lib/types';
  
  let searchQuery = $state('');
  let selectedContract = $state('');
  let selectedRemote = $state('');
  
  const contractTypes = ['', 'CDI', 'CDD', 'Freelance', 'Stage', 'Alternance'];
  const remoteTypes = [
    { value: '', label: 'Tous' },
    { value: 'on_site', label: 'Sur site' },
    { value: 'hybrid', label: 'Hybride' },
    { value: 'full_remote', label: 'Full remote' }
  ];
  
  let filteredListings = $derived(() => {
    return ($listings as JobListing[]).filter((l: JobListing) => {
      const matchesSearch = !searchQuery || 
        l.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
        l.companyName.toLowerCase().includes(searchQuery.toLowerCase()) ||
        (l.stack && l.stack.some((s: string) => s.toLowerCase().includes(searchQuery.toLowerCase())));
      
      const matchesContract = !selectedContract || l.contractType === selectedContract;
      const matchesRemote = !selectedRemote || l.remoteType === selectedRemote;
      
      return matchesSearch && matchesContract && matchesRemote;
    });
  });
  
  onMount(async () => {
    try {
      const data = await getJobListings();
      listings.set(data);
    } catch (err) {
      console.error('Failed to load listings:', err);
    }
  });
  
  function goToNewListing() {
    goto('/listings/new');
  }
  
  function formatSalary(min?: number, max?: number) {
    if (!min && !max) return 'Non précisé';
    if (min && max) return `${min.toLocaleString()} - ${max.toLocaleString()} €`;
    return min ? `À partir de ${min.toLocaleString()} €` : `Jusqu'à ${max?.toLocaleString()} €`;
  }
  
  function getRemoteLabel(value: string) {
    const type = remoteTypes.find(t => t.value === value);
    return type?.label || value;
  }
</script>

<div class="px-8 py-6">
  <!-- Header -->
  <div class="flex justify-between items-center mb-6">
    <div>
      <h1 class="text-3xl font-bold text-gray-900">Annonces sauvegardées</h1>
      <p class="text-gray-500 mt-1">{$listings.length} annonce{$listings.length > 1 ? 's' : ''}</p>
    </div>
    <button onclick={goToNewListing} class="btn-primary flex items-center gap-2">
      <Plus class="w-5 h-5" />
      <span>Nouvelle annonce</span>
    </button>
  </div>
  
  <!-- Search & Filters -->
  <div class="card mb-6">
    <div class="flex gap-4">
      <div class="flex-1 relative">
        <Search class="w-5 h-5 absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" />
        <input
          type="text"
          bind:value={searchQuery}
          placeholder="Rechercher par société, titre, stack..."
          class="input-field pl-10"
        />
      </div>
      
      <select bind:value={selectedContract} class="input-field w-40">
        <option value="">Tous contrats</option>
        {#each contractTypes.slice(1) as type}
          <option value={type}>{type}</option>
        {/each}
      </select>
      
      <select bind:value={selectedRemote} class="input-field w-40">
        {#each remoteTypes as type}
          <option value={type.value}>{type.label}</option>
        {/each}
      </select>
    </div>
  </div>
  
  <!-- Listings Grid -->
  {#if $listings.length === 0}
    <div class="card p-12 text-center">
      <div class="w-16 h-16 bg-primary-100 rounded-2xl flex items-center justify-center mx-auto mb-4">
        <Briefcase class="w-8 h-8 text-primary-600" />
      </div>
      <h3 class="text-lg font-semibold text-gray-900 mb-2">Aucune annonce</h3>
      <p class="text-gray-500 mb-4">Commencez par sauvegarder votre première offre</p>
      <button onclick={goToNewListing} class="btn-primary inline-flex items-center gap-2">
        <Plus class="w-5 h-5" />
        <span>Ajouter une annonce</span>
      </button>
    </div>
  {:else if filteredListings().length === 0}
    <div class="card p-12 text-center text-gray-500">
      <Filter class="w-12 h-12 mx-auto mb-3 text-gray-300" />
      <p>Aucune annonce ne correspond aux filtres</p>
    </div>
  {:else}
    <div class="space-y-4">
      {#each filteredListings() as listing (listing.id)}
        <a 
          href={`/listings/${listing.id}`}
          class="card hover:shadow-md transition block no-underline text-inherit"
        >
          <div class="flex items-start gap-4">
            <div class="w-14 h-14 bg-primary-200 rounded-2xl flex items-center justify-center text-primary-700 font-bold text-xl shrink-0">
              {listing.companyName[0]}
            </div>
            
            <div class="flex-1 min-w-0">
              <div class="flex items-start justify-between gap-4">
                <div>
                  <div class="flex items-center gap-2 mb-1">
                    <h3 class="font-semibold text-gray-900">{listing.title}</h3>
                    <span class="px-2 py-1 bg-primary-100 text-primary-700 text-xs font-semibold rounded-xl">
                      {listing.contractType}
                    </span>
                  </div>
                  <div class="flex items-center gap-3 text-sm text-gray-500">
                    <span class="flex items-center gap-1">
                      <Building2 class="w-4 h-4" />
                      {listing.companyName}
                    </span>
                    {#if listing.location}
                      <span class="flex items-center gap-1">
                        <MapPin class="w-4 h-4" />
                        {listing.location}
                      </span>
                    {/if}
                    <span class="flex items-center gap-1">
                      <Banknote class="w-4 h-4" />
                      {formatSalary(listing.salaryMin, listing.salaryMax)}
                    </span>
                  </div>
                </div>
                
                <span class="text-xs text-gray-400">
                  {new Date(listing.dateSaved).toLocaleDateString('fr-FR')}
                </span>
              </div>
              
              {#if listing.stack && listing.stack.length > 0}
                <div class="flex flex-wrap gap-2 mt-3">
                  {#each listing.stack.slice(0, 5) as tech}
                    <span class="px-2 py-1 bg-gray-100 rounded-lg text-xs text-gray-600">{tech}</span>
                  {/each}
                  {#if listing.stack.length > 5}
                    <span class="px-2 py-1 text-xs text-gray-400">+{listing.stack.length - 5}</span>
                  {/if}
                </div>
              {/if}
              
              {#if listing.description}
                <p class="mt-3 text-sm text-gray-600 line-clamp-2">{listing.description}</p>
              {/if}
            </div>
          </div>
        </a>
      {/each}
    </div>
  {/if}
</div>
