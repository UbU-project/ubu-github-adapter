use ubu_core::UbuId;

pub const MANAGED_LABEL: &str = "ubu-managed";
pub const UBU_LABEL: &str = "ubu";
pub const MANAGED_COMMENT_MARKER: &str = "<!-- ubu-managed: true -->";
pub const ORIGIN_MARKER_PREFIX: &str = "<!-- ubu-origin: ubu://task/";

pub const MANAGED_LABELS: [&str; 2] = [UBU_LABEL, MANAGED_LABEL];

pub fn origin_marker(task_id: &UbuId) -> String {
    format!("{ORIGIN_MARKER_PREFIX}{task_id} -->")
}

pub fn managed_body(task_id: &UbuId, body: &str) -> String {
    format!(
        "{MANAGED_COMMENT_MARKER}\n{}\n\n{}",
        origin_marker(task_id),
        body
    )
}

pub fn has_managed_markers(body: &str) -> bool {
    body.contains(MANAGED_COMMENT_MARKER) && body.contains(ORIGIN_MARKER_PREFIX)
}

pub fn is_managed_label(label: &str) -> bool {
    MANAGED_LABELS.contains(&label)
}
