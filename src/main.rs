use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
  let args: Vec<String> = env::args().collect();

  let input_file_name: &str = &args[1];
  println!("Input file name: {}", args[1]);

  let contents: String = fs::read_to_string(input_file_name)
        .expect("Should have been able to read the file");
  println!("Lines: {}", contents);
  let output_file_name: &str = &args[2];
  println!("Output file name: {}", output_file_name);

  let table_name:&str = &args[3];

  let output_file = File::options().write(true).truncate(true).create(true).open(output_file_name).expect("Should have been able to create file");

  let lines: Vec<&str> = contents.split("\n").filter(|s| *s != "").collect::<Vec<&str>>();

  let headers = lines[0].split(",").collect::<Vec<&str>>();

  for line in lines[1..].iter() {
    let mut new_line: String = format!("UPDATE {table_name} SET");
    let values = line.split(",").collect::<Vec<&str>>();

    for i in 1..values.len() {
      new_line = format!("{} \"{}\" = '{}'", new_line, headers[i], values[i]);
      if i < values.len() - 1 {
        new_line = format!("{},", new_line);
      }
    }
    new_line = format!("{} WHERE {} = {}", new_line, headers[0], values[0]);
    println!("Inserting: {}", new_line);
    writeln!(&output_file, "{}", new_line).expect("Should be able to add a line");
  }
}
