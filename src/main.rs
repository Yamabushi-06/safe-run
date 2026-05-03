use clap::Parser;
use landlock::{Access, AccessFs, Ruleset, RulesetAttr, ABI};
use std::process::Command;
use std::os::unix::process::CommandExt;
use seccompiler::{BpfProgram, SeccompAction, SeccompFilter, TargetArch};

#[derive(Parser, Debug)]
#[command(name = "safe-run")]
#[command(version = "1.0")]
#[command(about = "Exécuteur de commandes sécurisé (Landlock + Seccomp)", long_about = None)]
struct Cli {
    /// Désactive le verrouillage Seccomp (autorise les appels système complexes)
    #[arg(long)]
    allow_sys: bool,

    /// La commande à exécuter (ex: cat, ls, bash)
    cmd: String,

    /// Les arguments de la commande
    #[arg(trailing_var_arg = true)]
    cmd_args: Vec<String>,
}

fn main() {
    // 1. Analyse des arguments tapés par l'utilisateur
    let cli = Cli::parse();

    // 2. Activation de LANDLOCK (Verrouillage des fichiers - TOUJOURS ACTIF)
    let abi = ABI::V1;
    let ruleset = Ruleset::default()
        .handle_access(AccessFs::from_all(abi))
        .expect("Erreur Landlock");

    let status = ruleset.create().expect("Erreur creation ruleset");
    status.restrict_self().expect("Echec Landlock");

    // 3. Activation de SECCOMP (Verrouillage Système - OPTIONNEL)
    if !cli.allow_sys {
        let filter = SeccompFilter::new(
            vec![
                (libc::SYS_read, vec![]), (libc::SYS_write, vec![]),
                (libc::SYS_exit, vec![]), (libc::SYS_exit_group, vec![]),
                (libc::SYS_execve, vec![]), (libc::SYS_brk, vec![]),
                (libc::SYS_mmap, vec![]), (libc::SYS_munmap, vec![]),
                (libc::SYS_fstat, vec![]), (libc::SYS_rt_sigaction, vec![]),
                (libc::SYS_openat, vec![]), (libc::SYS_close, vec![]),
                (libc::SYS_mprotect, vec![]), (libc::SYS_arch_prctl, vec![]),
                (libc::SYS_set_tid_address, vec![]), (libc::SYS_set_robust_list, vec![]),
                (libc::SYS_rseq, vec![]), (libc::SYS_prlimit64, vec![]),
                (libc::SYS_newfstatat, vec![]), (libc::SYS_access, vec![]),
                (libc::SYS_pread64, vec![]), (libc::SYS_getrandom, vec![]),
            ].into_iter().collect(),
            SeccompAction::KillProcess,
            SeccompAction::Trap,
            TargetArch::x86_64,
        ).unwrap();

        let bpf_prog: BpfProgram = filter.try_into().unwrap();
        seccompiler::apply_filter(&bpf_prog).expect("Echec Seccomp");
        println!("🛡️ SAFE-RUN : Mode BUNKERS (Landlock + Seccomp) activé.");
    } else {
        println!("⚠️ SAFE-RUN : Mode SOUPLE (Landlock uniquement) activé.");
    }

    // 4. Exécution de la commande
    let err = Command::new(&cli.cmd)
        .args(&cli.cmd_args)
        .exec();

    eprintln!("❌ Erreur : {}", err);
}
