use ubu_core::{ObjectType, UbuId};
use ubu_github_adapter::markers::{has_managed_markers, managed_body};

#[test]
fn managed_issue_body_contains_required_markers() {
    let task_id = UbuId::new(ObjectType::Task);
    let body = managed_body(&task_id, "Create this from UbU.");

    assert!(body.contains("<!-- ubu-managed: true -->"));
    assert!(body.contains(&format!("<!-- ubu-origin: ubu://task/{task_id} -->")));
    assert!(has_managed_markers(&body));
}
