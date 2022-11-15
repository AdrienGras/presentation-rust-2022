# Programme de la présentation

* Chaque slide est séparé par une séparation horizontale `<hr />`.


<hr />

### Rust, le langage pour les 10 ans à venir

> **Commentaires :**  
> Salutations, remerciements, ...
>
> A qui se destine la présentation :
> Techs - découverte d'un nouveau langage, appréension des gains de Rust par rapport à leurs technos actuelles  
> CTO - avoir en tête de nouvelles alternatives pour les futurs développements des équipes  
> Tout le monde - découvrir de nouvelles technos

<hr />

### Salut !

* Architecte solution chez Owlnext​

* Auparavant Head of software & sytem architecture chez iBanFirst​

* Quelques réalisations​
    * Conception d’un système de chiffrage end-to-end : Heimdall​
    * Création d’un système de téléphonie VoIP via WebRTC avec IVRs, Files d’attentes, redirections d’appels, connexions CRM, etc.​
    * Refactoring d’un serveur de connexion aux salles de marchés internationale

> **Commentaires :**  
> Présentation personnelle, rapide.

<hr />

### Rust, c’est quoi ?​

<hr />

### Rust

* Remplaçant du C/C++​
    * Performances des langages bas-niveau​
    * Concepts des langages haut-niveau​
    * Sécurité​

* Utilisé principalement pour​
    * Des applications systèmes, des backends, des CLI​
    * Du web avec WASM​
    * Des services réseaux​
    * De l'informatique embarqué​

* Créé en 2006 par Graydon Hoare, puis repris à partir de 2010 par la fondation Mozilla​

* Version actuelle : 1.65.0 (2 Novembre 2022)​

* Sa mascotte : Ferris

<hr />

### Pourquoi choisir Rust ?​

Il a
* Un typage de données algébrique​
* La validation de l'exécution au moment de la compilation​
* De la métaprogrammation​
* Un transpileur natif et intégré​
* Une intégration des MONADS​
* Une compatibilité native avec C

Il n'a pas
* De surprises à l’exécution​
* De classes​
* D’environnement d’exécution ou de VM​
* De transformation type « Bytecode »​
* De compilation JIT​
* De garbage collector​
* De valeurs « NULL »

> **Commentaires :**  
> * Un typage de données algébrique​ : Jointures de types, comme les promises en JS  
> * La validation de l'exécution au moment de la compilation​ : tout est validé à la compilation, aucune erreur d'adressage ou de mémoire à l'exécution
> * De la métaprogrammation​ : vous pouvez programmer Rust avec du Rust via des macros, etc.
> * Un transpileur natif et intégré​ : vous pouvez interpréter du Rust en Rust, créer un walker, un lexer, etc.
> * Une compatibilité native avec C : peut exécuter du code C externe, et produire des libs utilisable par C

<hr />

### Les principales caractéristiques de Rust​

* Orientation bas-niveau​
    * Gestion de la mémoire via le mécanisme d’emprunt​
    * Gestion des pointeurs et smart-pointers​

* Thread sécurisés intégrés directement au langage​
    * Validation des mémoires partagés à la compilation​

* Sécurisation & compilation​
    * Typage strict​
    * Accès mémoires et emprunts validés à la compilation​
    * Variables immutables par défaut​
    * Pas de NULL​
    * Inférence de type​

* Filtrage par motifs et Foncteurs​
    * Option<T>, Result<T, E>, Futur<T>, etc.​

​
* Généricité et métaprogrammation​

<hr />

### We ❤️ Rust ​

> **Commentaires :**  
> Langage préféré des développeurs sur stackoverflow depuis 7 ans, c-à-d depuis sa version 1.0.

<hr />

### On se lance ?

<hr />

### Installation

> **Commentaires :**  
> * Installation en one-liner ou un exécutable pour tout les OS principaux
> * Une fois installé, vous avez tout les outils pour commencer à développer en Rust

<hr />

### L’IDE​

