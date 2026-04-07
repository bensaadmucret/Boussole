# 📐 Charte Graphique - Boussole

**Version** : 1.0  
**Date** : Avril 2025  
**Thème** : Fresh Green (Velocity-inspired)

---

## 🎯 Identité Visuelle

### Concept
Boussole accompagne les candidats dans leur recherche d'emploi avec optimisme et clarté. Le design évoque la **croissance**, la **progression** et les **opportunités** à travers une palette verte dynamique et rafraîchissante.

### Mots-clés
`Croissance` `Opportunités` `Clarté` `Optimisme` `Progrès` `Localisation`

---

## 🎨 Palette de Couleurs

### Couleurs Primaires

| Token | Valeur | Usage |
|-------|--------|-------|
| `--primary-900` | `#1a2e05` | Sidebar background |
| `--primary-800` | `#3f6212` | Gradient start, textes importants |
| `--primary-700` | `#4d7c0f` | Hover states, liens |
| `--primary-600` | `#65a30d` | Icons, secondary actions |
| `--primary-500` | `#84cc16` | **Brand color principale** |
| `--primary-400` | `#a3e635` | CTAs gradient, highlights |
| `--primary-200` | `#d9f99d` | Light backgrounds, badges |
| `--primary-100` | `#ecfccb` | Hover backgrounds, alerts light |
| `--primary-50` | `#f7fee7` | Très light backgrounds |

### Couleurs de Fond

| Token | Valeur | Usage |
|-------|--------|-------|
| `--bg-primary` | `#fafdf7` | Page background (green-tinted white) |
| `--bg-card` | `#ffffff` | Cartes, surfaces élevées |
| `--bg-hover` | `#ecfccb` | Hover sur éléments interactifs |
| `--bg-sidebar` | `linear-gradient(180deg, #3f6212 0%, #1a2e05 100%)` | Navigation |

### Couleurs de Texte

| Token | Valeur | Usage |
|-------|--------|-------|
| `--text-primary` | `#1f2937` | Titres, texte principal |
| `--text-secondary` | `#6b7280` | Sous-titres, descriptions |
| `--text-muted` | `#9ca3af` | Placeholders, hints |
| `--text-inverse` | `#ffffff` | Sur fonds foncés |

### Couleurs Fonctionnelles

| Token | Valeur | Usage |
|-------|--------|-------|
| `--success` | `#22c55e` | Succès, offres acceptées |
| `--success-light` | `#dcfce7` | Fonds de succès |
| `--warning` | `#f97316` | Relances, urgences |
| `--warning-light` | `#ffedd5` | Fonds d'alerte |
| `--error` | `#ef4444` | Refus, erreurs |
| `--error-light` | `#fee2e2` | Fonds d'erreur |
| `--info` | `#3b82f6` | Informations, liens externes |
| `--info-light` | `#dbeafe` | Fonds d'info |

### Couleurs de Statut (Kanban)

| Statut | Couleur | Background |
|--------|---------|------------|
| Wishlist | `#9ca3af` | `bg-gray-100` |
| Candidature envoyée | `#84cc16` | `bg-primary-100` |
| Entretien | `#a3e635` | `bg-accent-400/20` |
| Offre reçue | `#22c55e` | `bg-green-100` |
| Refus | `#ef4444` | `bg-red-50` |

---

## 🔤 Typographie

### Police

**Primary** : Inter (Google Fonts)  
**Fallback** : -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif

```css
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap');
```

### Échelle de Tailles

| Élément | Taille | Poids | Line-height | Letter-spacing |
|---------|--------|-------|-------------|----------------|
| H1 (Page) | `text-3xl` (30px) | 700 (bold) | 1.2 | -0.02em |
| H2 (Section) | `text-xl` (20px) | 700 (bold) | 1.3 | -0.01em |
| H3 (Card) | `text-lg` (18px) | 600 (semibold) | 1.4 | 0 |
| Body | `text-base` (16px) | 400 (normal) | 1.6 | 0 |
| Small | `text-sm` (14px) | 400 (normal) | 1.5 | 0 |
| XSmall | `text-xs` (12px) | 500 (medium) | 1.4 | 0.01em |
| Label | `text-sm` (14px) | 600 (semibold) | 1.4 | 0 |

