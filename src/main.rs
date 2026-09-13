use std::io;

fn search_dnf(repo: &str) -> String{
    let command = format!("dnf search {repo}");
    command
}

fn main() {
    let mut input = String::new();

    println!("Enter a repo:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let name = input.trim();

    println!("{}", search_dnf(&name));
}
