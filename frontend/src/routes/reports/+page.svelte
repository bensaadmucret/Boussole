<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { FileDown, FileText, Calendar, CheckCircle2, AlertCircle, Loader2 } from 'lucide-svelte';
  import jsPDF from 'jspdf';
  import autoTable from 'jspdf-autotable';

  interface Application {
    id: number;
    companyName: string;
    position: string;
    status: string;
    appliedDate: string;
    responseDate: string | null;
    notes: string | null;
    contactName: string | null;
  }

  const STATUS_LABELS: Record<string, string> = {
    applied: 'Candidature envoyée',
    phone: 'Entretien téléphonique',
    interview: 'Entretien en présentiel',
    final: 'Entretien final',
    offer: 'Proposition reçue',
    rejected: 'Refus',
    withdrawn: 'Retrait'
  };

  let startDate = $state('');
  let endDate = $state('');
  let generating = $state(false);
  let savedPath = $state('');
  let error = $state('');
  let previewApps = $state<Application[]>([]);
  let loadingPreview = $state(false);

  onMount(() => {
    const now = new Date();
    const firstDay = new Date(now.getFullYear(), now.getMonth(), 1);
    startDate = firstDay.toISOString().split('T')[0];
    endDate = now.toISOString().split('T')[0];
    loadPreview();
  });

  async function loadPreview() {
    if (!startDate || !endDate) return;
    loadingPreview = true;
    try {
      const all: Application[] = await invoke('get_applications');
      previewApps = all.filter(app => {
        const d = app.appliedDate?.split('T')[0] ?? '';
        return d >= startDate && d <= endDate;
      });
    } catch (e) {
      console.error(e);
    } finally {
      loadingPreview = false;
    }
  }

  $effect(() => {
    if (startDate && endDate) loadPreview();
  });

  function formatDate(dateStr: string | null | undefined): string {
    if (!dateStr) return '-';
    const d = new Date(dateStr);
    return d.toLocaleDateString('fr-FR');
  }

  async function generatePdf() {
    if (!startDate || !endDate) {
      error = 'Veuillez sélectionner une période.';
      return;
    }
    if (previewApps.length === 0) {
      error = 'Aucune candidature dans cette période.';
      return;
    }

    generating = true;
    savedPath = '';
    error = '';

    try {
      const doc = new jsPDF({ orientation: 'landscape', unit: 'mm', format: 'a4' });

      // Header
      doc.setFillColor(63, 98, 18);
      doc.rect(0, 0, 297, 28, 'F');
      doc.setTextColor(255, 255, 255);
      doc.setFontSize(18);
      doc.setFont('helvetica', 'bold');
      doc.text('Rapport de Recherche d\'Emploi', 14, 13);
      doc.setFontSize(10);
      doc.setFont('helvetica', 'normal');
      doc.text('France Travail — Justificatif d\'actions de recherche d\'emploi', 14, 21);

      // Period info
      doc.setTextColor(60, 60, 60);
      doc.setFontSize(11);
      doc.setFont('helvetica', 'bold');
      doc.text(`Période : du ${formatDate(startDate)} au ${formatDate(endDate)}`, 14, 38);
      doc.setFont('helvetica', 'normal');
      doc.setFontSize(10);
      doc.text(`Nombre de candidatures : ${previewApps.length}`, 14, 45);
      doc.text(`Document généré le : ${new Date().toLocaleDateString('fr-FR')}`, 14, 51);

      // Table
      autoTable(doc, {
        startY: 58,
        head: [[
          'Date de candidature',
          'Entreprise',
          'Poste',
          'Statut',
          'Date de réponse',
          'Contact',
          'Notes'
        ]],
        body: previewApps.map(app => [
          formatDate(app.appliedDate),
          app.companyName,
          app.position,
          STATUS_LABELS[app.status] ?? app.status,
          formatDate(app.responseDate),
          app.contactName ?? '-',
          app.notes ? app.notes.substring(0, 60) + (app.notes.length > 60 ? '…' : '') : '-'
        ]),
        headStyles: {
          fillColor: [63, 98, 18],
          textColor: 255,
          fontStyle: 'bold',
          fontSize: 9
        },
        bodyStyles: { fontSize: 8, textColor: [40, 40, 40] },
        alternateRowStyles: { fillColor: [245, 250, 240] },
        columnStyles: {
          0: { cellWidth: 30 },
          1: { cellWidth: 45 },
          2: { cellWidth: 45 },
          3: { cellWidth: 40 },
          4: { cellWidth: 28 },
          5: { cellWidth: 35 },
          6: { cellWidth: 'auto' }
        },
        margin: { left: 14, right: 14 },
        styles: { overflow: 'linebreak' }
      });

      // Footer on each page
      const pageCount = (doc as any).internal.getNumberOfPages();
      for (let i = 1; i <= pageCount; i++) {
        doc.setPage(i);
        doc.setFontSize(8);
        doc.setTextColor(150);
        doc.text(
          `Page ${i} / ${pageCount} — Boussole — Document confidentiel`,
          14,
          doc.internal.pageSize.height - 8
        );
      }

      // Send bytes to backend to save in Downloads
      const pdfBytes = doc.output('arraybuffer');
      const byteArray = Array.from(new Uint8Array(pdfBytes));
      const month = startDate.substring(0, 7).replace('-', '_');
      const filename = `rapport_ft_${month}.pdf`;

      const path: string = await invoke('save_report_pdf', {
        data: byteArray,
        filename
      });

      savedPath = path;
    } catch (e: any) {
      error = `Erreur : ${e?.message ?? e}`;
    } finally {
      generating = false;
    }
  }
