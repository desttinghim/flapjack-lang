std::collections:BTreeMap;

struct FlapJack {
	vm: VM,
	dict: BTree<String, u32>,
}

// Implemented as an "object" so it can stick around
// and have callbacks executed

impl FlapJack {
	fn new() -> FlapJack {
		FlapJack {
			vm: VM::new(),
			dict: BTreeMap::new(),
		}
	}

	fn add_fn(&self, name: String, func: fn()->Void) {
		// in: function name, function
		// out: n/a
		// side: new vm native_code xt, new entry in dict
		let xt = self.vm.add_native(func);
		self.dict.insert(name, xt);
	}

	fn execute(&self, code) {
		// in: script text to run
		// out: n/a
		// TODO: parse into tokens and compile
		vm.run(compile(parse)); // something like this
	}

	fn parse(code) {
		// in: code in text format
		// out: list of tokens
	}

	fn compile(tokens) {
		// in: list of tokens
		// out: byte code
	}
}