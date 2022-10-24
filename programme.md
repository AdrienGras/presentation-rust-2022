# Programme de la présentation

**Durée: 35-40min**

## Plan

* Présentation du sommaire (1m)

* Présentation personnelle (1m)

* A qui se destine la présentation ? (1m)
    * tech
    * CTO
    * les autres

* Présentation du langage (2m)
    * Historique
    * Présentation
        * Remplacement C/C++
        * Ultra rapide mais sécurisé
        * Langage préféré par les devs depuis 3 ans

* Installation (2-3m)
    * One-line install en 1 minute
    * configuration VS en 2 minutes

* `println!("Hello world ! 🦀");` (2m)

* La première app (5m)
    * Cargo, le packager
    * Organisation par module
    * Visibilité
    * Le jeu du juste prix (illustration synthaxe en 5 min)
        * v fonctionnel
        * match pour print +/-
        * struct (chiffre à deviner, score) + impl

* Le déploiement (2m)
    * un seul binaire, même pour un micro service
    * lançable en une seule ligne
    * containeurisable en 10 lignes
    * outils CI/CD

* Principaux points techniques (4m)
    * Orientation bas niveau
        * gestion de la mémoire manuelle (PAS DE GC)
        * pointeurs, et smart pointers
    * thread safe intégré
    * Memory safe
        * typage strict
        * accès mémoires validés par le compilo
        * variables immutables par défaut
        * messages safe entre threads
        * PAS DE NULL
    * inférence de type
    * filtrage par motifs
    * généricité
    * 1 exécutable avec tout dedans

* Rust, on en fait quoi ? (3m)
    * on en fait... (système, CLI, ETL, haute perf, micro-webservices, remplacement vieux code C/C++)
    * on ne fait pas pour l'instant (web, IHM/clients lourds)

* Niveau performances ? (2m)
    * exemple concrets, bench (fibo, erateustène ?)

* Les pours & contres (3m)
    * pours
        * Code performant
        * Code safe
    * contres
        * Recrutement
        * Courbe d'apprentissage
        * Encore jeune

* Rust versus le reste du monde (2m)
    * Rust VS langages (Go, Java, PHP, Python, JS/TS, C++)

* Rust, c'est utilisé dans quoi ? (2m)

* Et dans 10 ans ? (2m)
    * Apps mobiles (intégration Flutter pour code métier)
    * Apps hybride (EGUI, TauRi)
    * Web (Yew, WASM)
    * API (Rocket, Actix)