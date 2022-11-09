# Programme de la présentation

Consignes de lecture du plan
* Les parties en italiques sont des didascalies, énoncés à l'oral
* Le reste du texte compose les slides
* Les titres sont les parties du powerpoint (splitter full page avec juste le titre)
    * Les titres en italiques sont là en guise de repère, ils n'apparaitront pas en tant que page

> **Durée: 35-40min**

### *Début*

* Intro + Présentation du sommaire (1m)

* A qui se destine la présentation ? (1m)
    * tech
        * *se lancer dans le langage*
        * *connaitre de nouvelles manières de faire*
    * CTO
        * *connaitre de nouvelles alternatives pour produire du soft adapté*
    * les autres
        * *la curiosité*
    * *y'en aura pour tout le monde dans la prez de toute façon*

### Mais... C'est qui ?

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

### Rust kezako ?

* Présentation du langage (2m)
    * Présentation
        * Langage bas niveau, remplacement C++
        * Deux mots : performances & sécurité
    * Utilisé principalement pour
        * CLI
        * WebAssembly
        * services réseau
        * applications embarqués
    * Créé en 2006 par Graydon Hoare, puis repris en 2010 par la fondation Mozilla
    * Version actuelle : 1.65.0 (2 Novembre)

* Principaux points techniques (4m)
    * Orientation bas niveau
        * gestion de la mémoire manuelle
            * *Pas de GC*
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
    * généricité
    * 1 exécutable avec tout dedans
> note: peut-être à split en deux slide cette partie

### On se lance ?

* Installation (2-3m)
    * One-line install en 1 minute
        * *Juste la ligne de commande rustup sh sur le slide* -> simplicité

* Configuration VS en 2 minutes
    * VS est l'IDE le mieux intégré au niveau Rust pour l'instant
        * *Et pourtant grand fan de Jetbrains devant l'éternel*
    * Setup en quelques installations d’extensions​
        * Rust Analyzer - Code completion, imports, goto, références, documentation au survol, etc​
        * Crates - Auto-complétion des fichiers Cargo depuis la base de paquets crates.io.​
        * Code LLDB - Breakpoints avancés, intégration du débugger au code, etc.​
        * Better TOML - Synthax highlight et auto-complétion des fichiers TOML.​
        * Error lens - Permet d'avoir les erreurs de compilation en bout de ligne de code en place des simples sous-lignages jaunes et rouges.

* `println!("Hello world ! 🦀");` (2m)
    * *Sreen avec fn main() -> _ {....}*

### Démonstration de concepts (5min)

> note : exemples de wikipedia, mais assez parlants

* Valeur et mutabilité
    * contrairement à beaucoup de langages, variables immutables par défaut (pas modifiable)
    * besoin de mettre `mut` pour rendre la variable mutable
    * les constantes sont avec `const`

```rust
fn main() {
    // Déclaration de variables
    let mut a: usize = 5; // a est une variable modifiable
    let b: usize = a * 2; // b est non modifiable et du même type que a

    // Constantes
    const c: u32 = 5; // déclaration d'une constante entière non-signée
    const c: u8 = b - 3; // interdit car `b - 3` n'est pas une expression constante (b non défini à la compilation)
    const c = 5; // interdit car le type de c n'est pas précisé

    // Altération
    c = 3; // illégal car c est une constante
    b = 3; // illégal car b est une variable immuable
    a = 2; // autorisé car a est déclaré comme "mut"
    let a = a + 5; // autorisé une nouvelle variable a est créée valant 7,
                   // l'ancienne variable a est "couverte" par la nouvelle (shadowing)

    // Vérification des valeurs
    assert_eq!(a, 5); // faux
    assert_eq!(b, 10); // vrai
}
```

* Énumérations et filtrage par motif
    * Langage beaucoup basés sur les Enums.
    * filtrage par motif avec `match`

```rust
// On crée un type « Forme » pour décrire des formes géométriques.
enum Forme {
    Point,               // une forme peut être un point, sans données attachées.
    Rectangle(f64, f64), // une forme peut être un rectangle, caractérisé par les longueurs de ses côtés.
    Cercle(f64),         // une forme peut être un cercle, caractérisé par son rayon.
}

// Calcule l'aire d'une forme géométrique.
fn aire(f: Forme) -> f64 {
    match f {
        // Filtrage par motif avec « match »
        Forme::Point => 0.0,
        Forme::Cercle(rayon) => 3.14 * rayon * rayon,
        Forme::Rectangle(cote_a, cote_b) => cote_a * cote_b,
    }
}
```

* Comment on fait sans null ?
    * null remplacé par une Enum `Option<T>`
    * évaluation comme une énum

