# 🧭 Boussole

Boussole est une application desktop légère, sécurisée et 100% locale conçue pour simplifier et structurer votre recherche d'emploi. Gérez vos candidatures, centralisez vos documents, générez des lettres de motivation sur mesure avec l'IA, synchronisez votre agenda Google et produisez des rapports PDF clairs pour vos justificatifs France Travail. Le tout, sans compromis sur votre vie privée.

## ✨ Fonctionnalités principales

- **📊 Tableau Kanban & suivi détaillé** : visualisez vos candidatures, ajoutez des notes, des tags et suivez chaque étape du processus.
- **📄 Gestion documentaire** : versionning de CV, bibliothèque de modèles, import/export de pièces jointes.
- **🤖 Assistant IA (Gemini Flash)** : génération de lettres de motivation à la volée, analyse d'offres, préparation d'entretiens.
- **📅 Synchronisation Google Calendar** : vue unifiée de vos agendas, détection d'entretiens, rappels contextuels.
- **🇫🇷 Exports France Travail** : journal des démarches, génération de rapports PDF mensuels/trimestriels prêts à imprimer ou envoyer.
- **🔒 Local-first & sécurisé** : base SQLite chiffrée, fonctionnement hors-ligne, conformité RGPD, export/suppression totale des données.

## 🛠️ Stack technique

- **🦀 Backend** : Rust (sqlx, serde, tokio)
- **🖥️ Frontend & Shell** : Tauri 2 + Svelte
- **💾 Base de données** : SQLite (stockage local)
- **🔐 Authentification** : OAuth 2.0 (Google Calendar)
- **🌐 IA** : Google Gemini Flash API
- **📦 Distribution** : Binaires natifs Windows / macOS / Linux

## 🚀 Statut

🚧 En développement actif. Architecture définie, MVP en cours.

## 📜 Licence & Vie privée

Boussole stocke l'intégralité de vos données localement. Aucune donnée n'est envoyée vers des serveurs tiers, sauf lors de l'appel explicite à l'API Gemini pour la génération de texte (configurable). Respect strict du RGPD : export complet, suppression irréversible, chiffrement au repos.

## 💡 Conseils d'intégration GitHub

- **Section About** (à droite du repo) : colle la version courte. Ajoute les topics : `tauri`, `rust`, `svelte`, `job-search`, `local-first`, `privacy`, `gemini-api`, `france-travail`
