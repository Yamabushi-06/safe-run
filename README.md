# SAFE-RUN : Linux Sandboxing CLI

safe-run est un exécuteur de commandes en ligne (CLI) conçu pour les environnements Linux. Développé en Rust, il exploite nativement les API de sécurité du noyau (Landlock et Seccomp-BPF) pour isoler les processus non fiables et protéger le système hôte contre les exécutions malveillantes.

## Installation

Nécessite un noyau Linux 5.13+ et Cargo.
```bash
git clone [https://github.com/Yamabushi-06/safe-run.git](https://github.com/Yamabushi-06/safe-run.git)
cd safe-run
cargo build --release
## Utilisation
```
# Utilisation 
L'outil propose deux niveaux d'isolation selon vos contraintes de sécurité :
```bash
# 1. Mode Strict (Bunker par défaut)
# Active Landlock (fichiers) et Seccomp (processeur).
safe-run cat /etc/shadow

# 2. Mode Souple (Isolation Fichiers Uniquement)
# Désactive Seccomp pour autoriser le réseau, Landlock reste actif.
safe-run --allow-sys curl [https://api.github.com](https://api.github.com)

# 3. Afficher les options
safe-run --help
```
