use landlock::{Access, AccessFs, Ruleset, RulesetAttr, ABI};
use std::env;
use std::process::Command;
use std::os::unix::process::CommandExt;
use seccompiler::{BpfProgram, SeccompAction, SeccompFilter, TargetArch};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: safe-run <commande> [args]");
        return;
    }

    let abi = ABI::V1;
    // Correction 1 : Retrait du "mut" inutile
    let ruleset = Ruleset::default()
        .handle_access(AccessFs::from_all(abi))
        .expect("Erreur Landlock");

    let status = ruleset.create().expect("Erreur creation ruleset");
    status.restrict_self().expect("Echec Landlock");

    // Correction 2 : Différenciation des punitions (KillProcess vs Trap)
    let filter = SeccompFilter::new(
        vec![
            (libc::SYS_read, vec![]),
            (libc::SYS_write, vec![]),
            (libc::SYS_exit, vec![]),
            (libc::SYS_exit_group, vec![]),
            (libc::SYS_execve, vec![]),
            (libc::SYS_brk, vec![]),
            (libc::SYS_mmap, vec![]),
            (libc::SYS_munmap, vec![]),
            (libc::SYS_fstat, vec![]),
            (libc::SYS_rt_sigaction, vec![]),
            (libc::SYS_openat, vec![]),
            (libc::SYS_close, vec![]),
            (libc::SYS_mprotect, vec![]),
            (libc::SYS_arch_prctl, vec![]),
            (libc::SYS_set_tid_address, vec![]),
            (libc::SYS_set_robust_list, vec![]),
            (libc::SYS_rseq, vec![]),
            (libc::SYS_prlimit64, vec![]),
            (libc::SYS_newfstatat, vec![]),
            (libc::SYS_access, vec![]),
            (libc::SYS_pread64, vec![]),
            (libc::SYS_getrandom, vec![]),
        ].into_iter().collect(),
        SeccompAction::KillProcess, // Action 1 : Tue net si la commande est inconnue
        SeccompAction::Trap,        // Action 2 : Lance une alarme système si les arguments sont louches
        TargetArch::x86_64,
    ).unwrap();

    let bpf_prog: BpfProgram = filter.try_into().unwrap();
    seccompiler::apply_filter(&bpf_prog).expect("Echec Seccomp");

    println!("🛡️ SAFE-RUN : Double verrouillage (Landlock + Seccomp) activé.");
    
    let err = Command::new(&args[0])
        .args(&args[1..])
        .exec();

    eprintln!("❌ Erreur : {}", err);
}
