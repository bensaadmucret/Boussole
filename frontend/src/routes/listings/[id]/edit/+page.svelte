<script lang="ts">
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { ArrowLeft, Save, Building2, MapPin, Banknote, Home, Tags, Link, FileText, Calendar } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { getJobListings, updateJobListing } from '$lib/utils/tauri';
  import { listings } from '$lib/stores/listings';

  let id = $derived(parseInt($page.params.id ?? '0'));
  let listing = $derived($listings.find(l => l.id === id));

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
  let loaded = $state(false);

  const contractTypes = ['CDI', 'CDD', 'Freelance', 'Stage', 'Alternance'];
  const remoteTypes = [
    { value: 'on_site', label: 'Sur site' },
    { value: 'hybrid', label: 'Hybride' },
    { value: 'full_remote', label: 'Full remote' }
  ];

  onMount(async () => {
    if ($listings.length === 0) {
      const data = await getJobListings();
      listings.set(data);
    }
    const found = $listings.find(l => l.id === id);
    if (!found) { goto('/listings'); return; }

    formData = {
      companyName: found.companyName,
      title: found.title,
      location: found.location ?? '',
      salaryMin: found.salaryMin ?? null,
      salaryMax: found.salaryMax ?? null,
      contractType: found.contractType,
      remoteType: found.remoteType,
      stack: [...(found.stack ?? [])],
      sourceSite: found.sourceSite ?? '',
      sourceUrl: found.sourceUrl ?? '',
      description: found.description ?? '',
      datePosted: found.datePosted ? new Date(found.datePosted).toLocaleDateString('fr-FR') : ''
    };
    loaded = true;
  });

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
    if (e.key === 'Enter') { e.preventDefault(); addTag(); }
  }

  function normalizeDateInput(value: string): string | null {
    const trimmed = value.trim();
    if (!trimmed) return null;
    const isoMatch = trimmed.match(/^(\d{4})-(\d{2})-(\d{2})$/);
    if (isoMatch) { const [, y, m, d] = isoMatch; return `${y}-${m}-${d}`; }
    const frMatch = trimmed.match(/^(\d{2})\/(\d{2})\/(\d{4})$/);
    if (frMatch) { const [, d, m, y] = frMatch; return `${y}-${m}-${d}`; }
    return null;
  }

  async function handleSubmit(e: Event) {
    e.preventDefault();
    if (!formData.companyName || !formData.title) {
      error = 'Veuillez remplir les champs obligatoires (société et titre)';
      return;
    }
    isSubmitting = true;
    error = '';
    const normalizedDatePosted = normalizeDateInput(formData.datePosted);
    if (formData.datePosted && !normalizedDatePosted) {
      error = 'Format de date invalide. Utilisez JJ/MM/AAAA.';
      isSubmitting = false;
      return;
    }
    try {
      const updated = await updateJobListing(id, {
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
        datePosted: normalizedDatePosted
      });
      listings.update(all => all.map(l => l.id === id ? updated : l));
      goto(`/listings/${id}`);
    } catch (err) {
      error = 'Erreur lors de la mise à jour : ' + err;
    } finally {
      isSubmitting = false;
    }
  }
</script>

