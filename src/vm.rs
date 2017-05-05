use std::collections::BTreeMap;

pub struct VM {
	stack: 			Vec<u32>,
	alt_stack: 		Vec<u32>,
	variables: 		BTreeMap<u32, u32>,
	function_table: Vec<u32>,
	function_code: 	Vec<u8>,
	//native_code:	&'a mut Vec<Fn(&mut VM)->()>,
} 


pub const PRINT: u8 = 0;
pub const ADD: u8 	= 1;
pub const MUL: u8	= 2;
pub const SUB: u8	= 3;
pub const DIV: u8 	= 4;
pub const MOD: u8	= 5;
pub const PUSHR: u8	= 6;
pub const POPR: u8	= 7;
pub const STORE: u8	= 8;
pub const FETCH: u8	= 9;
pub const DUP: u8	= 10;
pub const SWAP: u8	= 11;
pub const ROT: u8	= 12;
pub const TUCK: u8 	= 13;
pub const DUP2: u8	= 14;
pub const SWAP2: u8	= 15;
pub const STARTDEF: u8 = 16;
pub const ENDDEF: u8 = 17;
pub const RECURSE: u8 = 18;
pub const CALL: u8	= 19;
pub const PUSH: u8	= 20;
pub const PUSHN: u8 = 21;
pub const PUSH1: u8 = 22;
pub const PUSH2: u8 = 23;
pub const PUSH3: u8 = 24;
pub const IF: u8	= 25;
pub const ELSE: u8	= 26;
pub const THEN: u8	= 27;
pub const EQ: u8	= 28;
pub const GT: u8	= 29;
pub const LT: u8	= 30;
pub const DROP: u8	= 31;
pub const STOP: u8 = 32;
pub const FETCHR: u8 = 33;
pub const OR: u8	= 34;
pub const AND: u8	= 35;

impl VM {
	pub fn new() -> VM {
		VM {
			stack: Vec::with_capacity(32),
			alt_stack: Vec::with_capacity(32),
			variables: BTreeMap::new(),
			function_table: vec![0; 255],
			function_code: Vec::new(),
			//native_code: Vec::new(),
		}
	}

	/*fn add_native(&self, func: Fn(&mut VM)->())->u32 {
		// TODO: figure out how to add functions to a vector
		self.native_code.push(func);
	}*/

