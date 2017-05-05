extern crate flapjack;
use flapjack::vm::VM;
use flapjack::vm;

fn main() {
	let print_func = |x:&mut Vec<u32>| {print!("{}", x.pop().unwrap() as u8 as char);};
	let mut vm = VM::new();
	let print_xt = vm.add_native(&print_func);
	vm.run(vec![vm::PUSHN, 6, vm::PUSH,	// push 6 numbers onto stack
				'H' as u8,
				1,
				(print_xt >> 24) as u8,
				(print_xt >> 16) as u8,
				(print_xt >> 8) as u8,
				(print_xt) as u8,
				vm::NATIVE]);
}
