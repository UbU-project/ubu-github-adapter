use ubu_github_adapter::cli::import_fixture::import_fixture;

fn main() -> ubu_github_adapter::Result<()> {
    let mapping = import_fixture("fixtures/github/issues-small.json")?;
    println!("imported {} task candidates", mapping.candidates.len());
    Ok(())
}