	pub fn run(&mut self, mut code: Vec<u8>) -> Vec<u32> {

		// reverse code in place
		code.reverse();
		let stack = &mut self.stack;
		let alt_stack = &mut self.alt_stack;
		let variables = &mut self.variables;
		let function_table = &mut self.function_table;
		let function_code = &mut self.function_code;
		//let native_code = self.function_code;
		
		while let Some(op) = code.pop() {
			match op as u8 {
				//print
				PRINT => {
					//this will do nothing for now
					//TODO: make it possible to send output to a callback
					//stack.rev().take(3);
				}
				// +
				ADD => {
					let b = stack.pop().unwrap();
					let d = stack.pop().unwrap();
					stack.push(b + d);
				}
				// *
				MUL => {
					let b = stack.pop().unwrap();
					let d = stack.pop().unwrap();
					stack.push(b * d);
				}
				// -
				SUB => {
					let b = stack.pop().unwrap();
					let d = stack.pop().unwrap();
					stack.push(d - b);
				}
				// /
				DIV => {
					let b = stack.pop().unwrap();
					let d = stack.pop().unwrap();
					stack.push(b / d);
				}
				// %
				MOD => {
					let b = stack.pop().unwrap();
					let d = stack.pop().unwrap();
					stack.push(d % b);
				}
				// >r
				PUSHR => {
					let b = stack.pop().unwrap();
					alt_stack.push(b);
				}
				// r>
				POPR=> {
					let b = alt_stack.pop().unwrap();
					stack.push(b);
				}
				// ! (store value in variable)
				STORE => {
					let name = stack.pop().unwrap();
					let value = stack.pop().unwrap();
					variables.insert(name, value);
				}
				// @ (fetch value in variable)
				FETCH => {
					let name = stack.pop().unwrap();
					let value = *variables.get(&name).unwrap_or(&0);
					stack.push(value);
				}
				// dup
				DUP => {
					let ab = stack.pop().unwrap();
					stack.push(ab);
					stack.push(ab);
				}
				// swap
				SWAP => {
					let a = stack.pop().unwrap();
					let b = stack.pop().unwrap();
					stack.push(a);
					stack.push(b);
				}
				// rot
				ROT => {
					let a = stack.pop().unwrap();
					let b = stack.pop().unwrap();
					let c = stack.pop().unwrap();
					stack.push(b);
					stack.push(c);
					stack.push(a);
				}
				// tuck
				TUCK => {
					let a = stack.pop().unwrap();
					let b = stack.pop().unwrap();
					let c = stack.pop().unwrap();
					stack.push(c);
					stack.push(a);
					stack.push(b);
				}
				// 2dup
				DUP2 => {
					let a = stack.pop().unwrap();
					let b = stack.pop().unwrap();
					stack.push(b);
					stack.push(a);
					stack.push(b);
					stack.push(a);
				}
				// 2swap
				SWAP2 => {
					let a = stack.pop().unwrap();
					let b = stack.pop().unwrap();
					let a2 = stack.pop().unwrap();
					let b2 = stack.pop().unwrap();
					stack.push(b);
					stack.push(a);
					stack.push(b2);
					stack.push(a2);
				}
				// : (define a word)
				STARTDEF => {
					let name = code.pop().unwrap();
					function_code.push(17u8);
					loop {
						match code.pop().expect("unterminated function") {
							ENDDEF => break,
							RECURSE => function_code.push(name),
							op => function_code.push(op)
						}
					}
					function_table[name as usize] = function_code.len() as u32;
				}
				ENDDEF|RECURSE => unreachable!(),
				// call
				CALL => {
					let name = stack.pop().unwrap();
					let function_start = function_table[name as usize];
					assert!(function_start != 0, "attempted to call undefined function");
					for &byte in function_code[..function_start as usize].iter().rev() {
						match byte {
							ENDDEF => break,
							_ => code.push(byte),
						}
					}
				}
				// push
				PUSH => {
					let b = stack.pop().unwrap();
					for _ in 0..b {
						let d = code.pop().unwrap();
						stack.push(d as u32);
					}
				}
				// pushn
				PUSHN => {
					let y = code.pop().unwrap();
					stack.push(y as u32);
				}
				// push1..3
				PUSH1|PUSH2|PUSH3 => {
					let count = op - 21;
					for _ in 0..count {
						let z = code.pop().unwrap() as u32;
						let b = code.pop().unwrap() as u32;
						let e = code.pop().unwrap() as u32;
						let d = code.pop().unwrap() as u32;
						let y = (d << 24) | (e << 16) | (b << 8) | z;
						stack.push(y);
					}
				}
				// if
				IF => {
					let y = stack.pop().unwrap();
					if y == 0 {
						//skip to else
						while code.pop().unwrap() != 26 { }
					}
				}
				// skip over else
				ELSE => {
					while code.pop().unwrap() != 27 { }
				}
				// endif
				THEN => {}
				// ==
				EQ => {
					let y = stack.pop().unwrap();
					let z = stack.pop().unwrap();
					stack.push((z == y) as u32);
				}
				// >
				GT => {
					let y = stack.pop().unwrap();
					let z = stack.pop().unwrap();
					stack.push((z > y) as u32);
				}
				// <
				LT => {
					let y = stack.pop().unwrap();
					let z = stack.pop().unwrap();
					stack.push((z < y) as u32);
				}
				// drop
				DROP => {
	 				stack.pop().unwrap();
				}
				// stop
				STOP => break,
				// r@
				FETCHR => {
					let z = alt_stack.pop().unwrap();
					stack.push(z);
					alt_stack.push(z);
				}
				// or
				OR => {
					let z = stack.pop().unwrap();
					let y = stack.pop().unwrap();
					stack.push((z != 0u32 || y != 0u32) as u32);
				}
				// and
				AND => {
					let z = stack.pop().unwrap();
					let y = stack.pop().unwrap();
					stack.push((z != 0u32 && y != 0u32) as u32);
				}
				// call native
				/*36u8 => {
					let value = stack.pop().unwrap();
					let f = self.native_code[value];
					f(self);
				}*/
				_ => panic!("unknown op code {}", op),
			} // match op
		}
		stack.to_vec()
	} // run()
}
