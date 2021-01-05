use std::collections::HashMap;

fn main() {
	let action = std::env::args().nth(1).expect("Please specify an action");
	let item = std::env::args().nth(2).expect("Please specify an item");

	let mut todo = Todo { // instantiate struct
		map: HashMap::new(),
	};

	if action == "add" {
		todo.insert(item);
		match todo.save() { // MATCH the Result from the save fn and print a message
			Ok(_) => println!("todo saved"),
			Err(why) => println!("An error occured: {}", why),
		}
	}
}    

struct Todo {
	map: HashMap<String, bool>,
}


impl Todo {
	fn insert(&mut self, key: String) {
		self.map.insert(key, true);
	}

	fn save(self) -> Result<(), std::io::Error> {
		/*
			It's important to notice that save take ownership of self.
			This is an arbitrary decision so that the compiler would stop us if we were
			to accidentally try to update the map after we called save
			(as the memory of self would be freed).

			This means that save must be the last method used when writing this
			program (or it won't compile). THis is how we use Rust's memory management
			to create strict code!
		*/
		let mut content = String::new();
		for (k, v) in self.map {
			let record = format!("{}\t{}\n", k, v);
			content.push_str(&record)
		}
		std::fs::write("db.txt", content)
	}
}