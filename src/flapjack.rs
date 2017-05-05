/*std::collections:BTreeMap;

struct FlapJack {
	vm: VM,
	dict: BTree<String, u32>,
}

// Implemented as an "object" so it can stick around
// and have callbacks executed

enum FlapJack {
	String(String),
	Word(String),
	Number(u32),
}

impl FlapJack {
	fn new() -> FlapJack {
		FlapJack {
			vm: VM::new(),
			dict: BTreeMap::new(),
		}
	}

	fn add_fn(&self, name: String, func: Fn(&mut VM)->()) {
		// in: function name, function
		// out: n/a
		// side: new vm native_code xt, new entry in dict
		let xt = self.vm.add_native(func);
		self.dict.insert(name, xt);
	}

	fn execute(&self, code: &str) {
		// in: script text to run
		// out: n/a
		// TODO: parse into tokens and compile
		vm.run(compile(parse(code))); // something like this
	}

	fn parse(code: &str)-> {
		// in: code in text format
		// out: list of tokens
		let tokens = vec![0; 255]
		for c in code.chars() {
			// amazing things
		}
	}

	fn compile(tokens) {
		// in: list of tokens
		// out: byte code
	}
}*/