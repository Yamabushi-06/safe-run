SAFE-RUN: Zero-Trust Executor for Linux

Sandboxing natif via Landlock. Bloque l'accès au système de fichiers par défaut.
Pourquoi ?

Pour lancer des scripts inconnus (curl | sh) sans risquer ses données. Même en sudo, le processus est enfermé.
Utilisation

cargo run --
Test de sécurité

$ cargo run -- touch test.txt
❌ Permission denied (os error 13)
