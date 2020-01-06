use std::env;
use std::fs;

fn main () {
  // get the filename passed as an argument
  let arguments: Vec<String> = env::args().collect();
  //borrow or reference to the filename
  let filename = &arguments[1];
  println!("The name of the file is: {}", filename);
  // get all the massess of the modules
  let masses = fs::read_to_string(filename)
    .expect("Something is not right with reading the file");
  println!("The various masses for modules found in the file are: {:?}", masses);
  // calculate & transform the fuel needs for the above masses
  let fuelvector: Vec<i32> = masses.split("\n").filter_map( |w| w.parse().ok()).map( |x: i32| x/3 - 2).collect();
  println!("The various fuel needs for the given modules are: {:?}", fuelvector);
  // calculate total sum fuel needs
  let fuelsum: i32 = fuelvector.iter().sum();
  println!("The sum of the fuel requirements for all of the modules on your spacecraft: {:?}", fuelsum);
}