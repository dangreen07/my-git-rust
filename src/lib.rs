use std::{cell::RefCell, mem::take, rc::Rc};

pub struct Git<'a> {
    name: &'a str,
    last_commit_id: i32,
    head: Rc<RefCell<Branch<'a>>>,
    branches: Vec<Rc<RefCell<Branch<'a>>>>
}

impl<'a> Git<'a> {
    /// # Usage
    /// Initialize a new git repository.
    /// ## Command example
    /// ```git init```
    pub fn new(name: &'a str) -> Git<'a> {
        let master = Branch::new("master", Box::new(None));
        let mut branches: Vec<Rc<RefCell<Branch>>> = Vec::with_capacity(1);
        let master = Rc::new(RefCell::new(master));
        let head = Rc::clone(&master);
        branches.push(master);

        Git {
            name: name,
            last_commit_id: -1,
            head: head,
            branches: branches
        }
    }

    /// # Usage
    /// Commit changes to a repository.
    /// ## Command example
    /// ```git commit -m "Initial Commit"```
    pub fn commit(&mut self, message: &'a str) {
        let new_id = self.last_commit_id + 1;
        self.last_commit_id = self.last_commit_id + 1;

        let mut self_head_commit = (*self.head).borrow_mut();
        let head_commit = take(&mut self_head_commit.commit);

        let commit = Commit::new(new_id, message,head_commit);

        self_head_commit.commit = Box::new(Some(commit));
    }

    /// # Usage
    /// Output a log of the commits in the current branch.
    /// ## Command example
    /// ```git log```
    pub fn log(&mut self) -> Vec<Commit<'a>> {
        let mut history: Vec<Commit> = Vec::new();
        let referenced = (*self.head).borrow_mut();
        let mut commit = &(*referenced.commit);
        while let Some(x) = commit {
            println!("{}", x.message);
            history.push(x.clone());
            let current = x.parent.as_ref();
            commit = current;
        }

        history
    }

    /// # Usage
    /// Switch the branch to a new branch or create it if it doesn't already exist.
    /// ## Command example
    /// ```git checkout existing-branch```  
    /// ```git checkout -b new-branch```
    pub fn checkout(&mut self, branch_name: &'a str) {
        // Loop through all branches and see if we have a branch
        // called `branchName`.
        let has_branch = self.find_existing_branch(branch_name);
        if has_branch {
            return;
        }

        // We reach here when no matching branch is found.

        // Cloned this value to avoid issues with mutable and immutable borrows
        let referenced = self.head.borrow_mut().clone();
        let commit = referenced.commit;

        let new_branch = Branch::new(branch_name, commit);
        let new_branch = RefCell::new(new_branch);
        let new_branch = Rc::new(new_branch);
        let new_branch_ref = Rc::clone(&new_branch);
        self.branches.push(new_branch);
        self.head = new_branch_ref;

        println!("Switched to new branch: {branch_name}");
    }

    fn find_existing_branch(&mut self, branch_name: &'a str) -> bool {
        for i in (0..self.branches.len()).rev() {
            let current_ref = self.branches.get(i).unwrap_or_else(|| panic!("Branch not found in repo!"));
            let current = (**current_ref).borrow_mut();
            if current.name == branch_name {
                println!("Switched to existing branch: {branch_name}");
                self.head = current_ref.clone();
                return true;
            }
        }
        return false;
    }

    /// The getter for the name of the repository
    pub fn name(&self) -> &str{
        &self.name
    }
}

#[derive(Clone)]
pub struct Commit<'a> {
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

    /// The getter for the message property of a commit
    pub fn message(&self) -> &'a str {
        self.message
    }

    /// The getter for the id
    pub fn id(&self) -> i32 {
        self.id
    }
}

#[derive(Clone)]
pub struct Branch<'a> {
    name: &'a str,
    commit: Box<Option<Commit<'a>>>
}

impl<'a> Branch<'a> {
    pub fn new(name: &'a str, commit: Box<Option<Commit<'a>>>) -> Branch<'a> {
        Branch {
            name: name,
            commit: commit
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn history_to_id_mapper<'a>(history: Vec<Commit<'a>>) -> String{
        let mut ids: Vec<String> = Vec::with_capacity(history.len());
        for item in history {
            ids.push(item.id.to_string());
        }
        let output = ids.join("-");
        return output;
    }

    #[test]
    fn git_log() {
        let mut repo = Git::new("test");
        repo.commit("Initial commit");
        repo.commit("Change 1");

        let log = repo.log();

        assert!(log.len() == 2);
        assert!(log[0].id == 1);
        assert!(log[1].id == 0);
    }

    #[test]
    fn git_checkout() {
        let mut repo = Git::new("test");
        repo.commit("Initial commit");
        assert!(repo.head.borrow_mut().name == "master");
        repo.checkout("testing");
        assert!(repo.head.borrow_mut().name == "testing");
        repo.checkout("master");
        assert!(repo.head.borrow_mut().name == "master");
        repo.checkout("testing");
        assert!(repo.head.borrow_mut().name == "testing");
    }

    #[test]
    fn git_branches_test() {
        let mut repo = Git::new("test");
        repo.commit("Initial commit");
        repo.commit("Change 1");
        assert!(history_to_id_mapper(repo.log()) == "1-0");
        repo.checkout("testing");
        repo.commit("Change 3");
        assert!(history_to_id_mapper(repo.log()) == "2-1-0");
        repo.checkout("master");
        assert!(history_to_id_mapper(repo.log()) == "1-0");
        repo.commit("Change 3");
        assert!(history_to_id_mapper(repo.log()) == "3-1-0");
    }
}