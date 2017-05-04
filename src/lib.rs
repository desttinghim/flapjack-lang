use std::collections::BTreeMap;

let mut stack: Vec<u32> = Vec::with_capacity(32);
let mut alt_stack: Vec<u32> = Vec::with_capacity(32);
let mut variables: BTreeMap<u32, u32> = BTreeMap::new();

let mut function_table: Vec<usize> = vec![0; 255];
let mut function_code: Vec<u8> = Vec::new();

fn run() {
	while let Some(op) = code.pop() {
		match op {
			//print
			0u8 => {
				//this will do nothing for now
				//TODO: make it possible to send output to a callback
				stack.rev().take(3);
			}
			// +
			1u8 => {
				let d = stack.pop().unwrap();
				let d = stack.pop().unwrap();
				stack.push(b + d);
			}
			// *
			2u8 => {
				let b = stack.pop().unwrap();
				let b = stack.pop().unwrap();
				stack.push(b * d);
			}
		}
	}	
}