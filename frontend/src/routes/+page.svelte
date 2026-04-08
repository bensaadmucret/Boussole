<script lang="ts">
  import { onMount } from 'svelte';
  import { Plus, FileText, Briefcase, Calendar, AlertCircle } from 'lucide-svelte';
  import { goto } from '$app/navigation';
  import { listings } from '$lib/stores/listings';
  import { applications } from '$lib/stores/applications';
  import { getJobListings, getApplications, getUnifiedCalendarEvents } from '$lib/utils/tauri';
  import type { UnifiedCalendarEvent } from '$lib/types';

  let calendarEvents = $state<UnifiedCalendarEvent[]>([]);

  onMount(async () => {
    if ($listings.length === 0) {
      const data = await getJobListings();
      listings.set(data);
    }
    if ($applications.length === 0) {
      const data = await getApplications();
      applications.set(data);
    }
    try {
      calendarEvents = await getUnifiedCalendarEvents();
    } catch {}
  });

  const now = new Date();
  const weekEnd = new Date(now.getTime() + 7 * 24 * 60 * 60 * 1000);

  let totalListings = $derived($listings.length);

  let activeApplications = $derived(
    $applications.filter(a => !['rejected', 'offer_accepted', 'offer_declined'].includes(a.status)).length
  );

  let upcomingInterviews = $derived(
    calendarEvents
      .filter(e => { const s = new Date(e.start); return s >= now && s <= weekEnd; })
      .sort((a, b) => new Date(a.start).getTime() - new Date(b.start).getTime())
      .slice(0, 4)
  );

  let followUps = $derived(
    $applications.filter(a => {
      if (a.status !== 'applied') return false;
      return (now.getTime() - new Date(a.appliedDate).getTime()) > 7 * 24 * 60 * 60 * 1000;
    }).length
  );

  let recentListings = $derived(
    [...$listings]
      .sort((a, b) => new Date(b.dateSaved).getTime() - new Date(a.dateSaved).getTime())
      .slice(0, 4)
  );

  function formatEventDate(dateStr: string) {
    const d = new Date(dateStr);
    const days = ['DIM', 'LUN', 'MAR', 'MER', 'JEU', 'VEN', 'SAM'];
    return { day: days[d.getDay()], num: d.getDate().toString().padStart(2, '0') };
  }

  function formatEventTime(dateStr: string) {
    const d = new Date(dateStr);
    return d.toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit' });
  }
</script>

<div class="px-8 py-6">
  <!-- Header -->
  <div class="flex justify-between items-center mb-6">
    <div>
      <h1 class="text-3xl font-bold text-gray-900">Bonjour 👋</h1>
      <p class="text-gray-500 mt-1">Voici l'actualité de votre recherche</p>
    </div>
    <button onclick={() => goto('/listings/new')} class="btn-primary flex items-center gap-2">
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
        <span class="text-3xl font-bold text-primary-700">{totalListings}</span>
      </div>
      <p class="text-sm text-gray-500 font-medium">Annonces sauvegardées</p>
    </div>

    <div class="card">
      <div class="flex items-center justify-between mb-3">
        <div class="w-14 h-14 bg-orange-100 rounded-2xl flex items-center justify-center">
          <Briefcase class="w-7 h-7 text-orange-500" />
        </div>
        <span class="text-3xl font-bold text-orange-600">{activeApplications}</span>
      </div>
      <p class="text-sm text-gray-500 font-medium">Candidatures en cours</p>
    </div>

    <div class="card">
      <div class="flex items-center justify-between mb-3">
        <div class="w-14 h-14 bg-green-100 rounded-2xl flex items-center justify-center">
          <Calendar class="w-7 h-7 text-green-600" />
        </div>
        <span class="text-3xl font-bold text-green-600">{upcomingInterviews.length}</span>
      </div>
      <p class="text-sm text-gray-500 font-medium">Évènements cette semaine</p>
    </div>

    <div class="card">
      <div class="flex items-center justify-between mb-3">
        <div class="w-14 h-14 bg-purple-100 rounded-2xl flex items-center justify-center">
          <AlertCircle class="w-7 h-7 text-purple-600" />
        </div>
        <span class="text-3xl font-bold text-purple-600">{followUps}</span>
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
      <div class="space-y-3">
        {#if recentListings.length === 0}
          <p class="text-sm text-gray-400 text-center py-6">Aucune annonce sauvegardée</p>
        {:else}
          {#each recentListings as listing}
            <a href="/listings/{listing.id}" class="flex items-start gap-4 p-4 bg-gray-50 rounded-2xl hover:bg-primary-50 transition cursor-pointer block">
              <div class="w-10 h-10 bg-primary-200 rounded-xl flex items-center justify-center text-primary-700 font-bold text-sm shrink-0">
                {listing.companyName[0].toUpperCase()}
              </div>
              <div class="flex-1 min-w-0">
                <div class="flex items-center gap-2 mb-0.5 flex-wrap">
                  <h3 class="font-semibold text-gray-900 text-sm truncate">{listing.title}</h3>
                  <span class="px-2 py-0.5 bg-primary-100 text-primary-700 text-xs font-semibold rounded-lg shrink-0">{listing.contractType}</span>
                </div>
                <p class="text-xs text-gray-500">{listing.companyName}{listing.location ? ` • ${listing.location}` : ''}</p>
                {#if listing.stack.length > 0}
                  <div class="flex gap-1 mt-1.5 flex-wrap">
                    {#each listing.stack.slice(0, 3) as tech}
                      <span class="px-2 py-0.5 bg-white border border-gray-200 rounded-lg text-xs text-gray-600">{tech}</span>
                    {/each}
                  </div>
                {/if}
              </div>
            </a>
          {/each}
        {/if}
      </div>
    </div>

    <!-- Upcoming Events -->
    <div class="card">
      <div class="flex items-center justify-between mb-5">
        <h2 class="font-bold text-xl text-gray-900">Prochains événements</h2>
        <a href="/calendar" class="text-primary-600 font-semibold text-sm hover:text-primary-700">Voir tout →</a>
      </div>
      <div class="space-y-3">
        {#if upcomingInterviews.length === 0}
          <p class="text-sm text-gray-400 text-center py-6">Aucun événement cette semaine</p>
        {:else}
          {#each upcomingInterviews as event, index}
            {@const dateInfo = formatEventDate(event.start)}
            <div class="flex items-center gap-4 p-4 {index === 0 ? 'bg-accent-400/10 border border-accent-400/20' : 'bg-gray-50'} rounded-2xl">
              <div class="w-12 h-12 {index === 0 ? 'bg-accent-400 text-primary-900' : 'bg-orange-100 text-orange-700'} rounded-2xl flex flex-col items-center justify-center shrink-0">
                <span class="text-xs font-bold leading-none">{dateInfo.day}</span>
                <span class="text-lg font-bold leading-none">{dateInfo.num}</span>
              </div>
              <div class="flex-1 min-w-0">
                <h3 class="font-semibold text-gray-900 text-sm truncate">{event.title}</h3>
                <p class="text-xs text-gray-500">{formatEventTime(event.start)}{event.location ? ` • ${event.location}` : ''}</p>
                <p class="text-xs text-gray-400 mt-0.5">{event.source.calendarName}</p>
              </div>
            </div>
          {/each}
        {/if}
      </div>
    </div>
  </div>
</div>
