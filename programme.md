# Programme de la présentation

Consignes de lecture du plan
* Les parties en italiques sont des didascalies, énoncés à l'oral
* Le reste du texte compose les slides
* Les titres sont les parties du powerpoint (splitter full page avec juste le titre)

> **Durée: 35-40min**

# Rust, ou le langage pour les 10 ans à venir

* A qui se destine la présentation ? (1m)
    * tech
        * *se lancer avec le langage*
        * *connaitre de nouvelles manières de faire*
    * CTO
        * *connaitre de nouvelles alternatives pour produire du soft adapté*
    * les autres
        * *la curiosité*
    * *y'en aura pour tout le monde dans la prez de toute façon*

## Mais... C'est qui ?

* Présentation personnelle (1m)
    * Architecte solution chez Owlnext​
        * Algorithmie avancée & R&D
        * Architecture applicative
        * Infrastructure & sécurité
    * Auparavant Head of software & sytem architecture chez iBanFirst​
        * Equipe de 5 personne
        * En charge de l'infrastructure applicative et bancaire
        * Chargé des projets R&D
    * Quelques réalisations​
        * Conception d’un système de chiffrage end-to-end : Heimdall​ (https://heimdall-tech.fr/)
        * Création d’un système de téléphonie VoIP via WebRTC avec IVRs, Files d’attentes, redirections d’appels, connexions CRM, etc.​
        * Refactoring d’un serveur de connexion aux salles de marchés internationale

## Rust kezako ?

* Présentation du langage (2m)
    * Présentation
        * Langage bas niveau, remplacement C++
        * Deux mots : performances & sécurité
    * Utilisé principalement pour
        * CLI & backend
        * front-end avec WebAssembly
        * services réseau
        * applications embarqués
    * Créé en 2006 par Graydon Hoare, puis repris en 2010 par la fondation Mozilla
    * Version actuelle : 1.65.0 (2 Novembre)

* Pourquoi choisir Rust ?
    * Il a
        * Des types de données algébriques
        * La validation de l'exécution au moment de la compilation
        * De la métaprogrammation
        * Un transpileur natif et intégré
        * Les monads
    * Il n'a pas
        * De surprise à l'exécution
        * De classes
        * D'environnement d'exécution ou de VM
        * Pas de transformation en bytecode
        * Pas de compilation JIT
        * Pas de garbage collector

> *Deux colonnes, les "il a " et les "il n'a pas"*

* Principaux points techniques (4m)
    * Orientation bas niveau
        * gestion de la mémoire
        * pointeurs, et smart pointers
    * thread safe intégré directement au langage & à la compilation
    * Memory safe
        * typage strict
        * accès mémoires validés par le compilo
        * variables immutables par défaut
        * messages safe entre threads
        * PAS DE NULL
    * inférence de type
    * filtrage par motifs
    * généricité & metaprogramming
> note: peut-être à split en deux slide cette partie

* Langage préféré depuis 7 ans sur SO
*Graphique des langages préférés sur stackoverflow*

## On se lance ?

* Installation (2-3m)
    * One-line install en 1 minute

> *Juste la ligne de commande rustup sh sur le slide -> simplicité*

* L'IDE : VSCode
    * VS est l'IDE le mieux intégré au niveau Rust pour l'instant
        * *Et pourtant grand fan de Jetbrains devant l'éternel*
    * Setup en quelques installations d’extensions​
        * *1 minute, 5 extensions, et go*

* `println!("Hello world ! 🦀");` (2m)
    * *Sreen avec fn main() -> _ {....}*

## Et après ?

* Le déploiement (2m)
    * un seul binaire, même pour un micro service
    * lançable en une seule ligne
    * containeurisable en 10 lignes
    * outils CI/CD

* Le gestionnaire de packets : Cargo
    * Cargo
        * package jamais effacés, invalidé pour nouveaux projets
        * approx 100k packages (npm 2012)

*Equivalent de NPM pour node*

* Des outils supplémentaires
    * CargoTest
        * Lancer des tests unitaires
            * *test intégrables au code, pas besoin de fichiers supplémentaires*
    * Clippy
        * Linter qui force a écrire du rust "correctement"
    * Rustfmt
        * Formatteur de code rust intégrable aux IDE et à la CI
    * CargoDoc
        * Documenteur automatisé de code & librairies

## Rust, on en fait quoi ?

* Rust, on en fait quoi ? (3m)
    * on en fait... (système, CLI, ETL, haute perf, micro-webservices, remplacement vieux code C/C++, web)
    * on ne fait pas (pour l'instant) (IHM/clients lourds*, apps mobiles*)

* Un rapide tour
    * CLI (clip)
    * Web frameworks (Rocket)
        * SQL ? (sqlx, ORM diesel)
    * Web (wasm, yew)
    * Apps mobiles (intégration Flutter pour code métier)
    * Apps hybride (EGUI, TauRi)
    * Jeux (Bevy)

## Niveau performances & fiabilité ? (2m)

* La performance de Rust
    * Au lieu de balancer des benchmarks soulant
    * Prendre 3 exemples concrets

> Les 3 exemples suivants ne seront qu'à l'oral avec un screen en support

* ex discord
> Capture 1 - logo discord

*Pas besoin de présenter ? discord*

> Capture 1 - graph avec Go

*systèmes de serveurs initiaux écrits en Go*  
*Problème de lags fréquents toutes les deux minutes*  
*observation que c'est à cause du GC, même après full optimisation*  

> Capture 2  

*réécriture en Rust, sans opti, plus performant que Go*  
*après finetuning, Rust bat Go dans tout les domaines*  

* ex cloudflare

> Capture 1 - logo cloudflare

*Plus complexe, pas forcément vu celui là : cloudflare*  
*reverse proxy, failovers, etc.*

> Capture 2 - schéma tech nginx

*Cloudfare a réécrit son service de proxy en Rust car Nginx n'était plus assez rapide*  
*On parle de Nginx, écrit en C, proxy/reverse l'un des plus rapide du marché finetuné par cloudflare*  

> Capture 2 - schéma tech pingora

*Pingora, le remplaçant écrit en rust, utilise 70% de CPU en moins, et 67% de RAM en moins*
*Alors qu'il sert 1 billion de rquêtes par jours, c'est 1 avec 12 zéros derrière*

> Capture citation bugs

* ex noyau linux

> screen linus torvalds
*qui est ce mec ?*
*Linus torvalds, écrit le noyau linux en 1986, supervise toujours aujourd'hui*
*Il est souvent connu pour son attention aux détails*
*Ce mec a dit non à l'intégration de C++ dans le noyau car je site :"

> Capture linus c++ is shit

*Dans la version 6.1 de Linux, une partie concernant les drivers sera écrite en Rust*
*Je vous laisse cogiter là dessus*

* D'autres utilisations
    * Deno (compilo)
    * Moteur graphique firefox
    * Moteur de rendu Figma via WASM

* Les pours & contres (3m)
    * pours
        * Mêmes perfomances que le C++
        * Code de production sûr
        * On peut en faire virtuellement n'importe quoi
        * Communauté qui grandi de plus en plus
    * contres
        * Recrutement
        * Courbe d'apprentissage
        * Temps de compilation

* Conclusion
> uniquement à l'oral
* *Dans 10 ans*
    * *Apps mobiles*
    * *Clients lourds*
    * *Apprentissage en cours*



## QRCode vers repo

> Slide avec QRCode vers repo github

*Vu que j'ai envie que vous repartiez avec quelque chose*
*Repo avec ressources d'apprentissage collecté*

## Questions