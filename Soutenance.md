# Soutenance

### Le Client
---
Le client fonctionne correctement et respecte les consignes données.
Utilisation du module `clap` pour permettre de préciser un `--name` et un `--addr` lors du lancement du client, par défaut les valeurs sont :`free_potato` et `localhost:7878`.
Le joueur a les yeux bandés donc après avoir fini son challenge, il envoie la patate à un joueur aléatoirement (qui peut être lui-même car il a fait tomber la patate)

Fait par: Louis XIA

### Le Challenge HashCash
---
Génération des seeders par incrémentation.
Ne doit pas dépasser une complexité de 10, au-delà des `timeout` peuvent survenir.

Fait par: William QUACH et Ilyess NAïT BELKACEM

### Le Challenge MonstrousMaze
---
Renvoie toutes les possibilités tant qu'on a assez d'endurance, et renvoie le chemin le plus court avec le max d'endurance à la fin du labyrinthe.
Fonctionne correctement.

Fait par: Ilyess NAïT BELKACEM et William QUACH

### Le Serveur
---
Le serveur attend d'avoir 2 joueurs inscrit, dès lors 3 rounds d'un challenge est lancé.
Les challenges sont choisis au hasard parmi HashCash et MonstrousMaze.
Le premier challenge est envoyé a un joueur au hasard, puis les suivants sont définis par le joueur venant de résoudre son challenge.

Fait par: William QUACH, Ilyess NAïT BELKACEM et Louis XIA

### Démarche d'élaboration des différents composants du projet
---

Les différents composants du projet ont été élaboration en parallèle.
Nous avons tous commencé à coder en un bloc de code dans le `main`. Lorsque le comportement des composants était ce que nous désirions, nous sommes passer à la refacto du code en créant de fonction. Les fonctions présentes dans plusieurs composants ont été mis dans un `shared` tel que `read_message` et `send_message`.

### Bonus réalisé
---

- MonstrousMaze
- Serveur
- CI 
