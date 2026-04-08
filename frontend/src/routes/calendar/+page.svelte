<script lang="ts">
  import { onMount } from 'svelte';
  import { googleCalendarAccounts, unifiedCalendarEvents } from '$lib/stores/documents';
  import type { UnifiedCalendarEvent } from '$lib/types';
  import {
    getGoogleCalendarAccounts,
    getUnifiedCalendarEvents
  } from '$lib/utils/tauri';

  const formatDate = new Intl.DateTimeFormat('fr-FR', {
    weekday: 'long',
    day: '2-digit',
    month: 'long'
  });

  const formatTime = new Intl.DateTimeFormat('fr-FR', {
    hour: '2-digit',
    minute: '2-digit'
  });

  onMount(() => {
    void (async () => {
      const [storedAccounts, storedEvents] = await Promise.all([
        getGoogleCalendarAccounts(),
        getUnifiedCalendarEvents()
      ]);

      googleCalendarAccounts.set(storedAccounts);
      unifiedCalendarEvents.set(storedEvents);
    })();
  });

  function parseStart(iso: string): Date {
    return new Date(iso.includes('+') || iso.endsWith('Z') ? iso : iso + 'Z');
  }

  function localDayKey(iso: string): string {
    const d = parseStart(iso);
    return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, '0')}-${String(d.getDate()).padStart(2, '0')}`;
  }

  $: todayKey = localDayKey(new Date().toISOString());

  $: events = [...$unifiedCalendarEvents]
    .filter((e) => localDayKey(e.start) >= todayKey)
    .sort((a, b) => parseStart(a.start).getTime() - parseStart(b.start).getTime());

  $: groupedEvents = events.reduce<{ day: string; label: string; items: UnifiedCalendarEvent[] }[]>((groups, event) => {
    const day = localDayKey(event.start);
    const label = new Intl.DateTimeFormat('fr-FR', { weekday: 'long', day: 'numeric', month: 'long', year: 'numeric' }).format(parseStart(event.start));
    const last = groups[groups.length - 1];
    if (last && last.day === day) {
      last.items.push(event);
    } else {
      groups.push({ day, label, items: [event] });
    }
    return groups;
  }, []);  

  $: activeAccounts = $googleCalendarAccounts.length;
  $: activeCalendars = $googleCalendarAccounts.reduce(
    (total, account) => total + account.calendars.filter((calendar) => calendar.active).length,
    0
  );

  function getEventTimeRange(event: UnifiedCalendarEvent) {
    const d = parseStart(event.start);
    const isAllDay = !event.start.includes('T') || event.start.endsWith('T00:00:00') || event.start.endsWith('T00:00:00Z');
    if (isAllDay) return 'Journée entière';
    const start = formatTime.format(parseStart(event.start));
    const end = formatTime.format(parseStart(event.end));
    return `${start} — ${end}`;
  }
</script>

<svelte:head>
  <title>Agenda · Boussole</title>
</svelte:head>

<div class="px-8 py-6 space-y-6">
  <div class="flex items-start justify-between gap-6">
    <div>
      <h1 class="text-3xl font-bold text-gray-900 mb-3">Agenda</h1>
      <p class="text-gray-500 max-w-3xl">
        Vue unifiée des entretiens et relances, consolidée depuis plusieurs comptes Gmail et plusieurs calendriers.
      </p>
    </div>

    <div class="text-right">
      <p class="text-sm text-gray-500">Comptes connectés</p>
      <p class="text-2xl font-bold text-gray-900">{activeAccounts}</p>
      <p class="text-xs text-gray-400">{activeCalendars} calendriers actifs</p>
    </div>
  </div>

  <div class="grid gap-4 md:grid-cols-3">
    <div class="card p-5">
      <p class="text-sm text-gray-500">Événements consolidés</p>
      <p class="text-3xl font-bold text-gray-900 mt-2">{events.length}</p>
    </div>

    <div class="card p-5">
      <p class="text-sm text-gray-500">Source principale</p>
      <p class="text-lg font-semibold text-gray-900 mt-2">Google Calendar</p>
      <p class="text-xs text-gray-400 mt-1">Mélange de plusieurs comptes, affiché dans une seule vue</p>
    </div>

    <div class="card p-5">
      <p class="text-sm text-gray-500">Synchronisation</p>
      <p class="text-lg font-semibold text-gray-900 mt-2">Active</p>
      <p class="text-xs text-gray-400 mt-1">Basée sur les réglages consolidés dans Paramètres</p>
    </div>
  </div>

  <div class="card p-6">
    <div class="flex items-center justify-between mb-5">
      <div>
        <h2 class="text-xl font-semibold text-gray-900">Événements à venir</h2>
        <p class="text-sm text-gray-500 mt-1">Chaque événement affiche sa provenance pour garder le contexte.</p>
      </div>
      <div class="text-sm text-gray-500">Triés par date</div>
    </div>

    <div class="space-y-6">
      {#if events.length === 0}
        <div class="rounded-3xl border border-dashed border-gray-200 bg-gray-50 p-8 text-center text-gray-500">
          Aucun événement synchronisé pour le moment. Associe un compte Google dans <a href="/settings" class="text-primary-700 font-medium">Paramètres</a> puis lance la synchronisation.
        </div>
      {:else}
        {#each groupedEvents as group}
          <div>
            <h3 class="text-xs font-bold text-gray-500 uppercase tracking-widest mb-3 border-b border-gray-100 pb-2">{group.label}</h3>
            <div class="space-y-3">
              {#each group.items as event}
              <article class="rounded-3xl border border-gray-100 bg-white p-5 shadow-sm flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
                <div class="flex items-start gap-4">
                  <div class="rounded-2xl px-4 py-3 text-center min-w-[60px] bg-gray-50 text-gray-700">
                    <div class="text-xs font-bold uppercase tracking-wide">{parseStart(event.start).toLocaleDateString('fr-FR', { month: 'short' })}</div>
                    <div class="text-2xl font-extrabold leading-none mt-1">{parseStart(event.start).getDate()}</div>
                  </div>

                  <div>
                    <h4 class="text-lg font-semibold text-gray-900">{event.title}</h4>
                    <p class="text-sm text-gray-500 mt-1">{getEventTimeRange(event)}</p>
                    {#if event.location}
                      <p class="text-sm text-gray-400 mt-1">{event.location}</p>
                    {/if}
                  </div>
                </div>

                <div class="flex flex-wrap items-center gap-2">
                  <span class="rounded-full px-3 py-1 text-xs font-medium text-gray-700 bg-gray-100">
                    {event.source.accountName}
                  </span>
                  <span class="rounded-full px-3 py-1 text-xs font-medium text-gray-700 bg-gray-100 flex items-center gap-1.5">
                    <span class="inline-block w-2 h-2 rounded-full flex-shrink-0" style={`background:${event.source.color}`}></span>
                    {event.source.calendarName}
                  </span>
                  <span class="rounded-full px-3 py-1 text-xs font-medium text-gray-500 bg-gray-50">
                    {event.source.accountEmail}
                  </span>
                </div>
              </article>
              {/each}
            </div>
          </div>
        {/each}
      {/if}
    </div>
  </div>
</div>
