<script lang="ts">
  import { onMount } from 'svelte';
  import {
    getGeminiConfig,
    saveGeminiConfig,
    generateCoverLetter,
    getCvProfiles,
    getDocuments,
    createDocument
  } from '$lib/utils/tauri';
  import type { Document, GeminiConfig } from '$lib/types';
  import { Check, Copy, ExternalLink, FileText, Loader2, Sparkles, MessageSquare } from 'lucide-svelte';

  // State
  let config: GeminiConfig = { apiKey: '' };
  let isConfigured = false;
  let isLoadingConfig = true;
  let isSavingConfig = false;
  let configFeedback = '';

  // Form
  let cvProfiles: Document[] = [];
  let selectedCvContent = '';
  let jobTitle = '';
  let companyName = '';
  let jobDescription = '';
  let selectedTone = 'formel';

  // Generation state
  let isGenerating = false;
  let generatedLetter = '';
  let isSavingLetter = false;
  let letterFeedback = '';
  let isCopied = false;

  // History
  let history: Document[] = [];
  let searchQuery = '';
  
  $: filteredHistory = history.filter(doc => 
    doc.name.toLowerCase().includes(searchQuery.toLowerCase()) || 
    (doc.profileName && doc.profileName.toLowerCase().includes(searchQuery.toLowerCase()))
  );

  onMount(async () => {
    await loadInitialData();
  });

  async function loadInitialData() {
    isLoadingConfig = true;
    try {
      // Load config
      const savedConfig = await getGeminiConfig();
      if (savedConfig && savedConfig.apiKey) {
        config = savedConfig;
        isConfigured = true;
      }

      // Load CVs
      const docs = await getDocuments();
      cvProfiles = docs.filter(d => d.docType === 'cv');
      
      // Select first CV by default if available
      if (cvProfiles.length > 0 && cvProfiles[0].content) {
        const textDecoder = new TextDecoder('utf-8');
        selectedCvContent = textDecoder.decode(new Uint8Array(cvProfiles[0].content));
      }

      // Load History
      history = docs.filter(d => d.docType === 'cover_letter').sort((a, b) => 
        new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime()
      );
    } catch (error) {
      console.error('Erreur lors du chargement initial:', error);
    } finally {
      isLoadingConfig = false;
    }
  }

  async function saveConfig() {
    isSavingConfig = true;
    configFeedback = '';
    try {
      await saveGeminiConfig(config);
      isConfigured = !!config.apiKey.trim();
      configFeedback = isConfigured ? 'Clé API enregistrée avec succès.' : 'Clé API supprimée.';
    } catch (error) {
      configFeedback = `Erreur lors de l'enregistrement : ${error}`;
    } finally {
      isSavingConfig = false;
    }
  }

  async function handleGenerate() {
    if (!jobTitle.trim() || !companyName.trim() || !jobDescription.trim() || !selectedCvContent.trim()) {
      letterFeedback = 'Veuillez remplir tous les champs obligatoires.';
      return;
    }

    isGenerating = true;
    letterFeedback = '';
    generatedLetter = '';

    try {
      const response = await generateCoverLetter({
        jobTitle,
        companyName,
        jobDescription,
        cvContent: selectedCvContent,
        tone: selectedTone
      });
      generatedLetter = response.content;
    } catch (error) {
      letterFeedback = `Erreur de génération : ${error}`;
    } finally {
      isGenerating = false;
    }
  }

  async function handleSaveLetter() {
    if (!generatedLetter.trim()) return;

    isSavingLetter = true;
    letterFeedback = '';

    try {
      const textEncoder = new TextEncoder();
      const contentArray = Array.from(textEncoder.encode(generatedLetter));
      
      await createDocument({
        docType: 'cover_letter',
        name: `Lettre - ${companyName} - ${jobTitle}`,
        profileName: jobTitle,
        content: contentArray
      });
      
      letterFeedback = 'Lettre sauvegardée dans l\'historique !';
      await loadInitialData(); // Refresh history
    } catch (error) {
      letterFeedback = `Erreur lors de la sauvegarde : ${error}`;
    } finally {
      isSavingLetter = false;
    }
  }

  async function copyToClipboard(text: string) {
    try {
      await navigator.clipboard.writeText(text);
      isCopied = true;
      setTimeout(() => isCopied = false, 2000);
    } catch (err) {
      console.error('Failed to copy text: ', err);
    }
  }

  function handleCvChange(event: Event) {
    const select = event.target as HTMLSelectElement;
    const selectedId = parseInt(select.value);
    const selectedCv = cvProfiles.find(cv => cv.id === selectedId);
    
    if (selectedCv && selectedCv.content) {
      const textDecoder = new TextDecoder('utf-8');
      selectedCvContent = textDecoder.decode(new Uint8Array(selectedCv.content));
    }
  }

  function viewHistoryLetter(doc: Document) {
    if (doc.content) {
      const textDecoder = new TextDecoder('utf-8');
      generatedLetter = textDecoder.decode(new Uint8Array(doc.content));
      companyName = doc.name.split(' - ')[1] || '';
      jobTitle = doc.profileName || '';
      
      // Scroll to editor
      document.getElementById('letter-editor')?.scrollIntoView({ behavior: 'smooth' });
    }
  }
