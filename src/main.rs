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
// todo! use constants from their definition in tip5 instead of hardcoding them
pub const TIP5_LIB: &str = "
    proc.tip5_init
        # push.0
        # mem_store.0
        push.7
        mem_store.1
        push.26
        mem_store.2
        push.63
        mem_store.3
        push.124
        mem_store.4
        push.215
        mem_store.5
        add
    end

    proc.tip5
       # round_0
       # round_1
       # round_2
       # round_3
       # round_4
    end

    begin
        call.tip5_init
        call.tip5
    end
";

fn main() {
    let assembler = Assembler::default()
        .with_library(&StdLibrary::default())
        .unwrap();

    let program = assembler.compile(TIP5_LIB).unwrap();

    let (outputs, proof) = prove(
        &program,
        StackInputs::default(),
        MemAdviceProvider::default(),
        ProofOptions::default(),
    )
    .unwrap();

    let program_info = ProgramInfo::new(program.hash(), Kernel::default());
    match verify(program_info, StackInputs::default(), outputs, proof) {
        Ok(_) => println!("Execution verified!"),
        Err(msg) => println!("Something went terribly wrong: {msg}"),
    }
}

#[cfg(test)]
mod tests {
    use miden_vm::execute;
    use miden_vm::MemAdviceProvider;
    use miden_vm::StackInputs;

    use crate::*;

    #[test]
    fn compliance() {
        let assembler = Assembler::default()
            .with_library(&StdLibrary::default())
            .unwrap();

        let program = assembler.compile(TIP5_LIB).unwrap();

        let stack_inputs = StackInputs::default();
        let mut advice_provider = MemAdviceProvider::default();
        let trace = execute(&program, stack_inputs.clone(), &mut advice_provider).unwrap();
        let advice_provider = MemAdviceProvider::default();
        let trace = execute(&program, stack_inputs, advice_provider).unwrap();
        let public_output = trace.stack_outputs().stack();

        let expected_output = vec![0; 16];
        assert_eq!(expected_output, public_output);
    }
}