### Usage Typographique

```html
<!-- Titre de page -->
<h1 class="text-3xl font-bold text-gray-900">Tableau de bord</h1>

<!-- Sous-titre -->
<p class="text-gray-500 mt-1">Voici l'actualité de votre recherche</p>

<!-- Titre de carte -->
<h2 class="font-bold text-xl text-gray-900">Annonces récentes</h2>

<!-- Label de formulaire -->
<label class="block text-sm font-semibold text-gray-700 mb-2">Nom de la société</label>

<!-- Texte secondaire -->
<span class="text-sm text-gray-500">TechCorp • Paris</span>
```

---

## 🧩 Composants

### Boutons

#### Bouton Primaire (CTA)
```html
<button class="bg-gradient-to-br from-lime-400 to-lime-500 hover:from-lime-500 hover:to-lime-600 text-white font-semibold px-6 py-3 rounded-2xl shadow-lg shadow-lime-500/30 transition transform hover:scale-105">
  Nouvelle annonce
</button>
```

**Usage** : Actions principales (créer, sauvegarder, postuler)

#### Bouton Secondaire
```html
<button class="px-5 py-2.5 text-primary-700 hover:bg-primary-50 rounded-xl font-medium transition border-2 border-transparent hover:border-primary-200">
  Voir
</button>
```

**Usage** : Actions secondaires (voir détails, modifier)

#### Bouton Tertiaire
```html
<button class="px-8 py-3 border-2 border-gray-200 text-gray-700 font-semibold rounded-xl hover:bg-gray-50 transition">
  Annuler
</button>
```

**Usage** : Actions de retour/annulation

---

### Cartes

#### Carte Standard
```html
<div class="bg-white rounded-2xl shadow-sm shadow-lime-500/5 border border-lime-500/10 p-6 hover:shadow-lg hover:shadow-lime-500/10 transition">
  <!-- Contenu -->
</div>
```

#### Carte de Statistique
```html
<div class="bg-white rounded-3xl shadow-sm shadow-lime-500/5 border border-lime-500/10 p-6">
  <div class="flex items-center justify-between mb-4">
    <div class="w-14 h-14 bg-primary-100 rounded-2xl flex items-center justify-center">
      <!-- Icon -->
    </div>
    <span class="text-3xl font-bold text-primary-700">12</span>
  </div>
  <p class="text-sm text-gray-500 font-medium">Annonces sauvegardées</p>
</div>
```

#### Carte Kanban
```html
<div class="bg-white rounded-2xl shadow-sm shadow-black/5 border border-lime-500/10 p-4 hover:shadow-lg hover:shadow-lime-500/15 hover:-translate-y-0.5 transition cursor-move">
  <!-- Contenu tâche -->
</div>
```

---

### Formulaires

#### Input Standard
```html
<input 
  type="text" 
  class="w-full px-4 py-3 bg-gray-50 border-2 border-gray-200 rounded-xl focus:outline-none focus:border-lime-500 focus:shadow-[0_0_0_3px_rgba(132,204,22,0.2)] transition"
  placeholder="Placeholder..."
>
```

#### Input avec Icon
```html
<div class="relative">
  <svg class="absolute left-4 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" ...></svg>
  <input 
    type="text" 
    class="w-full pl-12 pr-4 py-3 bg-white border border-gray-200 rounded-2xl focus:outline-none focus:border-lime-500 transition"
    placeholder="Rechercher..."
  >
</div>
```

