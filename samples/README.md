# Samples

## Conteneurs docker

Vous pouvez trouver ici deux conteneurs docker prêt à être utilisés :

* [docker-dev](./docker-dev/) est un conteneur utilisable en développement.
* [docker-prod](./docker-prod/) est un conteneur à deux stages pour déployer une application en production.

### Docker-dev

Ce conteneur lance juste un environement Rust prêt pour le développement. Copiez simplement les fichiers `docker-compose.yml` et `dockerfile` à la racine de votre projet Rust.

Pour le lancer :
```bash
docker-compose up --build -d
```

Puis pour s'y connecter :
```bash
docker-compose exec rust bash
```

Vous pourrez en suite lancer toutes vos commandes `cargo`.

> **Note :** N'hésitez pas à modifier le fichier `dockerfile` pour ajouter vos propres outils au build de l'image.

### Docker-prod

Ce conteneur va se construire en 2 étapes :
- Un builder qui va build l'exécutable de votre application
- Un runner qui sera plus léger et devra simplement exécuter votre application. C'est cette image que vous pouvez exporter sur une registry par exemple.

Comme le conteneur précédent, vous n'avez qu'à copier les fichiers `docker-compose.yml` et `dockerfile` à la racine de votre projet Rust.

Pour build l'image :
```bash
docker-compose build
```

Pour lancer votre app en mode "production" :
```bash
docker-compose up --build -d
```