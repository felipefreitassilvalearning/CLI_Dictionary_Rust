use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    // If it's null, break
    let key = arguments.next().unwrap();
    // "Optional"
    // let value = arguments.next();
    // Show error case not provided
    let value = arguments.next().expect("Value not provided");
    // println!("The key is {}", key);
    // println!("The value is {}", value);
    println!("{{ \"{}\": \"{}\" }}", key, value);
    // // format! macro works exactly like print[ln]!
    // let contents = format!("{}\t{}\n", key, value);
    // // let write_result =
    // std::fs::write("kv.db", contents).unwrap();
    // // To handle any errors:
    // // match write_result {
    // //     Ok(()) => {
    // //      ...
    // //     }
    // //     Err(e) => {
    // //      ...
    // //     }
    // // }

    let mut database = Database::new().expect("Creating db failed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    // Will not work since value was given to previous insert
    // println!("The value was {}", value);
    match database.flush() {
        Ok(()) => println!("YAY!"),
        Err(err) => println!("OH NOS! Error! {}", err),
    }
}

struct Database {
    map: HashMap<String, String>,
    flush: bool,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        // read the kv.db file
        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => {
        //         return Err(error);
        //     }
        // };
        // Match function is equivalent to "?"
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            // Newer alternative
            // let (key, value) = line.split_once('\t').expect("Corrupt database");
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No value!");
            // Two ways of implementing the same thing
            map.insert(key.to_owned(), String::from(value));
        }
        // parse the string
        // populate the map
        Ok(Database { map, flush: false })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(mut self) -> std::io::Result<()> {
        let mut contents = String::new();
        for (key, value) in &self.map {
            // let kvpair = format!("{}\t{}\n", key, value);
            // contents.push_str(&kvpair);
            // Or
            // push_str = "Word"
            contents.push_str(key);
            // push = 'char'
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }
        // // Add to remove errors and warnings while developing
        // todo!("Finish this method")
        self.flush = true;
        std::fs::write("kv.db", contents)
    }
}

impl Drop for Database {
    // Called "on desctruct", before freeing all memory
    fn drop(&mut self) {
        if (self.flush) {
            let mut contents = String::new();
            for (key, value) in &self.map {
                contents.push_str(key);
                contents.push('\t');
                contents.push_str(value);
                contents.push('\n');
            }
            // _[var] = ignore return value
            let _ = std::fs::write("kv.db", contents);
        }
    }
}