<div class="px-8 py-6 max-w-4xl">
  <div class="flex items-center gap-4 mb-6">
    <button
      onclick={() => goto(`/listings/${id}`)}
      class="p-2 text-gray-500 hover:text-primary-600 hover:bg-primary-50 rounded-xl transition"
      aria-label="Retour"
    >
      <ArrowLeft class="w-6 h-6" />
    </button>
    <div>
      <h1 class="text-3xl font-bold text-gray-900">Modifier l'annonce</h1>
      <p class="text-gray-500 mt-1">{listing?.title ?? '...'}</p>
    </div>
  </div>

  {#if error}
    <div class="mb-6 p-4 bg-red-50 border border-red-200 rounded-2xl flex items-center gap-3 text-red-700">
      <svg class="w-5 h-5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>
      <span>{error}</span>
    </div>
  {/if}

  {#if !loaded}
    <div class="text-center py-12 text-gray-400">Chargement...</div>
  {:else}
  <form onsubmit={handleSubmit} class="space-y-6">
    <div class="card">
      <h2 class="font-bold text-lg text-gray-900 mb-4 flex items-center gap-2">
        <Building2 class="w-5 h-5 text-primary-500" />
        Informations principales
      </h2>
      <div class="space-y-4">
        <div>
          <label for="company" class="block text-sm font-medium text-gray-700 mb-1">Nom de la société <span class="text-red-500">*</span></label>
          <input id="company" type="text" bind:value={formData.companyName} class="input-field" placeholder="Ex: TechCorp" required />
        </div>
        <div>
          <label for="title" class="block text-sm font-medium text-gray-700 mb-1">Titre du poste <span class="text-red-500">*</span></label>
          <input id="title" type="text" bind:value={formData.title} class="input-field" placeholder="Ex: Développeur Full Stack" required />
        </div>
      </div>
    </div>

    <div class="card">
      <h2 class="font-bold text-lg text-gray-900 mb-4 flex items-center gap-2">
        <MapPin class="w-5 h-5 text-primary-500" />
        Localisation
      </h2>
      <div class="grid grid-cols-2 gap-4">
        <div>
          <label for="location" class="block text-sm font-medium text-gray-700 mb-1">Ville / Région</label>
          <input id="location" type="text" bind:value={formData.location} class="input-field" placeholder="Ex: Paris, Lyon..." />
        </div>
        <div>
          <label for="remote" class="block text-sm font-medium text-gray-700 mb-1 flex items-center gap-1"><Home class="w-4 h-4" /> Télétravail</label>
          <select id="remote" bind:value={formData.remoteType} class="input-field">
            {#each remoteTypes as type}
              <option value={type.value}>{type.label}</option>
            {/each}
          </select>
        </div>
      </div>
    </div>

    <div class="card">
      <h2 class="font-bold text-lg text-gray-900 mb-4 flex items-center gap-2">
        <Banknote class="w-5 h-5 text-primary-500" />
        Rémunération et contrat
      </h2>
      <div class="grid grid-cols-3 gap-4">
        <div>
          <label for="contract" class="block text-sm font-medium text-gray-700 mb-1">Type de contrat</label>
          <select id="contract" bind:value={formData.contractType} class="input-field">
            {#each contractTypes as type}
              <option value={type}>{type}</option>
            {/each}
          </select>
        </div>
        <div>
          <label for="salMin" class="block text-sm font-medium text-gray-700 mb-1">Salaire min (€)</label>
          <input id="salMin" type="number" bind:value={formData.salaryMin} class="input-field" placeholder="40000" />
        </div>
        <div>
          <label for="salMax" class="block text-sm font-medium text-gray-700 mb-1">Salaire max (€)</label>
          <input id="salMax" type="number" bind:value={formData.salaryMax} class="input-field" placeholder="60000" />
        </div>
      </div>
    </div>

    <div class="card">
      <h2 class="font-bold text-lg text-gray-900 mb-4 flex items-center gap-2">
        <Tags class="w-5 h-5 text-primary-500" />
        Stack technique
      </h2>
      <div class="mb-3 flex gap-2">
        <input type="text" bind:value={newTag} onkeydown={handleKeydown} class="input-field flex-1" placeholder="Ajouter une technologie..." />
        <button type="button" onclick={addTag} class="px-4 py-2 bg-primary-100 text-primary-700 rounded-xl font-medium hover:bg-primary-200 transition">+</button>
      </div>
      <div class="flex flex-wrap gap-2">
        {#each formData.stack as tag}
          <span class="inline-flex items-center gap-1 px-3 py-1 bg-primary-100 text-primary-800 rounded-xl text-sm font-medium">
            {tag}
            <button type="button" onclick={() => removeTag(tag)} class="w-4 h-4 flex items-center justify-center rounded-full hover:bg-primary-200 transition" aria-label={`Retirer ${tag}`}>×</button>
          </span>
        {/each}
      </div>
    </div>

    <div class="card">
      <h2 class="font-bold text-lg text-gray-900 mb-4 flex items-center gap-2">
        <Link class="w-5 h-5 text-primary-500" />
        Source de l'annonce
      </h2>
      <div class="grid grid-cols-2 gap-4">
        <div>
          <label for="sourceSite" class="block text-sm font-medium text-gray-700 mb-1">Site de parution</label>
          <input id="sourceSite" type="text" bind:value={formData.sourceSite} class="input-field" placeholder="Ex: LinkedIn, Indeed..." />
        </div>
        <div>
          <label for="sourceUrl" class="block text-sm font-medium text-gray-700 mb-1">Lien de l'offre</label>
          <input id="sourceUrl" type="url" bind:value={formData.sourceUrl} class="input-field" placeholder="https://..." />
        </div>
      </div>
    </div>

    <div class="card">
      <div class="space-y-4">
        <div>
          <label for="desc" class="block text-sm font-medium text-gray-700 mb-1 flex items-center gap-1"><FileText class="w-4 h-4" /> Description complète</label>
          <textarea id="desc" bind:value={formData.description} rows="6" class="input-field resize-none" placeholder="Description de l'offre..."></textarea>
        </div>
        <div>
          <label for="datePosted" class="block text-sm font-medium text-gray-700 mb-1 flex items-center gap-1"><Calendar class="w-4 h-4" /> Date de publication</label>
          <input id="datePosted" type="text" bind:value={formData.datePosted} inputmode="numeric" placeholder="JJ/MM/AAAA" class="input-field w-48" />
        </div>
      </div>
    </div>

    <div class="flex items-center justify-between pt-4">
      <button type="button" onclick={() => goto(`/listings/${id}`)} class="px-6 py-3 text-gray-600 font-medium hover:text-gray-800 transition">
        Annuler
      </button>
      <button type="submit" disabled={isSubmitting} class="btn-primary flex items-center gap-2 disabled:opacity-50 disabled:cursor-not-allowed">
        {#if isSubmitting}
          <svg class="w-5 h-5 animate-spin" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
          <span>Sauvegarde...</span>
        {:else}
          <Save class="w-5 h-5" />
          <span>Enregistrer les modifications</span>
        {/if}
      </button>
    </div>
  </form>
  {/if}
</div>
