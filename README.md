cat <<EOF > README.md
# SAFE-RUN: Sandboxing Linux avec Rust

Exécuteur de commandes sécurisé basé sur Landlock. Ce programme restreint l'accès au système de fichiers pour les processus lancés.

## Installation
\`\`\`bash
git clone https://github.com/Yamabushi-06/safe-run.git
cd safe-run
cargo build --release
\`\`\`

## Utilisation
\`\`\`bash
./target/release/safe-run "ls /home"
\`\`\`
EOF

git add README.md
git commit -m "Update documentation"
git push

echo "DONNEE_CONFIDENTIELLE_TEST" > secret.txt

echo "Test 1 (Standard) :"
cat secret.txt

echo -e "\nTest 2 (Safe-run) :"
cargo run -- cat secret.txt
