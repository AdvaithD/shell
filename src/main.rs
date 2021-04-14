use std::process::Command;
use std::io::{stdin, stdout, Write};

fn main() {
  loop {
    print!("> ");
    stdout().flush();

    let mut input = String::new();
    stdin().read_line(&imut input).unwrap();


    let mut parts = input..trim().split_whitespace();
    let command = parts.next().unwrap();
    let args = parts;

    let mut child = Command::new(command)
      .args(args)
      .spawn()
      .unwrap();

    child.wait();
  }
}
