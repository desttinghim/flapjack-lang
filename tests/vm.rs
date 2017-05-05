extern crate flapjack;

use flapjack::vm::VM;
use flapjack::vm;

#[test]
fn addition() {
	let mut vm = VM::new();
	let result = vm.run(vec![vm::PUSHN, 5, 	// pushn 5
							 vm::PUSHN, 5, 	// pushn 5
							 vm::PUSHN, 1, 	// pushn 1
							 vm::ADD,		// +
							 vm::MUL 		// *
							]);
	assert_eq!(Some(&30), result.last());
}
/*
#[test]
fn native() {
	use std::sync::mpsc;
	let (tx, rx) = mpsc::channel();
	let native_func = |_| {tx.send("Hello");};
	let mut vm = VM::new();
	let print_xt = vm.add_native(&native_func);
	vm.run(vec![vm::PUSHN, 5, vm::PUSH,
				0,
				(print_xt >> 24) as u8,
				(print_xt >> 16) as u8,
				(print_xt >> 8) as u8,
				(print_xt) as u8,
				vm::NATIVE]);
	assert_eq!(rx.recv(), Ok("Hello"));
}
*/

