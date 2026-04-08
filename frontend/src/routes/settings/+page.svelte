<script lang="ts">
  import { onMount } from 'svelte';
  import { get } from 'svelte/store';
  import { googleCalendarAccounts, unifiedCalendarEvents, unifiedCalendarSettings } from '$lib/stores/documents';
  import type {
    GoogleCalendarAccount,
    GoogleOAuthConfig,
    UnifiedCalendarSettings
  } from '$lib/types';
  import {
    connectGoogleCalendarAccount,
    deleteGoogleCalendarAccount,
    getGoogleCalendarAccounts,
    getGoogleOauthConfig,
    getUnifiedCalendarEvents,
    getUnifiedCalendarSettings,
    saveGoogleCalendarAccounts,
    saveGoogleOauthConfig,
    saveUnifiedCalendarSettings,
    syncUnifiedCalendarEvents
  } from '$lib/utils/tauri';

  let googleClientId = '';
  let googleClientSecret = '';
  let oauthFeedback = '';
  let isSavingClientId = false;
  let isConnecting = false;
  let isSyncing = false;
  let refreshToken = 0;

  async function refreshGoogleCalendarState() {
    const token = ++refreshToken;

    const [storedConfig, storedAccounts, storedSettings, storedEvents] = await Promise.all([
      getGoogleOauthConfig(),
      getGoogleCalendarAccounts(),
      getUnifiedCalendarSettings(),
      getUnifiedCalendarEvents()
    ]);

    if (token !== refreshToken) {
      return;
    }

    googleClientId = storedConfig.clientId ?? '';
    googleClientSecret = storedConfig.clientSecret ?? '';
    googleCalendarAccounts.set(storedAccounts);
    unifiedCalendarSettings.set(storedSettings);
    unifiedCalendarEvents.set(storedEvents);
  }

  onMount(() => {
    void refreshGoogleCalendarState();
  });

  async function saveClientId() {
    isSavingClientId = true;
    oauthFeedback = '';

    try {
      const secret = googleClientSecret.trim();
      await saveGoogleOauthConfig({ clientId: googleClientId.trim(), ...(secret ? { clientSecret: secret } : {}) });
      oauthFeedback = 'Client ID Google enregistré.';
    } catch (error) {
      oauthFeedback = `Impossible d’enregistrer le Client ID : ${error}`;
    } finally {
      isSavingClientId = false;
    }
  }

  function updateSettings(patch: Partial<UnifiedCalendarSettings>) {
    const nextSettings = { ...$unifiedCalendarSettings, ...patch };
    unifiedCalendarSettings.set(nextSettings);
    void saveUnifiedCalendarSettings(nextSettings);
  }

  async function persistAccounts(accounts: GoogleCalendarAccount[]) {
    googleCalendarAccounts.set(accounts);
    await saveGoogleCalendarAccounts(accounts);
  }

  async function connectAccount() {
    if (!googleClientId.trim()) {
      oauthFeedback = 'Ajoute le Google Client ID avant de connecter un compte.';
      return;
    }

    isConnecting = true;
    oauthFeedback = '';

    try {
      const secret = googleClientSecret.trim();
      await saveGoogleOauthConfig({ clientId: googleClientId.trim(), ...(secret ? { clientSecret: secret } : {}) });
      const account = await connectGoogleCalendarAccount();

      const currentAccounts = get(googleCalendarAccounts);
      await persistAccounts([
        ...currentAccounts.filter((existing) => existing.email !== account.email),
        account
      ]);

      await syncUnifiedCalendarEvents();
      await refreshGoogleCalendarState();
      oauthFeedback = `Compte connecté : ${account.email}`;
    } catch (error) {
      oauthFeedback = `Connexion Google impossible : ${error}`;
    } finally {
      isConnecting = false;
    }
  }

  async function removeAccount(accountEmail: string) {
    await deleteGoogleCalendarAccount(accountEmail);

    await syncUnifiedCalendarEvents();
    await refreshGoogleCalendarState();
    oauthFeedback = `Compte déconnecté : ${accountEmail}`;
  }

  async function toggleCalendar(accountId: string, calendarId: string) {
    await persistAccounts(
      $googleCalendarAccounts.map((account) =>
        account.id !== accountId
          ? account
          : {
              ...account,
              calendars: account.calendars.map((calendar) =>
                calendar.id === calendarId ? { ...calendar, active: !calendar.active } : calendar
              )
            }
      )
    );

    await syncUnifiedCalendarEvents();
    await refreshGoogleCalendarState();
  }

  async function syncNow() {
    isSyncing = true;
    oauthFeedback = '';

    try {
      const events = await syncUnifiedCalendarEvents();
      await refreshGoogleCalendarState();

      if ($googleCalendarAccounts.length === 0) {
        oauthFeedback = 'Aucun compte Google connecté, aucune synchronisation effectuée.';
      } else if (events.length === 0) {
        oauthFeedback = 'Synchronisation terminée, mais aucun événement Google n’a été trouvé.';
      } else {
        oauthFeedback = `Agenda synchronisé avec succès : ${events.length} événement${events.length > 1 ? 's' : ''}.`;
      }
    } catch (error) {
      oauthFeedback = `Synchronisation impossible : ${error}`;
    } finally {
      isSyncing = false;
    }
  }
