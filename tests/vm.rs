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
	assert!(Some(&30) == result.last());
}

//#[test]
//fn 