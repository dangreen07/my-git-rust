use my_git_rust::Git;

fn main() {
    let mut repo = Git::new("test");
    repo.commit("Initial Commit");
    repo.commit("Second Commit");
    let history = repo.log();
    let first_commit = history[1].message();
    println!("First commit message: {first_commit}");
    println!("Number of Commits: {}",history.len());
}
