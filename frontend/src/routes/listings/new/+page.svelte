<script lang="ts">
  import { goto } from '$app/navigation';
  import { ArrowLeft, Save, Building2, MapPin, Banknote, Home, Tags, Link, FileText, Calendar } from 'lucide-svelte';
  import { createJobListing } from '$lib/utils/tauri';
  
  // État du formulaire avec Svelte 5 runes
  let formData = $state({
    companyName: '',
    title: '',
    location: '',
    salaryMin: null as number | null,
    salaryMax: null as number | null,
    contractType: 'CDI',
    remoteType: 'hybrid',
    stack: [] as string[],
    sourceSite: '',
    sourceUrl: '',
    description: '',
    datePosted: ''
  });
  
  let newTag = $state('');
  let isSubmitting = $state(false);
  let error = $state('');
  
  const contractTypes = ['CDI', 'CDD', 'Freelance', 'Stage', 'Alternance'];
  const remoteTypes = [
    { value: 'on_site', label: 'Sur site' },
    { value: 'hybrid', label: 'Hybride' },
    { value: 'full_remote', label: 'Full remote' }
  ];
  
  function addTag() {
    if (newTag.trim() && !formData.stack.includes(newTag.trim())) {
      formData.stack = [...formData.stack, newTag.trim()];
      newTag = '';
    }
  }
  
  function removeTag(tag: string) {
    formData.stack = formData.stack.filter(t => t !== tag);
  }
  
  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      e.preventDefault();
      addTag();
    }
  }
  
  async function handleSubmit(e: Event) {
    e.preventDefault();
    
    if (!formData.companyName || !formData.title) {
      error = 'Veuillez remplir les champs obligatoires (société et titre)';
      return;
    }
    
    isSubmitting = true;
    error = '';
    
    try {
      await createJobListing({
        companyName: formData.companyName,
        title: formData.title,
        location: formData.location || null,
        salaryMin: formData.salaryMin,
        salaryMax: formData.salaryMax,
        contractType: formData.contractType,
        remoteType: formData.remoteType,
        stack: formData.stack,
        sourceSite: formData.sourceSite || 'Autre',
        sourceUrl: formData.sourceUrl,
        description: formData.description,
        datePosted: formData.datePosted || null
      });
      
      goto('/listings');
    } catch (err) {
      error = 'Erreur lors de la sauvegarde: ' + err;
    } finally {
      isSubmitting = false;
    }
  }
  
  function goBack() {
    goto('/listings');
  }
</script>

