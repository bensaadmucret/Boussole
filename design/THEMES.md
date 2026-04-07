# 🎨 Propositions de Design - Boussole

## Option 1: Blue Professional (Actuel)
**Fichiers**: `index.html`, `listings.html`, `listing-form.html`, `kanban.html`

### Palette
- **Primary**: Blue-600 `#2563eb` - Confiance, professionnalisme
- **Accent**: Amber-500 `#f59e0b` - Énergie, actions
- **Background**: Gray-50 `#f9fafb` - Neutre, clean
- **Surface**: White - Cartes et contenus

### Ambiance
✅ Corporate, sérieux, rassurant  
✅ Standard SaaS/B2B  
⚠️ Peut paraître "classique" / peu différenciant

---

## Option 2: Fresh Green (Inspiré Velocity)
**Fichier**: `index-green.html`

### Palette
- **Primary**: Olive-800 → Lime gradient `#3f6212` → `#1a2e05` - Nature, croissance
- **Accent**: Lime-400/500 `#a3e635` / `#84cc16` - Énergie, optimisme
- **Background**: Green-tinted white `#fafdf7` - Fresh, organique
- **Surface**: White avec ombres vertes subtiles

### Ambiance
✅ Optimiste, dynamique, "growth mindset"  
✅ Différenciant pour une app job search  
✅ Évoque la progression, l'avancement  
⚠️ Moins corporate traditionnel

---

## 🎯 Recommandation

Pour **Boussole** (outil de recherche d'emploi), je recommande **Option 2 - Fresh Green** :

### Pourquoi ?
1. **Symbolisme** : Le vert = croissance, opportunités, nouveau départ
2. **Différenciation** : Peu d'apps RH/job utilisent cette palette
3. **Émotion** : La recherche d'emploi est stressante, le vert apaise tout en dynamisant
4. **Modernité** : Aligné avec les tendances 2024/2025 (Spotify, Notion, etc.)

### Ajustements suggérés
- Garder suffisamment de contraste pour l'accessibilité (WCAG AA)
- Utiliser le vert lime **avec parcimonie** pour les CTAs principaux
- Fonds légèrement verdâtres pour un effet "cohésif"
- Garder du blanc pur pour la lisibilité du texte

---

## 🚀 Pour décider

1. Ouvrir les deux versions côte à côte :
   ```bash
   open design/index.html        # Blue
   open design/index-green.html  # Green
   ```

2. Tester sur différents écrans (clarté, fatigue visuelle)

3. Se demander : "Quelle app me donne envie de candidater tous les jours ?"

---

## 📋 Palette détaillée (Green Theme)

```css
/* Primary - Navigation, fonds importants */
--olive-900: #1a2e05;    /* Sidebar background */
--olive-800: #3f6212;    /* Sidebar gradient start */
--lime-700:  #4d7c0f;    /* Hover states */
--lime-600:  #65a30d;    /* Primary text/icon */
--lime-500:  #84cc16;    /* Main brand color */
--lime-400:  #a3e635;    /* Accent, CTAs */
--lime-100:  #ecfccb;    /* Light backgrounds */
--lime-50:   #f7fee7;    /* Very light tint */

/* Backgrounds */
--bg-primary: #fafdf7;   /* Page background (green-tinted white) */
--bg-card:    #ffffff;   /* Cards */
--bg-hover:   #ecfccb;   /* Hover states */

/* Accents complémentaires */
--orange: #f97316;       /* Urgence, relances */
--purple: #8b5cf6;       /* Freelance, spécial */
--red:    #ef4444;       /* Refus, erreurs */
--green:  #22c55e;       /* Succès, offres */
```
