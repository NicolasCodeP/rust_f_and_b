# Gestion des Coûts F&B

Une application de gestion des coûts pour la restauration, développée en Rust avec egui.

## 🌟 Fonctionnalités

- ✅ **Gestion d'ingrédients** - Ajout, modification et suppression d'ingrédients avec prix et unités
- ✅ **Calcul de coûts automatique** - Conversions d'unités automatiques (kg ↔ g, etc.)
- ✅ **Gestion de recettes** - Création et modification de plats avec composants
- ✅ **Calcul de marges** - Calcul automatique des coûts, marges brutes et taux de marge
- ✅ **Interface en français** - Interface utilisateur entièrement traduite
- ✅ **Support web et desktop** - Fonctionne dans le navigateur et en application native

## 🚀 Demo en ligne

Essayez l'application directement dans votre navigateur : [https://nicolascodep.github.io/rust_todo_list_gui/](https://nicolascodep.github.io/rust_todo_list_gui/)

## 🛠️ Développement

### Prérequis

- Rust (édition 2024)
- Cargo
- Pour la version web : `trunk` et la cible `wasm32-unknown-unknown`

### Installation locale

```bash
# Cloner le dépôt
git clone https://github.com/NicolasCodeP/rust_todo_list_gui.git
cd rust_todo_list_gui

# Lancer la version desktop
cargo run --release

# Pour la version web
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve --release
```

## 📊 Technologies

- **Rust** - Langage de programmation
- **egui** - Framework d'interface utilisateur
- **eframe** - Framework d'application pour egui
- **poc_fnb** - Bibliothèque de gestion F&B personnalisée
- **Trunk** - Outil de build pour les applications web Rust/WASM

## 🏗️ Architecture

Le projet utilise une architecture modulaire :

- `poc_fnb/` - Bibliothèque métier pour la gestion F&B
- `src/app.rs` - Interface utilisateur principale
- `src/main.rs` - Point d'entrée de l'application
- `src/android_keyboard.rs` - Support clavier virtuel Android

## 📱 Support Android

L'application inclut un support spécialisé pour le clavier virtuel Android :

- **Affichage automatique** du clavier lors du focus sur les champs de texte
- **Masquage automatique** du clavier lorsque les champs perdent le focus
- **Gestion des exceptions JNI** pour une expérience stable
- **Compatibilité NativeActivity** pour les applications Android natives

### Configuration Android

Pour utiliser le support clavier Android, assurez-vous que :

1. Les dépendances JNI sont configurées (`jni`, `ndk-sys`, `ndk`)
2. L'application est compilée avec la cible `aarch64-linux-android` ou similaire
3. Le contexte Android est correctement initialisé au démarrage

## 📝 License

Ce projet est sous licence MIT/Apache 2.0.
