std::collections:BTreeMap;

struct FlapJack {
	vm: VM,
	dict: BTree<String, u32>,
}

impl FlapJack {
	fn new() -> FlapJack {
		FlapJack {
			vm: VM::new(),
			dict: BTreeMap::new(),
		}
	}
}