#### Select
```html
<select class="w-full px-4 py-3 bg-white border border-gray-200 rounded-2xl focus:outline-none focus:border-lime-500 transition appearance-none bg-[url('data:image/svg+xml,...')] bg-no-repeat bg-right-4">
  <option>Option 1</option>
</select>
```

#### Textarea
```html
<textarea 
  rows="10" 
  class="w-full px-4 py-4 bg-gray-50 border-2 border-gray-200 rounded-xl focus:outline-none focus:border-lime-500 focus:shadow-[0_0_0_3px_rgba(132,204,22,0.2)] transition resize-none"
  placeholder="Description..."
></textarea>
```

---

### Badges & Tags

#### Badge de Type
```html
<span class="px-3 py-1.5 bg-primary-100 text-primary-700 text-xs font-semibold rounded-xl">CDI</span>
<span class="px-3 py-1.5 bg-green-100 text-green-700 text-xs font-semibold rounded-xl">Hybride</span>
<span class="px-3 py-1.5 bg-purple-100 text-purple-700 text-xs font-semibold rounded-xl">Freelance</span>
```

#### Tag Technologie
```html
<span class="px-3 py-1 bg-gray-100 text-gray-700 text-sm rounded-xl font-medium">React</span>
<span class="px-3 py-1 bg-primary-100 text-primary-700 text-sm rounded-xl font-medium">TypeScript</span>
```

---

### Navigation

#### Sidebar Item (Actif)
```html
<a href="#" class="flex items-center gap-3 px-4 py-3 bg-white/10 rounded-2xl text-white backdrop-blur-sm">
  <svg class="w-5 h-5" ...></svg>
  <span class="font-medium">Tableau de bord</span>
</a>
```

#### Sidebar Item (Inactif)
```html
<a href="#" class="flex items-center gap-3 px-4 py-3 text-white/70 rounded-2xl hover:text-white hover:bg-lime-500/15 transition">
  <svg class="w-5 h-5" ...></svg>
  <span>Annonces</span>
</a>
```

---

## 📐 Espacements & Layout

### Échelle d'Espacement

| Token | Valeur | Usage |
|-------|--------|-------|
| `space-1` | 4px | Gaps très petits |
| `space-2` | 8px | Icon + text, tags |
| `space-3` | 12px | Padding boutons small |
| `space-4` | 16px | Standard padding |
| `space-5` | 20px | Card padding |
| `space-6` | 24px | Section padding |
| `space-8` | 32px | Grand espacements |

### Layout Principal

```
┌─────────────────────────────────────────────────┐
│  Sidebar (w-64)  │  Main Content                │
│                  │  ┌──────────────────────┐     │
│  🧭 Boussole     │  │ Header (px-8 py-6) │     │
│                  │  └──────────────────────┘     │
│  ─────────────   │                               │
│  Dashboard       │  ┌──────────────────────┐     │
│  Annonces ✓     │  │                      │     │
│  Candidatures    │  │   Content Area       │     │
│  Agenda          │  │   (px-8 pb-8)        │     │
│  Assistant IA    │  │                      │     │
│                  │  └──────────────────────┘     │
│  ─────────────   │                               │
│  Paramètres      │                               │
└─────────────────────────────────────────────────┘
```

### Breakpoints

| Breakpoint | Valeur | Usage |
|------------|--------|-------|
| `sm` | 640px | Mobile landscape |
| `md` | 768px | Tablette |
| `lg` | 1024px | Desktop small |
| `xl` | 1280px | Desktop standard |
| `2xl` | 1536px | Desktop large |

---

## 🌟 Effets & Animations

### Ombres

| Élément | Shadow |
|---------|--------|
| Card default | `0 2px 12px rgba(132, 204, 22, 0.08)` |
| Card hover | `0 4px 16px rgba(132, 204, 22, 0.12)` |
| Button CTA | `0 4px 14px rgba(132, 204, 22, 0.35)` |
| Button hover | `0 6px 20px rgba(132, 204, 22, 0.45)` |
| Dropdown | `0 10px 40px rgba(0, 0, 0, 0.1)` |

