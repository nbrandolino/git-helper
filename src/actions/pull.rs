use crate::actions::branch_ops::{BranchOperation, run_on_branches};

pub fn pull(repo_path: &str, quiet: bool) -> bool {
    run_on_branches(repo_path, BranchOperation::Pull, quiet)
}