<div class="px-8 py-6 max-w-4xl">
  <!-- Header -->
  <div class="flex items-center gap-4 mb-6">
    <button 
      onclick={goBack}
      class="p-2 text-gray-500 hover:text-primary-600 hover:bg-primary-50 rounded-xl transition"
      aria-label="Retour"
    >
      <ArrowLeft class="w-6 h-6" />
    </button>
    <div>
      <h1 class="text-3xl font-bold text-gray-900">Nouvelle annonce</h1>
      <p class="text-gray-500 mt-1">Enregistrez une offre d'emploi</p>
    </div>
  </div>
  
  {#if error}
    <div class="mb-6 p-4 bg-red-50 border border-red-200 rounded-2xl flex items-center gap-3 text-red-700">
      <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
      <span>{error}</span>
    </div>
  {/if}
  
  <form onsubmit={handleSubmit} class="space-y-6">
    <!-- Société et Titre -->
    <div class="card">
      <h2 class="font-bold text-lg text-gray-900 mb-4 flex items-center gap-2">
        <Building2 class="w-5 h-5 text-primary-500" />
        Informations principales
      </h2>
      
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">
            Nom de la société <span class="text-red-500">*</span>
          </label>
          <input
            type="text"
            bind:value={formData.companyName}
            class="input-field"
            placeholder="Ex: TechCorp"
            required
          />
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">
            Titre du poste <span class="text-red-500">*</span>
          </label>
          <input
            type="text"
            bind:value={formData.title}
            class="input-field"
            placeholder="Ex: Développeur Full Stack"
            required
          />
        </div>
      </div>
    </div>
    
    <!-- Localisation et Remote -->
    <div class="card">
      <h2 class="font-bold text-lg text-gray-900 mb-4 flex items-center gap-2">
        <MapPin class="w-5 h-5 text-primary-500" />
        Localisation
      </h2>
      
      <div class="grid grid-cols-2 gap-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Ville / Région</label>
          <input
            type="text"
            bind:value={formData.location}
            class="input-field"
            placeholder="Ex: Paris, Lyon, Remote..."
          />
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1 flex items-center gap-1">
            <Home class="w-4 h-4" />
            Télétravail
          </label>
          <select bind:value={formData.remoteType} class="input-field">
            {#each remoteTypes as type}
              <option value={type.value}>{type.label}</option>
            {/each}
          </select>
        </div>
      </div>
    </div>
    
    <!-- Salaire et Contrat -->
    <div class="card">
      <h2 class="font-bold text-lg text-gray-900 mb-4 flex items-center gap-2">
        <Banknote class="w-5 h-5 text-primary-500" />
        Rémunération et contrat
      </h2>
      
      <div class="grid grid-cols-3 gap-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Type de contrat</label>
          <select bind:value={formData.contractType} class="input-field">
            {#each contractTypes as type}
              <option value={type}>{type}</option>
            {/each}
          </select>
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Salaire min (€)</label>
          <input
            type="number"
            bind:value={formData.salaryMin}
            class="input-field"
            placeholder="40000"
          />
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Salaire max (€)</label>
          <input
            type="number"
            bind:value={formData.salaryMax}
            class="input-field"
            placeholder="60000"
          />
        </div>
      </div>
    </div>
    
    <!-- Stack technique -->
    <div class="card">
      <h2 class="font-bold text-lg text-gray-900 mb-4 flex items-center gap-2">
        <Tags class="w-5 h-5 text-primary-500" />
        Stack technique
      </h2>
      
      <div class="mb-3">
        <div class="flex gap-2">
          <input
            type="text"
            bind:value={newTag}
            onkeydown={handleKeydown}
            class="input-field flex-1"
            placeholder="Ajouter une technologie (ex: React, Rust, PostgreSQL...)"
          />
          <button
            type="button"
            onclick={addTag}
            class="px-4 py-2 bg-primary-100 text-primary-700 rounded-xl font-medium hover:bg-primary-200 transition"
          >
            +
          </button>
        </div>
      </div>
      
      <div class="flex flex-wrap gap-2">
        {#each formData.stack as tag}
          <span class="inline-flex items-center gap-1 px-3 py-1 bg-primary-100 text-primary-800 rounded-xl text-sm font-medium">
            {tag}
            <button
              type="button"
              onclick={() => removeTag(tag)}
              class="w-4 h-4 flex items-center justify-center rounded-full hover:bg-primary-200 transition"
              aria-label={`Retirer ${tag}`}
            >
              ×
            </button>
          </span>
        {/each}
      </div>
    </div>
    
    <!-- Source -->
    <div class="card">
      <h2 class="font-bold text-lg text-gray-900 mb-4 flex items-center gap-2">
        <Link class="w-5 h-5 text-primary-500" />
        Source de l'annonce
      </h2>
      
      <div class="grid grid-cols-2 gap-4">
        <div>
          <label for="sourceSite" class="block text-sm font-medium text-gray-700 mb-1">Site de parution</label>
          <input
            id="sourceSite"
            type="text"
            bind:value={formData.sourceSite}
            class="input-field"
            placeholder="Ex: LinkedIn, Indeed, Welcome to the Jungle..."
          />
        </div>
        
        <div>
          <label for="sourceUrl" class="block text-sm font-medium text-gray-700 mb-1">Lien de l'offre</label>
          <input
            id="sourceUrl"
            type="url"
            bind:value={formData.sourceUrl}
            class="input-field"
            placeholder="https://..."
          />
        </div>
      </div>
    </div>
    
    <!-- Description et Date -->
    <div class="card">
      <div class="grid grid-cols-1 gap-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1 flex items-center gap-1">
            <FileText class="w-4 h-4" />
            Description complète
          </label>
          <textarea
            bind:value={formData.description}
            rows="6"
            class="input-field resize-none"
            placeholder="Collez ici la description complète de l'offre..."
          ></textarea>
        </div>
        
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1 flex items-center gap-1">
            <Calendar class="w-4 h-4" />
            Date de publication
          </label>
          <input
            type="date"
            bind:value={formData.datePosted}
            class="input-field w-48"
          />
        </div>
      </div>
    </div>
    
    <!-- Actions -->
    <div class="flex items-center justify-between pt-4">
      <button
        type="button"
        onclick={goBack}
        class="px-6 py-3 text-gray-600 font-medium hover:text-gray-800 transition"
      >
        Annuler
      </button>
      
      <button
        type="submit"
        disabled={isSubmitting}
        class="btn-primary flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {#if isSubmitting}
          <svg class="w-5 h-5 animate-spin" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
          <span>Sauvegarde...</span>
        {:else}
          <Save class="w-5 h-5" />
          <span>Sauvegarder l'annonce</span>
        {/if}
      </button>
    </div>
  </form>
</div>
