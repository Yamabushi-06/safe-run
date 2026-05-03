# 🛡️ SAFE-RUN

![Rust](https://img.shields.io/badge/rust-v1.70+-orange?style=for-the-badge&logo=rust)
![Linux](https://img.shields.io/badge/Linux-5.13%2B-blue?style=for-the-badge&logo=linux)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)

**SAFE-RUN** est un exécuteur de commandes ultra-sécurisé (Sandbox) pour Linux, écrit en Rust. Il combine la puissance de **Landlock** et **Seccomp** pour emprisonner n'importe quel processus et empêcher les fuites de données ou les actions malveillantes.

> **💡 Pourquoi safe-run ?** Même un simple script peut cacher des commandes malveillantes. `safe-run` garantit que le processus ne touchera pas à vos fichiers sensibles et ne fera aucun appel système inattendu.

---

## ✨ Fonctionnalités clés

*   🔒 **Isolation Système de Fichiers (Landlock)** : Le processus lancé n'a aucun droit de lecture/écriture en dehors de ce qui est strictement nécessaire.
*   🛑 **Filtrage des Appels Système (Seccomp-BPF)** : Abat instantanément le processus (`KillProcess`) s'il tente d'utiliser le réseau, de modifier la mémoire système ou d'élever ses privilèges.
*   ⚡ **Performance Native** : Écrit en Rust, l'impact sur le temps d'exécution est quasi-nul.
*   🎛️ **Mode Souple optionnel** : Possibilité de relâcher Seccomp en cas de besoin tout en gardant Landlock actif.

---

## 🚀 Installation

Prérequis : Un système Linux récent (Noyau 5.13 ou supérieur) et `cargo` installé.
```bash
git clone [https://github.com/Yamabushi-06/safe-run.git](https://github.com/Yamabushi-06/safe-run.git)
cd safe-run
cargo build --release
