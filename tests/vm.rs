extern crate flapjack;

use flapjack::vm::VM;

#[test]
fn addition() {
	let mut vm = VM::new();
	let result = vm.run(vec![21, 5, 	// pushn 5
							 21, 5, 	// pushn 5
							 21, 1, 	// pushn 1
							 1,		// +
							 2 		// *
							]);
	assert!(Some(&30) == result.last());
}