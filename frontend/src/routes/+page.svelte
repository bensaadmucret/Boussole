<script lang="ts">
  import { onMount } from 'svelte';
  import { Plus, FileText, Briefcase, Calendar, CheckCircle, AlertCircle } from 'lucide-svelte';
  import { goto } from '$app/navigation';

  let stats = {
    listings: 12,
    applications: 5,
    interviews: 2,
    followUps: 3
  };

  let recentListings = [
    { id: 1, title: 'Développeur Full Stack', company: 'TechCorp', location: 'Paris (hybride)', type: 'CDI', stack: ['React', 'Node.js'] },
    { id: 2, title: 'Lead Developer Rust', company: 'StartupXYZ', location: 'Lyon (remote)', type: 'Freelance', stack: ['Rust', 'gRPC'] },
  ];

  let upcomingInterviews = [
    { id: 1, title: 'Entretien technique', company: 'TechCorp', date: 'MAR 08', time: '14:00', type: 'Visio' },
    { id: 2, title: 'Entretien RH', company: 'DataSoft', date: 'JEU 10', time: '10:30', type: 'Téléphone' },
  ];

  function goToNewListing() {
    goto('/listings/new');
  }
</script>

<div class="px-8 py-6">
  <!-- Header -->
  <div class="flex justify-between items-center mb-6">
    <div>
      <h1 class="text-3xl font-bold text-gray-900">Bonjour 👋</h1>
      <p class="text-gray-500 mt-1">Voici l'actualité de votre recherche</p>
    </div>
    <button on:click={goToNewListing} class="btn-primary flex items-center gap-2">
      <Plus class="w-5 h-5" />
      <span>Nouvelle annonce</span>
    </button>
  </div>

  <!-- Stats Cards -->
  <div class="grid grid-cols-4 gap-5 mb-8">
    <div class="card">
      <div class="flex items-center justify-between mb-3">
        <div class="w-14 h-14 bg-primary-100 rounded-2xl flex items-center justify-center">
          <FileText class="w-7 h-7 text-primary-600" />
        </div>
        <span class="text-3xl font-bold text-primary-700">{stats.listings}</span>
      </div>
      <p class="text-sm text-gray-500 font-medium">Annonces sauvegardées</p>
    </div>

    <div class="card">
      <div class="flex items-center justify-between mb-3">
        <div class="w-14 h-14 bg-orange-100 rounded-2xl flex items-center justify-center">
          <Briefcase class="w-7 h-7 text-orange-500" />
        </div>
        <span class="text-3xl font-bold text-orange-600">{stats.applications}</span>
      </div>
      <p class="text-sm text-gray-500 font-medium">Candidatures en cours</p>
    </div>

    <div class="card">
      <div class="flex items-center justify-between mb-3">
        <div class="w-14 h-14 bg-green-100 rounded-2xl flex items-center justify-center">
          <Calendar class="w-7 h-7 text-green-600" />
        </div>
        <span class="text-3xl font-bold text-green-600">{stats.interviews}</span>
      </div>
      <p class="text-sm text-gray-500 font-medium">Entretiens cette semaine</p>
    </div>

    <div class="card">
      <div class="flex items-center justify-between mb-3">
        <div class="w-14 h-14 bg-purple-100 rounded-2xl flex items-center justify-center">
          <AlertCircle class="w-7 h-7 text-purple-600" />
        </div>
        <span class="text-3xl font-bold text-purple-600">{stats.followUps}</span>
      </div>
      <p class="text-sm text-gray-500 font-medium">Relances à faire</p>
    </div>
  </div>

  <!-- Two Column Layout -->
  <div class="grid grid-cols-2 gap-6">
    <!-- Recent Listings -->
    <div class="card">
      <div class="flex items-center justify-between mb-5">
        <h2 class="font-bold text-xl text-gray-900">Annonces récentes</h2>
        <a href="/listings" class="text-primary-600 font-semibold text-sm hover:text-primary-700">Voir tout →</a>
      </div>
      <div class="space-y-4">
        {#each recentListings as listing}
          <div class="flex items-start gap-4 p-4 bg-gray-50 rounded-2xl hover:bg-primary-50 transition cursor-pointer">
            <div class="w-12 h-12 bg-primary-200 rounded-xl flex items-center justify-center text-primary-700 font-bold">
              {listing.company[0]}
            </div>
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2 mb-1">
                <h3 class="font-semibold text-gray-900">{listing.title}</h3>
                <span class="px-2 py-1 bg-primary-100 text-primary-700 text-xs font-semibold rounded-xl">{listing.type}</span>
              </div>
              <p class="text-sm text-gray-500">{listing.company} • {listing.location}</p>
              <div class="flex gap-2 mt-2">
                {#each listing.stack as tech}
                  <span class="px-2 py-1 bg-white rounded-lg text-xs text-gray-600">{tech}</span>
                {/each}
              </div>
            </div>
          </div>
        {/each}
      </div>
    </div>

    <!-- Upcoming Interviews -->
    <div class="card">
      <div class="flex items-center justify-between mb-5">
        <h2 class="font-bold text-xl text-gray-900">Prochains entretiens</h2>
        <a href="/calendar" class="text-primary-600 font-semibold text-sm hover:text-primary-700">Voir tout →</a>
      </div>
      <div class="space-y-4">
        {#each upcomingInterviews as interview, index}
          <div class="flex items-center gap-4 p-4 {index === 0 ? 'bg-accent-400/10 border border-accent-400/20' : 'bg-gray-50'} rounded-2xl">
            <div class="w-14 h-14 {index === 0 ? 'bg-accent-400' : 'bg-orange-200'} rounded-2xl flex flex-col items-center justify-center {index === 0 ? 'text-primary-900' : 'text-orange-700'}">
              <span class="text-xs font-bold">{interview.date.split(' ')[0]}</span>
              <span class="text-lg font-bold leading-none">{interview.date.split(' ')[1]}</span>
            </div>
            <div class="flex-1">
              <h3 class="font-semibold text-gray-900">{interview.title}</h3>
              <p class="text-sm text-gray-500">{interview.company} • {interview.time} • {interview.type}</p>
            </div>
            {#if index === 0}
              <button aria-label="Rejoindre l'appel visio" class="w-10 h-10 bg-accent-400 rounded-xl flex items-center justify-center text-primary-900 hover:bg-accent-500 transition">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z"/></svg>
              </button>
            {/if}
          </div>
        {/each}
      </div>
    </div>
  </div>
</div>
