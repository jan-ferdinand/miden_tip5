//! Contains Miden assembly of the [Tip5 permutation](https://eprint.iacr.org/2023/107.pdf).
//! The binary runs the permutation once on statically defined input.

use miden_stdlib::StdLibrary;
use miden_vm::prove;
use miden_vm::verify;
use miden_vm::Assembler;
use miden_vm::Kernel;
use miden_vm::MemAdviceProvider;
use miden_vm::ProgramInfo;
use miden_vm::ProofOptions;
use miden_vm::StackInputs;

/// The [Tip5](https://eprint.iacr.org/2023/107.pdf) permutation.
///
/// While this is technically not a Miden library, it should be relatively easy to convert it to
/// one.
pub const TIP5_LIB: &str = "proc.tip5 push.5 add end begin push.3 call.tip5 end";

fn main() {
    let assembler = Assembler::default()
        .with_library(&StdLibrary::default())
        .map_err(|err| format!("Failed to load stdlib - {err}"))
        .unwrap()
        .with_debug_mode(true);

    let program = assembler
        .compile(TIP5_LIB)
        .map_err(|err| format!("Failed to compile program - {err}"))
        .unwrap();

    let (outputs, proof) = prove(
        &program,
        StackInputs::default(),
        MemAdviceProvider::default(),
        ProofOptions::default(),
    )
    .unwrap();

    assert_eq!(Some(&8), outputs.stack().first());

    let program_info = ProgramInfo::new(program.hash(), Kernel::default());
    match verify(program_info, StackInputs::default(), outputs, proof) {
        Ok(_) => println!("Execution verified!"),
        Err(msg) => println!("Something went terribly wrong: {msg}"),
    }
}
