use crate::actions::branch_ops::{BranchOperation, run_on_branches};

pub fn push(repo_path: &str, quiet: bool) {
    run_on_branches(repo_path, BranchOperation::Push, quiet);
}
