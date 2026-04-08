# Google Calendar OAuth

Cette page explique comment connecter un compte Google Calendar dans Boussole.

## Pourquoi un Google Client ID ?

Boussole utilise le flux OAuth Google en local. Le champ **Google Client ID** identifie ton application Google auprès de l’API Google Calendar.

Sans ce Client ID, l’application ne peut pas ouvrir le flux d’autorisation ni échanger le code OAuth contre un token.

## Comment obtenir le Client ID

1. Ouvre la [Google Cloud Console](https://console.cloud.google.com/).
2. Sélectionne ou crée un projet.
3. Active les APIs suivantes :
   - Google Calendar API
   - OAuth consent screen si nécessaire
4. Va dans **APIs & Services** → **Credentials**.
5. Crée des **OAuth client IDs**.
6. Choisis un type **Desktop app**.
7. Copie la valeur affichée, au format :

```text
xxxxxxxxxxxx.apps.googleusercontent.com
```

## Où le renseigner dans Boussole

1. Ouvre **Paramètres**.
2. Colle le Client ID dans le champ **Connexion Google OAuth**.
3. Clique sur **Enregistrer le Client ID**.
4. Clique ensuite sur **Associer un compte Gmail**.

## Ce qui se passe ensuite

- Boussole ouvre le navigateur système.
- Tu te connectes à ton compte Google.
- Google redirige vers le callback local de l’application.
- Le refresh token est stocké dans le coffre système, pas en clair dans la base SQLite.
- Les calendriers actifs sont synchronisés dans la vue **Agenda**.

## En cas de problème

- **Le bouton ne fonctionne pas** : vérifie que le Client ID a bien été enregistré.
- **Le flux OAuth refuse la connexion** : vérifie que l’application Google a bien été créée comme app desktop.
- **Aucun calendrier ne s’affiche** : assure-toi d’avoir autorisé la lecture du calendrier lors de la connexion.
