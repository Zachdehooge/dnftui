fn searchDNF(repo: &str) -> String{
    let command = format!("dnf search {repo}");
    command
}

fn main() {
    println!("{}", searchDNF("vscode"));
}
