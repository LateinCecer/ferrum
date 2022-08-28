use ferrum::bytecode::chunk::Chunk;
use ferrum::bytecode::opcode::{NumeralType, OpCode};
use ferrum::vm::VM;

fn main() -> Result<(), ()> {
    let mut chunk = Chunk::new(String::from("main"));
    chunk.write(OpCode::Const(0, 8), 0, 0);
    chunk.write_value(1.2 as f64);
    chunk.write(OpCode::Neg(NumeralType::F64), 0, 6);
    chunk.write(OpCode::Const(8, 8), 1, 0);
    chunk.write_value(2.0 as f64);
    chunk.write(OpCode::Mul(NumeralType::F64), 2, 16);
    chunk.write(OpCode::Ret, 2, 0);

    println!("chunk: {:#?}", chunk);
    println!("\nrunning vm...");

    let mut vm = VM::new(chunk);
    while vm.is_active {
        println!("{:?}", vm);
        vm.cycle().map_err(|e| {
            println!("ERROR: {}", e);
            ()
        })?;
    }
    println!("Process finished with exit code: {}", vm.exit_code);
    Ok(())
}