* Visual Studio Code est l’IDE de choix pour Rust​
    * Gratuit​
    * Possède toute la suite d’outil pour travailler avec Rust​
    * Configuration en 2 minutes

> **Commentaires :**  
> * Pourtant, paye 200€/ans pour des outils Jetbrains tellement je les aiment

<hr />

### Hello world ! ​

> **Commentaires :**  
> * fonction définie par `fn`, main classique
> * utilisation d'une macro pour print (définition de rust écrit en rust)
> * Emoji dans la chaine, car Rust garanti toutes les chaines UTF8 valides

<hr />

### Et après ?​

<hr />

### On déploie comment ?​
* Un seul binaire de généré pour toute votre application​

* Lançable en une ligne de commande, conteneurisable en 10​

* Tout les outils dont vous avez besoin pour vos CI/CD​
    * Tests unitaires​
    * Linting & formatting​
    * Loggers & outils de monitoring​

* Intégration direct avec les principaux outils de CI/CD

<hr />

### Cargo : le packager​

* Gestionnaire de paquet pour les projets Rust​
    * Utilise les paquets de crates.io​
    * Paquets = crates​
    * Actuellement +100k crates géré sur crates.io​

* Pas qu’un gestionnaire de paquet :​
    * Installe les outils additionnels (linters, etc.)​
    * Lance les builds & compilations​
    * Génère la documentation​
    * Etc.​

> **Commentaires :**  
> * Equivalent NPM pour node

<hr />

### Quelques autres outils​
* RUSTFMT​
    * Le code-formatter​

* Clippy​
    * Le linter​

* CargoTest​
    * Pour les tests unitaires​

* CargoDoc​
    * Pour la génération de documentation​

​* Tout ces outils sont intégrables automatiquement à VSCode

<hr />

### Le playground

> **Commentaires :**  
> * Utile pour découvrir Rust et ses mécanismes.
> * Peut utiliser tout les outils additionnels (fmt, clippy, etc.)

<hr />

### Rust, on en fait quoi ?

<hr />

### Avec Rust, on peut « quasi » tout faire​

* On peut développer​
    * Des applications CLI/Serveur​
    * Des ETL​
    * Des applications haute-performances​
    * Des applications web​
    * Des micro-services​
    * Du remplacement de code legacy C/C++​
    * Des clients légers & hybrides​

* On ne peut pas – pour l’instant – développer​
    * Des clients lourds​
    * Des applications mobiles

<hr />

### Un rapide tour : les CLI avec CLAP​

> **Commentaires :**  
> * 5 min pour comprendre la lib, et vous avez des CLI avec des beaux paramètres automatiquement parsés

<hr />

### Un rapide tour : Rocket, le framework web​

> **Commentaires :**  
> * Equivalent Symfony, Django, ROR, Spring, etc.

<hr />

### Un rapide tour : …avec un peu de SQL ? L’ORM Diesel​

> **Commentaires :**  
> * QueryBuilder, validations des requêtes à la compilation, interconnexion avec le modèle, migrations, etc.

<hr />

### Un rapide tour : Le frontend avec WASM​

> **Commentaires :**  
> * Permet de faire du développement web... en Rust

<hr />

### Un rapide tour : …ou remplacer React par Yew​

> **Commentaires :**  
> * La plupart des features de React (routeur, states, hooks, etc.)
> * 3x plus rapide que React

<hr />

### Un rapide tour : …ou écrire le code lourd de votre app mobile en Rust ? ​

> **Commentaires :**  
> * Plus rapide que Dart pour les calculs lourds type 3D
> * On peut faire la même avec Kotlin et Swift

<hr />

### Un rapide tour : et pourquoi pas développer des jeux-vidéo ?​

<hr />

### Et la performance dans tout ça ?

<hr />

### Discord : de Go à Rust​

> **Commentaires :**  
> * Tout le monde connait discord
> * millions de messages/conversations vocales et vidéos par jour

<hr />

### Discord : de Go à Rust​

