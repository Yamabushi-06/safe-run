use landlock::{Access, AccessFs, Ruleset, RulesetAttr, ABI};
use std::env;
use std::os::unix::process::CommandExt;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- ÉTAPE 1 : RÉCUPÉRATION ---
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: safe-run <commande> [args...]");
        std::process::exit(1);
    }
    let target_cmd = &args[1];
    let target_args = &args[2..];

    // --- ÉTAPE 2 : DÉFINITION ---
    let abi = ABI::V1; 
    let ruleset = Ruleset::default()
        .handle_access(AccessFs::from_all(abi))?
        .create()?;

    // --- ÉTAPE 3 : VERROUILLAGE ---
    ruleset.restrict_self()?;
    println!("🔒 SAFE-RUN : Environnement verrouillé. Exécution de {}...", target_cmd);

    // --- ÉTAPE 4 : EXÉCUTION ---
    let err = Command::new(target_cmd)
        .args(target_args)
        .exec(); 

    eprintln!("❌ Erreur d'exécution : {}", err);
    std::process::exit(1);
}

