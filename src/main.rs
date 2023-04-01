use miden_stdlib::StdLibrary;
use miden_vm::prove;
use miden_vm::verify;
use miden_vm::Assembler;
use miden_vm::Kernel;
use miden_vm::MemAdviceProvider;
use miden_vm::ProgramInfo;
use miden_vm::ProofOptions;
use miden_vm::StackInputs;

fn main() {
    let assembler = Assembler::default()
        .with_library(&StdLibrary::default())
        .map_err(|err| format!("Failed to load stdlib - {}", err))
        .unwrap()
        .with_debug_mode(true);

    let program = assembler
        .compile("proc.tip5 push.5 add end begin push.3 call.tip5 end")
        .map_err(|err| format!("Failed to compile program - {}", err))
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
        Err(msg) => println!("Something went terribly wrong: {}", msg),
    }
}
