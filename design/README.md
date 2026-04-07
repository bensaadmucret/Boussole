# Maquettes Boussole

Ce dossier contient les maquettes HTML statiques de l'application **Boussole**.

## 🎯 Objectif

Valider l'UX/UI et le design visuel avant d'implémenter le frontend Svelte dans l'application Tauri.

## 📁 Fichiers

| Fichier | Description |
|---------|-------------|
| `index.html` | Dashboard - Vue d'ensemble (Thème Blue) |
| `index-green.html` | Dashboard - Vue d'ensemble (Thème Green) |
| `listings.html` | Liste des annonces sauvegardées avec filtres |
| `listing-form.html` | Formulaire d'ajout/modification d'une annonce |
| `kanban.html` | Tableau Kanban des candidatures |
| `THEMES.md` | Comparaison des propositions de design |

## 🎨 Deux propositions de design

Deux palettes sont disponibles pour comparaison :

1. **Blue Professional** (`index.html`) - Classique, corporate, rassurant
2. **Fresh Green** (`index-green.html`) - Inspiré du design Velocity, optimiste et dynamique

👉 Voir `THEMES.md` pour la comparaison détaillée et la palette complète.

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
