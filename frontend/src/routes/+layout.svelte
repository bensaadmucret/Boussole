<script lang="ts">
  import { page } from '$app/stores';
  import { LayoutDashboard, FileText, Briefcase, Calendar, Sparkles, Settings, File, ClipboardList } from 'lucide-svelte';
  import '../app.css';

  const navItems = [
    { path: '/', label: 'Tableau de bord', icon: LayoutDashboard },
    { path: '/listings', label: 'Annonces', icon: FileText },
    { path: '/applications', label: 'Candidatures', icon: Briefcase },
    { path: '/documents', label: 'Documents', icon: File },
    { path: '/calendar', label: 'Agenda', icon: Calendar },
    { path: '/ai', label: 'Assistant IA', icon: Sparkles },
    { path: '/reports', label: 'Rapports FT', icon: ClipboardList },
  ];

  $: currentPath = $page.url.pathname;
</script>

<div class="flex h-screen">
  <!-- Sidebar -->
  <aside class="w-64 text-white flex flex-col" style="background: linear-gradient(180deg, #3f6212 0%, #1a2e05 100%);">
    <div class="p-6 flex items-center gap-3">
      <div class="w-10 h-10 bg-accent-400 rounded-2xl flex items-center justify-center text-primary-900 font-bold text-lg shadow-lg">
        🧭
      </div>
      <span class="font-bold text-xl tracking-tight">Boussole</span>
    </div>
    
    <nav class="flex-1 px-3 space-y-1">
      {#each navItems as item}
        <a 
          href={item.path} 
          class="nav-item {currentPath === item.path ? 'bg-white/10 text-white backdrop-blur-sm' : 'text-white/70 hover:text-white'}"
        >
          <svelte:component this={item.icon} class="w-5 h-5" />
          <span class={currentPath === item.path ? 'font-medium' : ''}>{item.label}</span>
        </a>
      {/each}
    </nav>
    
    <div class="p-3">
      <a 
        href="/settings" 
        class="nav-item {currentPath === '/settings' ? 'bg-white/10 text-white' : 'text-white/70 hover:text-white'}"
      >
        <Settings class="w-5 h-5" />
        <span>Paramètres</span>
      </a>
    </div>
  </aside>

  <!-- Main Content -->
  <main class="flex-1 overflow-auto">
    <slot />
  </main>
</div>