</script>

<svelte:head>
  <title>Paramètres · Boussole</title>
</svelte:head>

<div class="px-8 py-6 space-y-8">
  <div class="flex items-start justify-between gap-6">
    <div>
      <h1 class="text-3xl font-bold text-gray-900 mb-3">Paramètres</h1>
      <p class="text-gray-500 max-w-3xl">
        Configure la synchronisation Google Calendar, associe plusieurs comptes Gmail et consolide tous les calendriers dans un agenda unique dans Boussole.
      </p>
    </div>

    <button
      type="button"
      onclick={connectAccount}
      disabled={isConnecting}
      class="px-5 py-3 rounded-2xl bg-gradient-to-r from-primary-500 to-accent-400 text-white font-semibold shadow-lg hover:shadow-xl transition whitespace-nowrap shrink-0 disabled:opacity-60 disabled:cursor-not-allowed"
    >
      {isConnecting ? 'Connexion...' : 'Associer un compte Gmail'}
    </button>
  </div>

  <section class="card p-6 space-y-4">
    <div class="flex flex-col gap-2 md:flex-row md:items-end md:justify-between">
      <div>
        <h2 class="text-xl font-semibold text-gray-900">Connexion Google OAuth</h2>
        <p class="text-sm text-gray-500 mt-1">Ajoute le Client ID de ton application Google, puis associe un compte Gmail réel.</p>
      </div>

      <button
        type="button"
        onclick={saveClientId}
        disabled={isSavingClientId}
        class="px-4 py-3 rounded-xl border border-primary-200 text-primary-700 font-semibold hover:bg-primary-50 transition whitespace-nowrap disabled:opacity-60"
      >
        {isSavingClientId ? 'Enregistrement...' : 'Enregistrer le Client ID'}
      </button>
    </div>

    <div class="grid gap-3 md:grid-cols-[1fr_auto]">
      <input
        bind:value={googleClientId}
        type="text"
        class="input-field"
        placeholder="xxxxxxxxxxxx.apps.googleusercontent.com"
      />
      <input
        bind:value={googleClientSecret}
        type="password"
        class="input-field"
        placeholder="Client Secret (requis pour Web App, vide pour Desktop App)"
      />
      <button
        type="button"
        onclick={syncNow}
        disabled={isSyncing}
        class="px-4 py-3 rounded-xl bg-gray-900 text-white font-semibold hover:bg-gray-800 transition whitespace-nowrap disabled:opacity-60"
      >
        {isSyncing ? 'Synchronisation...' : 'Synchroniser maintenant'}
      </button>
    </div>

    {#if oauthFeedback}
      <p class="text-sm text-gray-600">{oauthFeedback}</p>
    {/if}
  </section>

  <div class="grid gap-4 md:grid-cols-3">
    <div class="card p-5">
      <p class="text-sm text-gray-500">Comptes liés</p>
      <p class="text-3xl font-bold text-gray-900 mt-2">{$googleCalendarAccounts.length}</p>
    </div>

    <div class="card p-5">
      <p class="text-sm text-gray-500">Calendriers actifs</p>
      <p class="text-3xl font-bold text-gray-900 mt-2">
        {$googleCalendarAccounts.reduce((total, account) => total + account.calendars.filter((calendar) => calendar.active).length, 0)}
      </p>
    </div>

    <div class="card p-5">
      <p class="text-sm text-gray-500">Vue consolidée</p>
      <p class="text-3xl font-bold text-gray-900 mt-2">
        {$unifiedCalendarSettings.enabled ? 'Activée' : 'Désactivée'}
      </p>
    </div>
  </div>

  <div class="grid gap-6 xl:grid-cols-2">
    <section class="card p-6 space-y-5">
      <div class="flex items-start justify-between gap-4">
        <div>
          <h2 class="text-xl font-semibold text-gray-900">Comptes Google connectés</h2>
          <p class="text-sm text-gray-500 mt-1">Chaque compte peut exposer plusieurs calendriers, que tu peux activer ou masquer individuellement.</p>
        </div>
      </div>

      <div class="space-y-4">
        {#if $googleCalendarAccounts.length === 0}
          <div class="rounded-3xl border border-dashed border-gray-200 bg-gray-50 p-6 text-sm text-gray-500">
            Aucun compte Google n’est encore associé. Utilise le bouton <strong>Associer un compte Gmail</strong> pour lancer l’OAuth réel.
          </div>
        {:else}
          {#each $googleCalendarAccounts as account}
          <article class="rounded-3xl border border-gray-100 bg-white p-5 shadow-sm">
            <div class="flex items-start justify-between gap-4 mb-4">
              <div class="flex items-center gap-4">
                <div class="w-12 h-12 rounded-2xl flex items-center justify-center text-white font-bold shadow-sm" style={`background:${account.avatarColor}`}>
                  {account.displayName.slice(0, 1)}
                </div>
                <div>
                  <h3 class="font-semibold text-gray-900">{account.displayName}</h3>
                  <p class="text-sm text-gray-500">{account.email}</p>
                  <p class="text-xs text-gray-400 mt-1">{account.connectedAt}</p>
                </div>
              </div>

              <button
                type="button"
                onclick={() => removeAccount(account.email)}
                class="text-sm font-medium text-rose-600 hover:text-rose-700 transition"
              >
                Déconnecter
              </button>
            </div>

            <div class="space-y-3">
              {#each account.calendars as calendar}
                <label class="flex items-center justify-between gap-4 rounded-2xl bg-gray-50 px-4 py-3">
                  <div class="flex items-center gap-3 min-w-0">
                    <span class="w-3 h-3 rounded-full shrink-0" style={`background:${calendar.color}`}></span>
                    <div class="min-w-0">
                      <p class="font-medium text-gray-900 truncate">{calendar.name}</p>
                      <p class="text-xs text-gray-500">Source synchronisée dans l’agenda unifié</p>
                    </div>
                  </div>

                  <input
                    type="checkbox"
                    checked={calendar.active}
                    onchange={() => toggleCalendar(account.id, calendar.id)}
                    class="h-5 w-5 rounded border-gray-300 text-primary-600 focus:ring-primary-500"
                  />
                </label>
              {/each}
            </div>
          </article>
          {/each}
        {/if}
      </div>
    </section>

    <section class="card p-6 space-y-5">
      <div>
        <h2 class="text-xl font-semibold text-gray-900">Agenda consolidé</h2>
        <p class="text-sm text-gray-500 mt-1">Boussole fusionne les événements des différents comptes dans une seule vue, tout en gardant la provenance de chaque événement.</p>
      </div>

      <div class="space-y-4">
        <label class="flex items-center justify-between gap-4 rounded-2xl bg-gray-50 px-4 py-3">
          <div>
            <p class="font-medium text-gray-900">Activer l’agenda consolidé</p>
            <p class="text-xs text-gray-500">Tous les calendriers actifs seront regroupés dans l’Agenda de l’application.</p>
          </div>
          <input
            type="checkbox"
            checked={$unifiedCalendarSettings.enabled}
            onchange={(event: Event) => updateSettings({ enabled: (event.currentTarget as HTMLInputElement).checked })}
            class="h-5 w-5 rounded border-gray-300 text-primary-600 focus:ring-primary-500"
          />
        </label>

        <label class="flex items-center justify-between gap-4 rounded-2xl bg-gray-50 px-4 py-3">
          <div>
            <p class="font-medium text-gray-900">Afficher la source</p>
            <p class="text-xs text-gray-500">Ajoute le compte et le calendrier source sur chaque événement.</p>
          </div>
          <input
            type="checkbox"
            checked={$unifiedCalendarSettings.showSourceLabels}
            onchange={(event: Event) => updateSettings({ showSourceLabels: (event.currentTarget as HTMLInputElement).checked })}
            class="h-5 w-5 rounded border-gray-300 text-primary-600 focus:ring-primary-500"
          />
        </label>

        <label class="flex items-center justify-between gap-4 rounded-2xl bg-gray-50 px-4 py-3">
          <div>
            <p class="font-medium text-gray-900">Colorer par compte</p>
            <p class="text-xs text-gray-500">Chaque compte garde une couleur distincte pour mieux repérer les événements.</p>
          </div>
          <input
            type="checkbox"
            checked={$unifiedCalendarSettings.colorEventsByAccount}
            onchange={(event: Event) => updateSettings({ colorEventsByAccount: (event.currentTarget as HTMLInputElement).checked })}
            class="h-5 w-5 rounded border-gray-300 text-primary-600 focus:ring-primary-500"
          />
        </label>
      </div>

      <div class="rounded-3xl border border-gray-100 bg-white p-5 space-y-5 shadow-sm">
        <div>
          <div class="flex items-center justify-between gap-4">
            <div>
              <p class="font-medium text-gray-900">Fenêtre de synchronisation</p>
              <p class="text-xs text-gray-500">Nombre de jours chargés avant et après la date courante.</p>
            </div>
            <span class="text-sm font-semibold text-primary-700">{$unifiedCalendarSettings.syncWindowDays} jours</span>
          </div>
          <input
            type="range"
            min="7"
            max="90"
            step="1"
            value={$unifiedCalendarSettings.syncWindowDays}
            oninput={(event: Event) => updateSettings({ syncWindowDays: Number((event.currentTarget as HTMLInputElement).value) })}
            class="mt-4 w-full accent-primary-500"
          />
        </div>

        <label class="block">
          <span class="block text-sm font-medium text-gray-700 mb-2">Rafraîchissement automatique</span>
          <select
            class="input-field w-full"
            value={$unifiedCalendarSettings.refreshIntervalMinutes}
            onchange={(event: Event) => updateSettings({ refreshIntervalMinutes: Number((event.currentTarget as HTMLSelectElement).value) })}
          >
            <option value="5">Toutes les 5 minutes</option>
            <option value="15">Toutes les 15 minutes</option>
            <option value="30">Toutes les 30 minutes</option>
            <option value="60">Toutes les 60 minutes</option>
          </select>
        </label>
      </div>
    </section>
  </div>

  <section class="card p-6">
    <h2 class="text-xl font-semibold text-gray-900 mb-4">Flux cible</h2>
    <div class="grid gap-4 md:grid-cols-3 text-sm text-gray-600">
      <div class="rounded-2xl bg-gray-50 p-4">
        <p class="font-semibold text-gray-900 mb-2">1. Connexion OAuth</p>
        <p>Tu ajoutes un compte Google dans Paramètres.</p>
      </div>
      <div class="rounded-2xl bg-gray-50 p-4">
        <p class="font-semibold text-gray-900 mb-2">2. Sélection des calendriers</p>
        <p>Tu actives les calendriers à consolider dans l’agenda unifié.</p>
      </div>
      <div class="rounded-2xl bg-gray-50 p-4">
        <p class="font-semibold text-gray-900 mb-2">3. Vue unique dans Boussole</p>
        <p>Les événements remontent dans une seule vue avec leur source affichée.</p>
      </div>
    </div>
  </section>
</div>
