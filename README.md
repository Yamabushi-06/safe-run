# Mise à jour du fichier README.md
cat <<EOF > README.md
# SAFE-RUN: Sandboxing Linux avec Rust

Exécuteur de commandes sécurisé basé sur Landlock. Ce programme restreint l'accès au système de fichiers pour les processus lancés.

## Installation
Prérequis : Rust et noyau Linux 5.13+.
\`\`\`bash
git clone https://github.com/Yamabushi-06/safe-run.git
cd safe-run
cargo build --release
\`\`\`

## Utilisation
Lancement d'une commande sans accès au système de fichiers :
\`\`\`bash
./target/release/safe-run "ls /home"
\`\`\`
EOF

# Synchronisation avec le dépôt distant
git add README.md
git commit -m "Update documentation"
git push

# Procédure de test de sécurité
echo "Initialisation du test..."
echo "DONNEE_CONFIDENTIELLE_TEST" > secret.txt

echo "Test 1 : Accès standard (lecture autorisée) :"
cat secret.txt

echo -e "\nTest 2 : Accès via safe-run (lecture restreinte) :"
cargo run -- cat secret.txt
