# SAFE-RUN: Sandboxing Linux avec Rust

Exécuteur de commandes sécurisé basé sur Landlock. Ce programme restreint l'accès au système de fichiers pour les processus lancés.

## Installation
Prérequis : Rust et noyau Linux 5.13+.
```bash
git clone [https://github.com/Yamabushi-06/safe-run.git](https://github.com/Yamabushi-06/safe-run.git)
cd safe-run
cargo build --release
