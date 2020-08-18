mod data_types;
mod guess;

use std::io;
use std::collections::HashMap;
use std::process::exit;


struct Module {
    name: &'static str,
    func: fn()
}

fn main() {
    let mut mods = HashMap::new();
    mods.insert("0", Module {name: "Guess number", func: guess::main});
    mods.insert("1", Module {name: "Data Types playground", func: data_types::main});
    println!("Select module for run:");
    for (key, data) in &mods {
        println!("{} - {}", key, data.name);
    }
    let mut item = String::new();
    io::stdin().read_line(&mut item).expect("Failed to read line");
    let item = item.trim();
    if mods.contains_key(&item){
        println!("{}", mods[&item].name);
        let func =  mods[&item].func;
        func();
    }else{
        println!("Module was not selected! Bye!");
        exit(0);
    }
}