> **Commentaires :**  
> * Graphique : perf d'un serveur discord écrit en Go
> * pointes toutes les 2 minutes = lagspike + conso élevé
> * Après recherche, observation faute du GC
> * Besoin de réécrire les serveurs avec un langage sans GC

<hr />

### Discord : de Go à Rust​

> **Commentaires :**  
> * Réécriture des serveurs en Rust
> * Sans optimisation, Rust plus perfomant que Go
> * Après finetuning, Rust bats go dans toutes les catégories

<hr />

### Cloudflare : de Nginx à Pingora (Rust)​

> **Commentaires :**  
> * Moins fréquent mais tout aussi connu : cloudflare
> * Services de proxy, sécurisation, load-balancing, etc.

<hr />

### Cloudflare : de Nginx à Pingora (Rust)​

> **Commentaires :**  
> * Cloudfare a réécrit son service de proxy en Rust car Nginx n'était plus assez rapide
> * On parle de Nginx, écrit en C, proxy/reverse l'un des plus rapide du marché finetuné par cloudflare

<hr />

### Cloudflare : de Nginx à Pingora (Rust)​

> **Commentaires :**  
> * Pingora, le remplaçant écrit en rust, utilise 70% de CPU en moins, et 67% de RAM en moins
> * Alors qu'il sert 1 billion de rquêtes par jours, c'est 1 avec 12 zéros derrière

<hr />

### Cloudflare : de Nginx à Pingora (Rust)​

> **Commentaires :**  
> * Phrase du responsable technique de Pingora chez Cloudflare

<hr />

### Noyau linux : du Rust dans la ver6.1​

> **Commentaires :**  
> * qui est ce mec ?
> * Linus torvalds, écrit le noyau linux en 1986, supervise toujours aujourd'hui
> * Il est souvent connu pour son attention aux détails

<hr />

### Noyau linux : du Rust dans la ver6.1​

> **Commentaires :**  
> * Ce mec a dit non à l'intégration de C++ dans le noyau car je site :
> * Dans la version 6.1 de Linux, une partie concernant les drivers sera écrite en Rust
>
> D'autres exeples :
> * Le compilateur de Deno, remplaçant de NodeJS est écrit en Rust​
> * Magic Pocket, le système de stockage distribué de Dropbox est écrit en Rust

<hr />

### En résumé​

<hr />

### Bon alors, Rust ??​

Les +
* Mêmes performances que le C​
* Un code sûr en production​
* On peut en faire -presque- ce que l’on veut​
* Une communauté grandissante de jours en jours​
* Le langage est en cours d’adoption par de grandes entreprises

Les -
* Le recrutement​
* La courbe d’apprentissage​
* Le temps de compilation

> **Commentaires :**  
> * Anecdote sur communauté
>
> * Recrutement : pas de développeurs formé à ce langage nativement
> * courbe d'apprentissage : peut être difficile si pas les bonnes ressources
> * temps de compilation : peut prendre du temps sur certains projets, notamment avec de la BD dedans

<hr />

### Conclusion​

> **Commentaires :**  
> * Aujourd'hui besoin de logiciel performants
> * Problème c'est que les abstractions des langages rendent de plus en plus énergivore et consommateurs de performances les programmes écrits
> * De moins en moins d'accès au bas niveau, et donc on doit palier à ça en réécrivant souvent des couches ou paquets par dessus des Frameworks
>
> * Rust prend le pari inverse, développer un langage avec des perfs bas niveau, avec tout les concepts de développement haut-niveau (traits, monads, généricité) tout en étant sécurisé et efficace.
>
> * Si le C a été le langage des 40 dernières années, et que Rust continue son assention, il pourrait bien être le futur langage des 40 prochaines années. 

<hr />

### 🎁- Le repo Git 

* Le contenu de la présentation​

* Comment bien démarrer Rust​

* Des ressources complémentaires​
    * Extensions VSCode​
    * Playlists et vidéos d’explications​
    * Conteneurs docker prêt à l’emploi​

* …Et bien plus encore

<hr />

### Questions