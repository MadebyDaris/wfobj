
use wfobj::*;

fn main() {
    let world =  parse_file(include_str!("../test/monkey.obj"));
    match world {
        Ok(x) => println!("nomber of vertices is {:?}", x.vertices.len()),
        Err(e) => println!("{}", e),
    }
}