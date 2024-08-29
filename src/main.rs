use std::mem::take;

struct Git<'a> {
    name: &'a str,
    last_commit_id: i32,
    head: Box<Option<Commit<'a>>>
}

impl<'a> Git<'a> {
    pub fn new(name: &'a str) -> Git<'a> {
        Git {
            name: name,
            last_commit_id: -1,
            head: Box::new(None)
        }
    }

    pub fn commit(&mut self, message: &'a str) {
        let new_id = self.last_commit_id + 1;
        // Taking ownership of the self.head field
        let head = take(&mut self.head);

        let commit = Commit::new(new_id, message,head);

        self.head = Box::new(Some(commit));
    }

    pub fn log(&self) -> Vec<&Commit> {
        let mut history: Vec<&Commit> = Vec::new();
        let mut commit = &(*self.head);
        while let Some(ref x) = commit {
            println!("{}",x.message);
            commit = &(*x.parent);
            history.push(x);
        }
        return history;
    }
}


struct Commit<'a> {
    id: i32,
    message: &'a str,
    parent: Box<Option<Commit<'a>>>
}

impl<'a> Commit<'a> {
    pub fn new(id: i32, message: &'a str, parent: Box<Option<Commit<'a>>>) -> Commit<'a> {
        Commit {
            id: id,
            message: message,
            parent: parent
        }
    }
}

fn main() {
    let mut repo = Git::new("test");
    repo.commit("Initial Commit");
    repo.commit("Second Commit");
    let history = repo.log();
    let first_commit = history[1].message;
    println!("First commit message: {first_commit}");
    println!("Number of Commits: {}",history.len());
}
