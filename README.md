# Rust🦀, le langage des 10 prochaines années
*Edition 2022 - Adrien Gras*

## Concept

Cette présentation se déroule dans le cadre du DevFest organisé par Developers Group Dijon le 2 Décembre 2022.

La présentation a pour but de présenter rapidement le langage Rust, ainsi que ses utilisations possibles, ses pours et ses contres, et un panel d'ouverture sur ce que pourrait devenir le langage dans les 10 prochaines années.

## Au programme

* Présentation du langage
* Installation
* Hello World ! 🦀
* Les outils
* Rust, on en fait quoi ?

Voici le script de la présentation : [Programme](./programme.md)  
Et aussi les slides : [Lien vers la présentation](./slides/Rust.pdf)

## Les ressources

Vous pourrez trouver dans cette section quelques ressources utiles pour commencer Rust ou appronfondir vos connaissances.

### Installer Rust

[Installation Rust sur toutes plateformes](https://doc.rust-lang.org/book/ch01-01-installation.html)

### Configurer son IDE

Après avoir utilisé plusieurs IDE pour créer du code Rust, il s'avère que VSCode est le plus adapté et dispose du meilleur pack d'outils pour écrire efficacement du Rust.

Je vous recommande les extensions VS suivantes pour être efficace :

* [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) - Code completion, imports, goto, références, documentation au survol, etc.
* [Crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates) - Auto-complétion des fichiers Cargo depuis la base de paquets crates.io.
* [Code LLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) - Breakpoints avancés, intégration du débugger au code, etc.
* [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml) - Synthax highlight et auto-complétion des fichiers TOML.
* [Error lens (optionnel)](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens) - Permet d'avoir les erreurs de compilation en bout de ligne de code en place des simples sous-lignages jaunes et rouges.

Vous trouverez l'installation de ces extensions et leurs configurations dans [cette vidéo de Let's Get Rusty - IDE Setup For Rust Development](https://youtu.be/x_iZEK6Rww4)

### Apprendre Rust

Voici un petit parcours pour apprendre à votre rythme à utiliser Rust et créer des applications : 

1. [Le cookbook](https://jimskapt.github.io/rust-book-fr/) - Le cookbook pour commencer à apprendre et utiliser Rust 📙.
2. [Rust by examples](https://doc.rust-lang.org/rust-by-example/) - Des exemples concrêts pour comprendre Rust 🚀.
3. [Rustlings](https://github.com/rust-lang/rustlings) - Des exercices intéractifs pour apprendre Rust ⏩.

### Des vidéos pour comprendre Rust

* [Let's Get Rusty - The rust lang book](https://youtube.com/playlist?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8) - playlist qui suit et explique le Rust cookbook.
* [No Boilerplate - Rust makes you feel like a GENIUS](https://youtu.be/0rJ94rbdteE) - vidéo qui permet de comprendre facilement le concept de "borrow"
* [Code to the moon - Rust Demystified 🪄 Simplifying The Toughest Parts](https://youtu.be/TJTDTyNdJdY) - vidéo d'explication sur les concepts complexes à aborder pour un nouveau développeur
* [Code to the moon - Rust's Alien Data Types 👽 Box, Rc, Arc](https://youtu.be/CTTiaOo4cbY) - explication des types de smart-pointers

### Quelques librairies utiles

* [Clap](https://crates.io/crates/clap) pour créer des CLI facilement.
* [Serde](https://serde.rs/) pour la manipulation de JSON.
* [Rocket](https://rocket.rs/) comme framework web. *(équivalent Symfony/Laravel, Django, etc.)*
* [Actix](https://actix.rs/) pour créer un serveur web rapidement. *(Equivalent expressJS)*
* [Disel](https://diesel.rs/) comme ORM SQL. *(Equivalent Doctrine, Sequelize, etc.)*
* [sqlx](https://github.com/launchbadge/sqlx) pour intéragir plus simplement avec une base de données.
* [yew](https://yew.rs/) pour un frontend moderne en Rust *(Equivalent React)*
* [Rust + WASM](https://rustwasm.github.io/docs/book/) ou WASM pour intégrer simplement Rust et Javascript.
* [TauRi](https://tauri.app/) pour construire des apps hybrides *(Equivalent Electron)*
* [Bevy](https://bevyengine.org/) comme moteur de jeu-vidéo.

### Des conteneurs docker prêts à l'emploi

Vous pourrez trouver mes conteneurs docker prêt à l'emploi dans le dossier [samples](./samples/README.md).

### Quelques autres médias

* [Fireship - Rust in 100 seconds](https://youtu.be/5C_HPTJg5ek)
* [NoBoilerplate - Rust talks](https://youtube.com/playlist?list=PLZaoyhMXgBzoM9bfb5pyUOT3zjnaDdSEP)
* [Fireship - Tauri in 100 seconds](https://youtu.be/-X8evddpu7M)
* [Fireship - WASM in 100 seconds](https://youtu.be/cbB3QEwWMlA)
* [kostya - benchmarks](https://github.com/kostya/benchmarks#measurements)
* [Vercel - benchmarks](https://programming-language-benchmarks.vercel.app/go-vs-rust)

## Licence

Les sources de ce repository sont sous [licence MIT](./LICENSE.md).

La présentation et son contenu peuvent être utilisés de manière libre, à la seule condition de citer la source et l'auteur original.