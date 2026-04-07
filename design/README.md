# Maquettes Boussole

Ce dossier contient les maquettes HTML statiques de l'application **Boussole**.

## 🎯 Objectif

Valider l'UX/UI et le design visuel avant d'implémenter le frontend Svelte dans l'application Tauri.

## 📁 Fichiers

| Fichier | Description |
|---------|-------------|
| `CHARTE_GRAPHIQUE.md` | **📐 Charte graphique complète** - Couleurs, typo, composants, accessibilité |
| `THEMES.md` | Historique de la comparaison des propositions de design |
| `index.html` | **Dashboard** - Vue d'ensemble avec stats et activité |
| `listings.html` | **Liste des annonces** sauvegardées avec filtres |
| `listing-form.html` | **Formulaire** d'ajout/modification d'une annonce |
| `kanban.html` | **Kanban** des candidatures |

## 🎨 Thème : Fresh Green (Velocity-inspired)

Le thème **Fresh Green** a été choisi pour son caractère optimiste et dynamique.

### Pourquoi ce choix ?
- **Symbolisme** : Le vert évoque la croissance, les opportunités et le renouveau 🌱
- **Différenciation** : Unique dans l'univers des apps de recherche d'emploi
- **Émotion** : Dynamisant pour une activité qui peut être stressante
- **Moderne** : Aligné avec les tendances design 2024/2025

### Palette
- **Sidebar** : Olive gradient (`#3f6212` → `#1a2e05`)
- **CTAs** : Lime gradient (`#a3e635` → `#84cc16`)
- **Fond** : Green-tinted white (`#fafdf7`)
- **Accents** : Lime shades selon contexte

Voir `CHARTE_GRAPHIQUE.md` pour la palette complète et tous les composants.

## 🚀 Utilisation

1. **Ouvrir directement** : Double-cliquez sur n'importe quel fichier `.html` pour l'ouvrir dans le navigateur
2. **Live Server** (VS Code) : Clic droit → "Open with Live Server" pour le hot-reload
3. **Modifier** : Éditez le HTML/Tailwind, rafraîchissez pour voir les changements

## 🧭 Navigation entre maquettes

Toutes les pages sont liées entre elles via la sidebar et les boutons :
- 🧭 Logo → Dashboard
- "Annonces" dans la sidebar → Liste des annonces
- "Nouvelle annonce" → Formulaire
- "Candidatures" → Kanban

## 📝 TODO Maquettes

- [ ] Vue calendrier (synchronisation Google Calendar)
- [ ] Assistant IA (génération lettre de motivation)
- [ ] Page détail candidature
- [ ] Modal de confirmation / alertes
- [ ] Vue mobile responsive
