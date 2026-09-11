fn search_dnf(repo: &str) -> String{
    let command = format!("dnf search {repo}");
    command
}

fn main() {
    println!("{}", search_dnf("vscode"));
}