```rust
// récupération de la version d'un logiciel sur une API
fn getVersionFromAPI(software_name: String) -> Option<String> {
    let mut version: String = String::from("");

    // ...simulation d'une requête sur une API
    // modification de name si la version est trouvée

    // si il n'y a pas de version du logiciel sur l'API, on retoure None
    if "" == version {
        return None;
    }

    // si par contre la version a été retrouvé, on la retourne
    Some(version)
}

fn main() {
    // appel à l'API pour voir la dernière version de Rust
    let result: Option<String> = getVersionFromAPI("rust".into());

    // filtrage par motif si la valeur a été trouvé ou non
    match result {
        // une valeur a été trouvé, elle est donc dans Some
        Some(version_name) => println!("Version trouvé ! : {}", version_name),
        // aucune valeur n'a été trouvée, c'est donc None qui a été retrouvé
        None => println!("Version pas trouvé sur l'API")
    }

    // version alternative
    if let Some(version_name) = result {
        println!("Version trouvé ! : {}", version_name),
    } else {
        println!("Version pas trouvé sur l'API")
    }
}
```

* Programmation générique
    * Métaprogrammation possible avec des traits

```rust
// fonction de tri à bulle pour toute struct supportant le trait PartialOrd
fn tri_bulle<T>(liste: &mut Vec<T>)
    where T: std::cmp::PartialOrd
{
    // Ici, on a besoin de l’opérateur de comparaison < implémenté par le trait
    // PartialOrd
    let mut i: usize = 0;

    while i < (liste.len() - 1) {
        if liste[i] < liste[i + 1] {
            liste.swap(i, i + 1);
            i = 0;
        } else {
            i += 1;
        }
    }
}
```
> note : pas sur de le mettre celui là, je pense que déjà à la prononciation de metaprogrammation y'aura des anévrismes dans le public, c'est une notion complexe

* Définition des méthodes communes par le biais de trait
    * Traits semblable à des interfaces/classes abstraites dans d'autre langages
    * Ils définissent les méthodes qui seront proposées par les structures les implémentant
    * Ces derniers peuvent également proposer une implémentation générique de certaines de leur fonctions à condition que ces dernières ne requièrent pas l’utilisation de données stockées dans les objets.

```rust
// Représentation d'un point géométrique 2D
struct Point {
    x: f32,
    y: f32
}

trait Polygone {
    // Tout polygone peut calculer un nombre de côtés
    fn nombre_cotes(&self) -> usize;

    // Tout polygone peut renvoyer une liste de ses points d'ancrage
    fn points(&self) -> Vec<Point>;

    // Il n’existe pas de méthode simple et générique pour calculer l’aire d’un
    // polygone, donc on la laisse sans définition.
    // Elle devra être défini par chacune des structure qui implémenteront le trait.
    fn aire(&self) -> f32;

    // Par contre, on peut calculer de manière universelle le périmètre d'un polygone
    // On propose donc une implémentation par défaut directement dans le trait.
    // Cette implémentation pourra être redéfini dans les structures implémentant le trait
    fn perimetre(&self) -> f32 {
        let mut ret: f32 = 0.0;
        let points: Vec<Point> = self.points();

        for i in 1..points.len() {
            let dimension: f32 = 
                f32::pow(points[i].x - points[i - 1].x, 2) + 
                f32::pow(points[i].y - points[i - 1].y, 2);

            ret += f32::sqrt(dimension);
        }

        return ret;
    }
}
```

* Possession et emprunt
    * Concept permettant la sécurité de la mémoire dans Rust
    * Ainsi, une valeur a toujours un seul propriétaire. Si la valeur change de propriétaire, l'ancien propriétaire ne peut plus l'utiliser.

    * Règles simples
        * autant d'emprunt non mutable que l'on veut
        * qu'un seul emprunt mutable possible
        * une fois que la possession a été transmise, la variable perd l'utilisation de sa valeur


> Exemple 1 : ownership & borrow
```rust
// La représentation d'un livre
struct Book {}

// l'image de la librairie
fn main() {
    let neuromancer = Book {};

    loan_by_maxime(&neuromancer);
    loan_by_lucy(&neuromancer);
    // les deux sont des emprunts, pas de soucis pour l'instant

    withdraw_book(neuromancer);
    // le livre est retiré des rayons

    loan_by_maelle(&neuromancer);
    // ERREUR - le livre a été "retiré" (emprunté) par la fonction précédente
}
```

> Exemple 2 : ownership & mutable borrow
```rust
// La représentation d'un manuscrit
struct Manuscipt {}
// Représentation d'un éditeur de livre
struct Editor {}

// l'image de la boite d'édition
fn main() {
    let neuromancer = Manuscript {};

    let ace_books = Editor {};
    let dave = Editor {};

    edit(&mut neuromancer, ace_books);
    // emprunt mutable pour modificaton du livre

    edit(&mut neuromancer, dave);
    // ERREUR -> il ne peut y avoir qu'un seul emprunt pour modification à la fois

    // Même avec le premier emprunt, nous sommes toujours propriétaire de neuromancer
    // Il a juste été emprunté pour modification

    sell(neuromancer);
    // emprunt "définitif", la possession du livre a été confié à la fonciton sell()

    load(&neuromancer);
    // ERREUR -> on ne peut plus louer neuromancer car il est possédé maintenant par sell()
}
```

* Le déploiement (2m)
    * un seul binaire, même pour un micro service
    * lançable en une seule ligne
    * containeurisable en 10 lignes
    * outils CI/CD

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