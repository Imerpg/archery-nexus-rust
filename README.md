# 🏹 Archery Nexus

Application native multiplateforme ultra-léger et moderne pour la gestion de compétitions de tir à l'arc.

---

## 🏗️️ Architecture & Stack Technique

- **Back-end :** **Rust** (performance, sécurité mémoire, binaire unique).
- **API REST & WebSockets :** **Axum** (serveur léger, communication en temps réel avec les tablettes sur le pas de tir).
- **Interface Graphique (UI) :** **Dioxus** + **Tailwind CSS** (UI native, fluide, responsive : Windows, macOS, Linux).
- **Base de données :** **SQLite (mode WAL)** (stockage local ultra-rapide, 100 % hors-ligne, zéro configuration externe).

---

## ⚙️ Principes de Fonctionnement

- **Binaire unique & Zéro-installation :** L'API et l'UI sont packagées dans un seul exécutable. Un double-clic suffit pour lancer l'application.
- **Exécution en arrière-plan (System Tray) :**
    - L'API Axum continue de tourner dans la zone de notification (près de l'horloge) même si la fenêtre principale est fermée.
    - Permet de maintenir la réception des scores depuis les tablettes sans interruption.

---

## 🎯 Philosophie Produit & Ergonomie

- **L'anti-Ianseo :** Code ultra-modulaire, maintenance facile, architecture moderne et lisible.
- **Simplicité avant tout :** Le cœur de l'application reste minimaliste ; la complexité est déportée dans des modules de règles dynamiques.
- **Base de données épurée :** Stockage strict de la donnée brute immuable (archers, tirs, impacts) sans calculs complexes enregistrés en dur.
- **Guidage visuel & Zéro frustration :**
    - Infobulles contextuelles (_tooltips_) et assistants pas à pas (_wizards_) pour accompagner les bénévoles novices.
    - Documentation et Wiki intégrés.

---

## 💡 Idées en Vrac & Évolutions

### 1. Le « Workshop » de Règles & Duels

- **Moteur de règles modulaire (JSON / WebAssembly) :**
    - Formats officiels (Fédéral, TAEN, Tir à 18m, Campagne, 3D).
    - Règles personnalisées (tir de club, tir de la Saint-Sébastien, formats d'entraînement).
    - Modes Duels / Matchs (Set System, score cumulé, tir de barrage / _Arrow-off_).

### 2. Saisie sur le Pas de Tir (Tablettes / Smartphones)

- **PWA / Interface Web locale :** Connexion des marques depuis le navigateur d'un smartphone via le Wi-Fi local du PC.
- **Tolérance aux pannes réseau :** Stockage local des volées sur la tablette si le réseau coupe, avec synchronisation automatique à la reconnexion.
- **Saisie tactile hybride :** Choix entre un pavé numérique à grands boutons ou un blason virtuel tactile pour pointer les impacts.

### 3. Affichage & Animation

- **Vue Grand Écran (Vidéoprojecteur / TV) :** Projections du chrono, de l'ordre de tir (A/B - C/D) et du classement en direct avec animations.
- **Chronomètre réseau :** Gestion des feux de tir (Vert / Orange / Rouge) contrôlable depuis le PC de la greffe.

### 4. Import / Export & Compatibilité

- **Importation :** Chargement rapide des listes d'inscrits depuis des fichiers CSV ou JSON.
- **Exportation :** Édition automatique des feuilles de marque au format PDF et sauvegardes JSON.