</script>

<svelte:head>
  <title>Assistant IA · Boussole</title>
</svelte:head>

<div class="px-8 py-6 space-y-8">
  <div>
    <h1 class="text-3xl font-bold text-gray-900 mb-3">Assistant IA</h1>
    <p class="text-gray-500 max-w-3xl">
      Configure ta clé API Gemini pour générer des lettres de motivation personnalisées et percutantes.
    </p>
  </div>

  {#if isLoadingConfig}
    <div class="flex items-center justify-center p-12">
      <Loader2 class="w-8 h-8 text-primary-500 animate-spin" />
    </div>
  {:else}
    <!-- Configuration Section -->
    <section class="card p-6 space-y-4 border-l-4 {isConfigured ? 'border-l-emerald-500' : 'border-l-amber-500'}">
      <div class="flex flex-col gap-2 md:flex-row md:items-end md:justify-between">
        <div>
          <h2 class="text-xl font-semibold text-gray-900 flex items-center gap-2">
            Configuration Gemini API
            {#if isConfigured}
              <span class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-emerald-100 text-emerald-700">
                <Check class="w-3 h-3 mr-1" />
                Connecté
              </span>
            {/if}
          </h2>
          <p class="text-sm text-gray-500 mt-1">
            Obtiens une clé API gratuite sur <a href="https://aistudio.google.com/app/apikey" target="_blank" rel="noopener noreferrer" class="text-primary-600 hover:underline inline-flex items-center">Google AI Studio <ExternalLink class="w-3 h-3 ml-1"/></a>.
          </p>
        </div>

        <button
          type="button"
          onclick={saveConfig}
          disabled={isSavingConfig}
          class="btn-secondary whitespace-nowrap"
        >
          {#if isSavingConfig}
            <Loader2 class="w-4 h-4 mr-2 animate-spin" />
            Enregistrement...
          {:else}
            {isConfigured ? 'Mettre à jour la clé' : 'Enregistrer la clé API'}
          {/if}
        </button>
      </div>

      <div class="max-w-2xl">
        <input
          bind:value={config.apiKey}
          type="password"
          class="input-field w-full"
          placeholder="AIzaSy..."
        />
        {#if configFeedback}
          <p class="text-sm mt-2 {configFeedback.includes('Erreur') ? 'text-red-600' : 'text-emerald-600'}">{configFeedback}</p>
        {/if}
      </div>
    </section>

    <div class="grid grid-cols-1 xl:grid-cols-12 gap-8">
      <!-- Left Column: Generator & Editor -->
      <div class="xl:col-span-8 space-y-6">
        
        {#if isConfigured}
          <section class="card p-6 space-y-6">
            <h2 class="text-xl font-semibold text-gray-900 flex items-center gap-2">
              <Sparkles class="w-5 h-5 text-primary-500" />
              Générer une lettre
            </h2>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">CV Source</label>
                <select class="input-field w-full" onchange={handleCvChange}>
                  {#if cvProfiles.length === 0}
                    <option value="">Aucun CV disponible</option>
                  {:else}
                    {#each cvProfiles as cv}
                      <option value={cv.id}>{cv.profileName || cv.name}</option>
                    {/each}
                  {/if}
                </select>
                {#if cvProfiles.length === 0}
                  <p class="text-xs text-amber-600 mt-1">Ajoute d'abord un CV dans la section Documents.</p>
                {/if}
              </div>

              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Ton de la lettre</label>
                <select bind:value={selectedTone} class="input-field w-full">
                  <option value="formel">Formel & Professionnel</option>
                  <option value="chaleureux">Chaleureux & Enthousiaste</option>
                  <option value="direct">Direct & Concis</option>
                  <option value="creatif">Créatif & Audacieux</option>
                </select>
              </div>

              <div class="md:col-span-2 grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">Titre du poste visé</label>
                  <input bind:value={jobTitle} type="text" class="input-field w-full" placeholder="Ex: Développeur Fullstack" />
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-1">Entreprise</label>
                  <input bind:value={companyName} type="text" class="input-field w-full" placeholder="Ex: Acme Corp" />
                </div>
              </div>

              <div class="md:col-span-2">
                <label class="block text-sm font-medium text-gray-700 mb-1">Description de l'offre (copier-coller)</label>
                <textarea bind:value={jobDescription} class="input-field w-full h-32 resize-y" placeholder="Colle ici le contenu de l'offre d'emploi..."></textarea>
              </div>
            </div>

            <div class="flex items-center justify-between">
              <p class="text-sm text-gray-600 max-w-md">{letterFeedback}</p>
              <button
                onclick={handleGenerate}
                disabled={isGenerating || !selectedCvContent || !jobTitle || !companyName || !jobDescription}
                class="btn-primary"
              >
                {#if isGenerating}
                  <Loader2 class="w-4 h-4 mr-2 animate-spin" />
                  Génération en cours...
                {:else}
                  <Sparkles class="w-4 h-4 mr-2" />
                  Générer la lettre
                {/if}
              </button>
            </div>
          </section>

          <!-- Editor -->
          <section id="letter-editor" class="card p-0 overflow-hidden flex flex-col h-[600px]">
            <div class="bg-gray-50 border-b border-gray-100 p-4 flex items-center justify-between">
              <h3 class="font-semibold text-gray-700 flex items-center gap-2">
                <FileText class="w-4 h-4" />
                Résultat
              </h3>
              <div class="flex items-center gap-2">
                <button
                  onclick={() => copyToClipboard(generatedLetter)}
                  disabled={!generatedLetter}
                  class="btn-secondary py-1.5 px-3 text-sm"
                  title="Copier"
                >
                  {#if isCopied}
                    <Check class="w-4 h-4 mr-1 text-emerald-500" /> Copié!
                  {:else}
                    <Copy class="w-4 h-4 mr-1" /> Copier
                  {/if}
                </button>
                <button
                  onclick={handleSaveLetter}
                  disabled={!generatedLetter || isSavingLetter}
                  class="btn-primary py-1.5 px-3 text-sm"
                >
                  {#if isSavingLetter}
                    <Loader2 class="w-4 h-4 mr-1 animate-spin" />
                  {:else}
                    Enregistrer
                  {/if}
                </button>
              </div>
            </div>
            
            <textarea
              bind:value={generatedLetter}
              class="flex-1 w-full p-6 resize-none focus:outline-none focus:ring-0 border-0 leading-relaxed text-gray-800"
              placeholder="La lettre générée apparaîtra ici. Tu pourras la modifier librement."
            ></textarea>
          </section>
        {:else}
          <div class="card p-12 text-center text-gray-500 border border-dashed border-gray-200">
            <Sparkles class="w-12 h-12 mx-auto mb-4 text-gray-300" />
            <h3 class="text-lg font-medium text-gray-900 mb-2">Assistant IA désactivé</h3>
            <p>Renseigne ta clé API Gemini ci-dessus pour débloquer la génération de lettres de motivation.</p>
          </div>
        {/if}
      </div>

      <!-- Right Column: History -->
      <div class="xl:col-span-4 space-y-4">
        <div class="card p-5 h-[calc(100vh-12rem)] flex flex-col">
          <h2 class="text-lg font-semibold text-gray-900 mb-4 flex items-center gap-2">
            <MessageSquare class="w-5 h-5 text-gray-400" />
            Historique
          </h2>

          <input
            bind:value={searchQuery}
            type="text"
            placeholder="Rechercher (ex: Acme)..."
            class="input-field w-full mb-4 text-sm"
          />

          <div class="flex-1 overflow-y-auto pr-2 space-y-3">
            {#if history.length === 0}
              <p class="text-sm text-center text-gray-500 py-8">Aucune lettre générée pour le moment.</p>
            {:else if filteredHistory.length === 0}
              <p class="text-sm text-center text-gray-500 py-8">Aucun résultat pour cette recherche.</p>
            {:else}
              {#each filteredHistory as doc}
                <button
                  class="w-full text-left p-3 rounded-xl border border-gray-100 hover:border-primary-200 hover:bg-primary-50/50 transition group"
                  onclick={() => viewHistoryLetter(doc)}
                >
                  <p class="font-medium text-gray-900 truncate group-hover:text-primary-700 transition">{doc.profileName || 'Poste inconnu'}</p>
                  <p class="text-sm text-gray-500 truncate">{doc.name.replace('Lettre - ', '')}</p>
                  <p class="text-xs text-gray-400 mt-2">{new Date(doc.createdAt).toLocaleDateString('fr-FR')}</p>
                </button>
              {/each}
            {/if}
          </div>
        </div>
      </div>
      
    </div>
  {/if}
</div>