### Transitions

| Élément | Durée | Easing |
|---------|-------|--------|
| Hover | 150ms | ease-out |
| Focus ring | 200ms | ease-in-out |
| Card lift | 200ms | cubic-bezier(0.4, 0, 0.2, 1) |
| Modal | 300ms | cubic-bezier(0.4, 0, 0.2, 1) |

### Hover Effects

```css
/* Card lift */
.card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 16px rgba(132, 204, 22, 0.12);
}

/* Button scale */
.btn-primary:hover {
  transform: scale(1.05);
}

/* Nav item glow */
.nav-item:hover {
  background: rgba(132, 204, 22, 0.15);
}

/* Input glow */
.input:focus {
  box-shadow: 0 0 0 3px rgba(132, 204, 22, 0.2);
}
```

---

## ♿ Accessibilité

### Contrastes Minimums (WCAG 2.1 AA)

| Combinaison | Ratio | Status |
|-------------|-------|--------|
| `text-gray-900` sur `bg-white` | 15.3:1 | ✅ Pass |
| `text-white` sur sidebar (`#1a2e05`) | 14.2:1 | ✅ Pass |
| `text-primary-700` sur `bg-primary-100` | 4.6:1 | ✅ Pass |
| `text-lime-600` sur `bg-white` | 3.2:1 | ⚠️ Large text only |

### Règles à Suivre

1. **Texte** : Toujours utiliser `text-gray-900` sur fonds clairs, `text-white` sur fonds sombres
2. **Focus** : Tous les éléments interactifs doivent avoir un ring de focus visible
3. **Boutons** : Ratio minimum 4.5:1 pour le texte, 3:1 pour les icônes
4. **États** : Les états disabled doivent avoir un contraste réduit mais rester identifiables

### Exemple Accessible

```html
<button 
  class="bg-gradient-to-br from-lime-400 to-lime-500 text-white font-semibold px-6 py-3 rounded-2xl shadow-lg shadow-lime-500/30 transition transform hover:scale-105 focus:outline-none focus:ring-4 focus:ring-lime-500/30"
  aria-label="Créer une nouvelle annonce d'emploi"
>
  Nouvelle annonce
</button>
```

---

## 📱 Responsive

### Mobile (< 768px)

- Sidebar devient drawer/bottom nav
- Cards passent en pleine largeur
- Kanban devient liste vertical
- Typography réduite d'une taille

### Tablette (768px - 1024px)

- Sidebar compact (icons + text small)
- 2 colonnes pour les grids
- Kanban scroll horizontal

### Desktop (> 1024px)

- Layout complet avec sidebar
- Grids de 3-4 colonnes
- Kanban multi-colonnes visible

---

## 🎭 Dark Mode (Futur)

### Palette Inversée Proposée

| Light | Dark |
|-------|------|
| `bg-[#fafdf7]` | `bg-[#0f1a05]` |
| `bg-white` | `bg-[#1a2e0a]` |
| `text-gray-900` | `text-gray-100` |
| `text-gray-500` | `text-gray-400` |
| `border-gray-200` | `border-gray-700` |
| `--primary-500` | `--primary-400` |

---

## 📚 Ressources

- **Font** : [Inter - Google Fonts](https://fonts.google.com/specimen/Inter)
- **Icons** : [Heroicons](https://heroicons.com/) (outline, 24px)
- **Palette** : Tailwind Lime + custom Olive
- **Inspiration** : Velocity Fitness App (green vibes)

---

## ✅ Checklist Implémentation

- [ ] Configurer Tailwind avec les couleurs custom
- [ ] Importer la font Inter
- [ ] Créer les composants de base (Button, Card, Input)
- [ ] Implémenter le layout Sidebar + Main
- [ ] Vérifier les contrastes avec un outil (WCAG)
- [ ] Tester les animations de hover/focus
- [ ] Valider le responsive mobile
