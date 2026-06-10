use ubu_github_adapter::projection::labels::apply_managed_label;
use ubu_github_adapter::projection::operations::GitHubProjectionTarget;
use ubu_github_adapter::projection::preview::preview_for_operations;

fn main() -> ubu_github_adapter::Result<()> {
    let operation = apply_managed_label(
        "example-label",
        GitHubProjectionTarget::issue("UbU-project", "ubu-github-adapter", 7),
        "ubu",
    );
    let preview = preview_for_operations(vec![operation])?;
    println!("{}", serde_json::to_string_pretty(&preview)?);
    Ok(())
}
