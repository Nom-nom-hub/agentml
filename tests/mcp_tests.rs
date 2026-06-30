use agentml::mcp::{
    handle_audit_diff, handle_generate_context, handle_generate_brief,
    handle_get_brief, handle_get_contract, handle_validate_contract,
    handle_validate_skill, risk_status, ToolCallParams,
};

#[test]
fn mcp_get_project_contract_returns_parsed_contract() {
    let result = handle_get_contract();
    assert!(result.is_object());
    let obj = result.as_object().unwrap();
    assert!(obj.contains_key("project"));
    assert!(obj.contains_key("purpose"));
    assert!(obj.contains_key("validation"));
}

#[test]
fn mcp_get_agent_brief_includes_forbidden_paths() {
    let params = ToolCallParams {
        name: "get_agent_brief".to_string(),
        format: Some("json".to_string()),
        include_diff: Some(false),
        ..Default::default()
    };
    let result = handle_get_brief(&params);
    assert!(result.is_object());
    let obj = result.as_object().unwrap();
    assert!(obj.contains_key("forbidden_paths"));
    let forbidden = obj.get("forbidden_paths").unwrap().as_array().unwrap();
    assert!(forbidden.is_empty());
}

#[test]
fn mcp_audit_diff_returns_risk_score() {
    let result = handle_audit_diff();
    assert!(result.is_object());
    let obj = result.as_object().unwrap();
    assert!(obj.contains_key("risk_score"));
    assert!(obj.contains_key("status"));
    let score = obj.get("risk_score").unwrap().as_u64().unwrap();
    assert!(score <= 100);
}

#[test]
fn mcp_validate_contract_returns_valid_true() {
    let result = handle_validate_contract();
    assert!(result.is_object());
    let obj = result.as_object().unwrap();
    assert_eq!(obj.get("valid").unwrap().as_bool().unwrap(), true);
    assert!(obj.get("errors").unwrap().as_array().unwrap().is_empty());
}

#[test]
fn mcp_validate_skill_blocks_path_traversal() {
    let params = ToolCallParams {
        name: "validate_skill".to_string(),
        path: Some("../etc/passwd".to_string()),
        ..Default::default()
    };
    let result = handle_validate_skill(&params);
    assert_eq!(result["valid"].as_bool().unwrap(), false);
    assert!(result["errors"].as_array().unwrap()[0]
        .as_str().unwrap().contains("traversal"));
}

#[test]
fn mcp_generate_context_write_only_writes_to_agentml() {
    let result = handle_generate_context(true);
    assert!(result.is_object());
    assert_eq!(result["written_path"].as_str().unwrap(), ".agentml/context.md");
}

#[test]
fn mcp_generate_brief_write_only_writes_to_agentml() {
    let params = ToolCallParams {
        name: "generate_brief".to_string(),
        write: Some(true),
        format: Some("md".to_string()),
        ..Default::default()
    };
    let result = handle_generate_brief(&params);
    assert_eq!(result["written_path"].as_str().unwrap(), ".agentml/brief.md");
}

#[test]
fn mcp_rejects_unknown_tools_cleanly() {
    let params = ToolCallParams {
        name: "unknown_tool".to_string(),
        ..Default::default()
    };
    let result = handle_get_brief(&params);
    assert!(result.is_object());
}

#[test]
fn mcp_does_not_panic_on_malformed_input() {
    let params = ToolCallParams {
        name: "validate_skill".to_string(),
        path: Some("/etc/passwd".to_string()),
        ..Default::default()
    };
    let result = handle_validate_skill(&params);
    assert!(result.is_object());
}

#[test]
fn risk_status_returns_correct_levels() {
    assert_eq!(risk_status(150), "blocked");
    assert_eq!(risk_status(90), "critical");
    assert_eq!(risk_status(60), "high");
    assert_eq!(risk_status(30), "medium");
    assert_eq!(risk_status(10), "low");
}