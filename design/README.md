# Maquettes Boussole

Ce dossier contient les maquettes HTML statiques de l'application **Boussole**.

## 🎯 Objectif

Valider l'UX/UI et le design visuel avant d'implémenter le frontend Svelte dans l'application Tauri.

## 📁 Fichiers

| Fichier | Description |
|---------|-------------|
| `CHARTE_GRAPHIQUE.md` | **📐 Charte graphique complète** - Couleurs, typo, composants |
| `THEMES.md` | Comparaison des propositions de design |
| `index.html` | Dashboard - Thème Blue (legacy) |
| `index-green.html` | **Dashboard - Thème Green** (⭐ actuel) |
| `listings-green.html` | **Liste des annonces** avec filtres |
| `listing-form-green.html` | **Formulaire** d'ajout d'annonce |
| `kanban-green.html` | **Kanban** des candidatures |

## 🎨 Thème retenu : Fresh Green (Velocity-inspired)

Suite à la comparaison, le thème **Fresh Green** a été choisi pour son caractère optimiste et dynamique.

### Pourquoi ce choix ?
- **Symbolisme** : Le vert évoque la croissance, les opportunités et le renouveau 🌱
- **Différenciation** : Unique dans l'univers des apps de recherche d'emploi
- **Émotion** : Dynamisant pour une activité qui peut être stressante
- **Moderne** : Aligné avec les tendances design 2024/2025

### Fichiers du thème Green

| Fichier | Description |
|---------|-------------|
| `index-green.html` | **Dashboard** - Vue d'ensemble (⭐ actuel) |
| `listings-green.html` | **Liste des annonces** avec filtres |
| `listing-form-green.html` | **Formulaire** d'ajout d'annonce |
| `kanban-green.html` | **Kanban** des candidatures |

### Fichiers legacy (Blue - non retenus)

| Fichier | Description |
|---------|-------------|
| `index.html` | Dashboard - Thème Blue (legacy) |
| `listings.html` | Liste annonces - Thème Blue (legacy) |
| `kanban.html` | Kanban - Thème Blue (legacy) |
| `listing-form.html` | Formulaire - Thème Blue (legacy) |

### Palette détaillée

```css
/* Primary - Navigation, fonds importants */
--olive-900: #1a2e05;    /* Sidebar background */
--olive-800: #3f6212;    /* Sidebar gradient */
--lime-500:  #84cc16;    /* Main brand color */
--lime-400:  #a3e635;    /* Accent, CTAs */
--lime-100:  #ecfccb;    /* Light backgrounds */

/* Backgrounds */
--bg-primary: #fafdf7;   /* Page background (green-tinted) */
--bg-card:    #ffffff;   /* Cards */
```

Voir `THEMES.md` pour la comparaison complète.

## 🚀 Utilisation

1. **Ouvrir directement** : Double-cliquez sur n'importe quel fichier `.html` pour l'ouvrir dans le navigateur
2. **Live Server** (VS Code) : Clic droit → "Open with Live Server" pour le hot-reload
3. **Modifier** : Éditez le HTML/Tailwind, rafraîchissez pour voir les changements

## 🎨 Design System

### Couleurs
- **Primary** : Blue-600 (`#2563eb`) - Navigation, actions principales
- **Accent** : Amber-500 (`#f59e0b`) - Boutons d'action, highlights
- **Background** : Gray-50 - Fond de page
- **Surface** : White - Cartes, formulaires

### Typographie
- **Font** : Inter (Google Fonts)
- **Titres** : font-bold, text-2xl
- **Corps** : text-sm, text-gray-600

### Composants récurrents
- **Cards** : `bg-white rounded-xl shadow-sm border border-gray-100 p-6`
- **Buttons Primary** : `bg-accent-500 hover:bg-accent-600 text-white rounded-lg`
- **Buttons Secondary** : `border border-gray-300 hover:bg-gray-50 rounded-lg`
- **Inputs** : `px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-primary-500`

## 📝 TODO Maquettes

- [ ] Vue calendrier (synchronisation Google Calendar)
- [ ] Assistant IA (génération lettre de motivation)
- [ ] Page détail candidature
- [ ] Modal de confirmation / alertes
- [ ] Vue mobile responsive

## 🔗 Navigation entre maquettes

Toutes les pages sont liées entre elles via la sidebar et les boutons. Cliquez sur :
- 🧭 Logo → Dashboard
- "Annonces" dans la sidebar → Liste des annonces
- "Nouvelle annonce" → Formulaire
- "Candidatures" → Kanban