</script>

<div class="p-6 max-w-5xl mx-auto">
  <!-- Header -->
  <div class="flex items-center gap-3 mb-6">
    <div class="w-10 h-10 bg-primary-100 rounded-xl flex items-center justify-center">
      <FileText class="w-5 h-5 text-primary-700" />
    </div>
    <div>
      <h1 class="text-2xl font-bold text-gray-900">Rapports France Travail</h1>
      <p class="text-sm text-gray-500">Génère un PDF de tes candidatures pour justifier ta recherche active d'emploi</p>
    </div>
  </div>

  <!-- Period selector -->
  <div class="bg-white rounded-2xl border border-gray-200 p-6 mb-6 shadow-sm">
    <h2 class="font-semibold text-gray-800 mb-4 flex items-center gap-2">
      <Calendar class="w-4 h-4 text-primary-600" />
      Sélectionner la période
    </h2>
    <div class="flex gap-4 items-end">
      <div class="flex-1">
        <label for="start-date" class="block text-sm font-medium text-gray-700 mb-1">Du</label>
        <input
          id="start-date"
          type="date"
          bind:value={startDate}
          class="input-field w-full"
        />
      </div>
      <div class="flex-1">
        <label for="end-date" class="block text-sm font-medium text-gray-700 mb-1">Au</label>
        <input
          id="end-date"
          type="date"
          bind:value={endDate}
          class="input-field w-full"
        />
      </div>
      <button
        onclick={generatePdf}
        disabled={generating || previewApps.length === 0}
        class="btn-primary inline-flex items-center gap-2 whitespace-nowrap disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {#if generating}
          <Loader2 class="w-4 h-4 animate-spin" />
          Génération…
        {:else}
          <FileDown class="w-4 h-4" />
          Générer le PDF
        {/if}
      </button>
    </div>
  </div>

  <!-- Feedback -->
  {#if savedPath}
    <div class="flex items-start gap-3 bg-green-50 border border-green-200 rounded-xl p-4 mb-6">
      <CheckCircle2 class="w-5 h-5 text-green-600 shrink-0 mt-0.5" />
      <div>
        <p class="font-medium text-green-800">PDF enregistré avec succès !</p>
        <p class="text-sm text-green-700 mt-0.5 break-all">{savedPath}</p>
      </div>
    </div>
  {/if}

  {#if error}
    <div class="flex items-start gap-3 bg-red-50 border border-red-200 rounded-xl p-4 mb-6">
      <AlertCircle class="w-5 h-5 text-red-500 shrink-0 mt-0.5" />
      <p class="text-sm text-red-700">{error}</p>
    </div>
  {/if}

  <!-- Preview table -->
  <div class="bg-white rounded-2xl border border-gray-200 shadow-sm overflow-hidden">
    <div class="px-6 py-4 border-b border-gray-100 flex items-center justify-between">
      <h2 class="font-semibold text-gray-800">
        Aperçu des candidatures
        {#if !loadingPreview}
          <span class="ml-2 text-sm font-normal text-gray-500">({previewApps.length} candidature{previewApps.length > 1 ? 's' : ''})</span>
        {/if}
      </h2>
    </div>

    {#if loadingPreview}
      <div class="flex items-center justify-center py-12">
        <Loader2 class="w-6 h-6 animate-spin text-primary-500" />
      </div>
    {:else if previewApps.length === 0}
      <div class="text-center py-12 text-gray-400">
        <FileText class="w-10 h-10 mx-auto mb-2 opacity-40" />
        <p>Aucune candidature sur cette période</p>
      </div>
    {:else}
      <div class="overflow-x-auto">
        <table class="w-full text-sm">
          <thead>
            <tr class="bg-gray-50 text-gray-600 text-xs uppercase tracking-wider">
              <th class="text-left px-4 py-3 font-medium">Date</th>
              <th class="text-left px-4 py-3 font-medium">Entreprise</th>
              <th class="text-left px-4 py-3 font-medium">Poste</th>
              <th class="text-left px-4 py-3 font-medium">Statut</th>
              <th class="text-left px-4 py-3 font-medium">Réponse</th>
            </tr>
          </thead>
          <tbody class="divide-y divide-gray-100">
            {#each previewApps as app}
              <tr class="hover:bg-gray-50 transition-colors">
                <td class="px-4 py-3 text-gray-700">{formatDate(app.appliedDate)}</td>
                <td class="px-4 py-3 font-medium text-gray-900">{app.companyName}</td>
                <td class="px-4 py-3 text-gray-700">{app.position}</td>
                <td class="px-4 py-3">
                  <span class="px-2 py-0.5 rounded-full text-xs font-medium
                    {app.status === 'offer' ? 'bg-green-100 text-green-700' :
                     app.status === 'rejected' ? 'bg-red-100 text-red-600' :
                     app.status === 'interview' || app.status === 'final' ? 'bg-blue-100 text-blue-700' :
                     'bg-gray-100 text-gray-600'}">
                    {STATUS_LABELS[app.status] ?? app.status}
                  </span>
                </td>
                <td class="px-4 py-3 text-gray-500">{formatDate(app.responseDate)}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </div>
</div>
