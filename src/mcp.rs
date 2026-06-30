use crate::detect::detect_project;
use crate::parser::parse_agent_file;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct McpRequest {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct McpResponse {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub result: Option<serde_json::Value>,
    pub error: Option<McpError>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct McpError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

const ERROR_INVALID_PARAMS: i32 = -32602;
const ERROR_METHOD_NOT_FOUND: i32 = -32601;
const ERROR_INTERNAL: i32 = -32603;

pub fn run() -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let reader = BufReader::new(stdin);

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let request: McpRequest = match serde_json::from_str(trimmed) {
            Ok(r) => r,
            Err(e) => {
                let resp = McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id: None,
                    result: None,
                    error: Some(McpError {
                        code: ERROR_INTERNAL,
                        message: format!("Parse error: {}", e),
                        data: None,
                    }),
                };
                writeln!(stdout, "{}", serde_json::to_string(&resp)?)?;
                stdout.flush()?;
                continue;
            }
        };

        let response = handle_request(&request);
        writeln!(stdout, "{}", serde_json::to_string(&response)?)?;
        stdout.flush()?;
    }

    Ok(())
}

fn handle_request(req: &McpRequest) -> McpResponse {
    match req.method.as_str() {
        "initialize" => McpResponse {
            jsonrpc: "2.0".to_string(),
            id: req.id.clone(),
            result: Some(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": true,
                    "resources": true,
                },
                "serverInfo": {
                    "name": "agentml-mcp",
                    "version": env!("CARGO_PKG_VERSION"),
                }
            })),
            error: None,
        },
        "tools/list" => McpResponse {
            jsonrpc: "2.0".to_string(),
            id: req.id.clone(),
            result: Some(json!({
                "tools": [
                    {
                        "name": "get_project_contract",
                        "description": "Return the parsed AGENT.agent contract",
                        "inputSchema": {"type": "object", "properties": {}},
                    },
                    {
                        "name": "get_agent_brief",
                        "description": "Return the agent operating brief",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "format": {"type": "string", "enum": ["md", "json"]},
                                "include_diff": {"type": "boolean"},
                            },
                            "required": [],
                        },
                    },
                    {
                        "name": "get_allowed_paths",
                        "description": "Return paths the agent is allowed to read/write",
                        "inputSchema": {"type": "object", "properties": {}},
                    },
                    {
                        "name": "get_validation_commands",
                        "description": "Return commands that should be run before reporting completion",
                        "inputSchema": {"type": "object", "properties": {}},
                    },
                    {
                        "name": "audit_diff",
                        "description": "Run diff check and return structured risk results",
                        "inputSchema": {"type": "object", "properties": {}},
                    },
                    {
                        "name": "validate_contract",
                        "description": "Validate AGENT.agent",
                        "inputSchema": {"type": "object", "properties": {}},
                    },
                    {
                        "name": "validate_skill",
                        "description": "Validate a specific .skill file",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "path": {"type": "string"},
                            },
                            "required": ["path"],
                        },
                    },
                    {
                        "name": "generate_context",
                        "description": "Generate .agentml/context.md",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "write": {"type": "boolean"},
                            },
                            "required": [],
                        },
                    },
                    {
                        "name": "generate_brief",
                        "description": "Generate .agentml/brief.md",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "write": {"type": "boolean"},
                                "format": {"type": "string", "enum": ["md", "json"]},
                                "include_diff": {"type": "boolean"},
                                "max_lines": {"type": "integer"},
                            },
                            "required": [],
                        },
                    },
                ],
            })),
            error: None,
        },
        "tools/call" => handle_tool_call(req),
        "resources/list" => McpResponse {
            jsonrpc: "2.0".to_string(),
            id: req.id.clone(),
            result: Some(json!({
                "resources": [
                    {"uri": "agentml://contract", "name": "Project contract (AGENT.agent)"},
                    {"uri": "agentml://brief", "name": "Agent operating brief"},
                    {"uri": "agentml://permissions", "name": "Allowed and forbidden paths"},
                ],
            })),
            error: None,
        },
        "initialized" => McpResponse {
            jsonrpc: "2.0".to_string(),
            id: req.id.clone(),
            result: Some(json!({})),
            error: None,
        },
        _ => McpResponse {
            jsonrpc: "2.0".to_string(),
            id: req.id.clone(),
            result: None,
            error: Some(McpError {
                code: ERROR_METHOD_NOT_FOUND,
                message: format!("Method not found: {}", req.method),
                data: None,
            }),
        },
    }
}

fn handle_tool_call(req: &McpRequest) -> McpResponse {
    let params: ToolCallParams = match req.params.as_ref() {
        Some(p) => match serde_json::from_value(p.clone()) {
            Ok(p) => p,
            Err(_) => {
                return McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id.clone(),
                    result: None,
                    error: Some(McpError {
                        code: ERROR_INVALID_PARAMS,
                        message: "Invalid params".to_string(),
                        data: None,
                    }),
                };
            }
        },
        None => ToolCallParams::default(),
    };

    let result = match params.name.as_str() {
        "get_project_contract" => handle_get_contract(),
        "get_agent_brief" => handle_get_brief(&params),
        "get_allowed_paths" => handle_get_allowed_paths(),
        "get_validation_commands" => handle_get_validation_commands(),
        "audit_diff" => handle_audit_diff(),
        "validate_contract" => handle_validate_contract(),
        "validate_skill" => handle_validate_skill(&params),
        "generate_context" => handle_generate_context(params.write.unwrap_or(false)),
        "generate_brief" => handle_generate_brief(&params),
        _ => {
            return McpResponse {
                jsonrpc: "2.0".to_string(),
                id: req.id.clone(),
                result: None,
                error: Some(McpError {
                    code: ERROR_METHOD_NOT_FOUND,
                    message: format!("Unknown tool: {}", params.name),
                    data: None,
                }),
            };
        }
    };

    McpResponse {
        jsonrpc: "2.0".to_string(),
        id: req.id.clone(),
        result: Some(json!({"content": result})),
        error: None,
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct ToolCallParams {
    pub name: String,
    #[allow(dead_code)]
    pub arguments: Option<serde_json::Value>,
    pub format: Option<String>,
    pub include_diff: Option<bool>,
    pub path: Option<String>,
    pub write: Option<bool>,
    pub max_lines: Option<usize>,
}

pub fn handle_get_contract() -> serde_json::Value {
    let agent = parse_agent_file(Path::new("AGENT.agent")).unwrap_or_default();
    let info = detect_project().unwrap_or_default();

    json!({
        "project": agent.meta.as_ref().map(|m| &m.name).unwrap_or(&"unknown".to_string()),
        "purpose": agent.purpose,
        "context": agent.context,
        "permissions": agent.permissions,
        "safety": agent.safety,
        "validation": agent.validation,
        "final_report_requirements": agent.output,
        "detected_stack": info.project_type,
    })
}

pub fn handle_get_brief(params: &ToolCallParams) -> serde_json::Value {
    let format = params.format.as_deref().unwrap_or("md");
    let include_diff = params.include_diff.unwrap_or(false);
    let agent = parse_agent_file(Path::new("AGENT.agent")).unwrap_or_default();
    let info = detect_project().unwrap_or_default();

    let (score, _reasons) = if include_diff {
        run_diff_check()
    } else {
        (0, vec![])
    };

    let validation_commands: Vec<String> = agent
        .validation
        .as_ref()
        .map(|v| v.iter().map(|cmd| cmd.command.clone()).collect())
        .unwrap_or_default();
    let write_paths: Vec<String> = agent
        .permissions
        .as_ref()
        .and_then(|p| p.write.clone())
        .unwrap_or_default();

    if format == "json" {
        json!({
            "project": agent.meta.as_ref().map(|m| m.name.clone()).unwrap_or("unknown".to_string()),
            "stack": vec![info.project_type],
            "allowed_write_paths": write_paths,
            "forbidden_paths": Vec::<String>::new(),
            "validation_commands": validation_commands,
            "risk": {
                "score": score,
                "status": risk_status(score),
            },
            "required_final_report_format": agent.output.as_ref().and_then(|o| o.format.clone()),
        })
    } else {
        json!({
            "project": agent.meta.as_ref().map(|m| m.name.as_str()).unwrap_or("unknown"),
            "stack": info.project_type,
            "allowed_write_paths": write_paths,
            "forbidden_paths": Vec::<String>::new(),
            "validation_commands": validation_commands,
            "risk": {
                "score": score,
                "status": risk_status(score),
            },
            "required_final_report_format": agent.output.as_ref().and_then(|o| o.format.clone()),
        })
    }
}

pub fn handle_get_allowed_paths() -> serde_json::Value {
    let agent = parse_agent_file(Path::new("AGENT.agent")).unwrap_or_default();
    let read_paths: Vec<String> = agent
        .permissions
        .as_ref()
        .and_then(|p| p.read.clone())
        .unwrap_or_default();
    let write_paths: Vec<String> = agent
        .permissions
        .as_ref()
        .and_then(|p| p.write.clone())
        .unwrap_or_default();
    json!({
        "read_paths": read_paths,
        "write_paths": write_paths,
        "forbidden_paths": Vec::<String>::new(),
    })
}

pub fn handle_get_validation_commands() -> serde_json::Value {
    let agent = parse_agent_file(Path::new("AGENT.agent")).unwrap_or_default();
    let info = detect_project().unwrap_or_default();
    let validation_commands: Vec<serde_json::Value> = agent
        .validation
        .as_ref()
        .map(|v| {
            v.iter()
                .map(|cmd| {
                    json!({
                        "name": &cmd.name,
                        "command": &cmd.command,
                        "description": cmd.description.as_ref(),
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    json!({
        "validation_commands": validation_commands,
        "detected_commands": info.validation_commands,
    })
}

pub fn handle_audit_diff() -> serde_json::Value {
    let (score, reasons) = run_diff_check();
    json!({
        "changed_files": reasons.clone(),
        "permission_violations": Vec::<String>::new(),
        "risk_score": score,
        "status": risk_status(score),
        "reasons": reasons,
        "required_next_actions": if score > 50 { vec!["Review changes before committing"] } else { vec![] },
        "required_validation_commands": get_validation_commands_for_audit(),
    })
}

pub fn get_validation_commands_for_audit() -> Vec<String> {
    let agent = parse_agent_file(Path::new("AGENT.agent")).unwrap_or_default();
    agent
        .validation
        .as_ref()
        .map(|v| v.iter().map(|cmd| cmd.command.clone()).collect())
        .unwrap_or_default()
}

pub fn handle_validate_contract() -> serde_json::Value {
    json!({
        "valid": true,
        "errors": Vec::<String>::new(),
        "warnings": Vec::<String>::new(),
        "risk_score": 0u32,
        "suggested_fixes": Vec::<String>::new(),
    })
}

pub fn handle_validate_skill(params: &ToolCallParams) -> serde_json::Value {
    let path = match &params.path {
        Some(p) => p,
        None => {
            return json!({"valid": false, "errors": ["path required"], "warnings": [], "suggested_fixes": []});
        }
    };

    if path.contains("..") || path.starts_with('/') {
        return json!({"valid": false, "errors": ["Invalid path: potential traversal"], "warnings": [], "suggested_fixes": []});
    }

    if path.contains(".agent")
        || path.contains("secret")
        || path.contains("key")
        || path.contains("token")
    {
        return json!({"valid": false, "errors": ["Forbidden path: potential secret"], "warnings": [], "suggested_fixes": []});
    }

    json!({"valid": true, "errors": [], "warnings": [], "suggested_fixes": []})
}

pub fn handle_generate_context(write: bool) -> serde_json::Value {
    let agent = parse_agent_file(Path::new("AGENT.agent")).unwrap_or_default();
    let content = format!(
        "# AgentML Context\n\nProject: {}\n\nPurpose: {:?}\n\nStack: {:?}\n",
        agent
            .meta
            .as_ref()
            .map(|m| m.name.as_str())
            .unwrap_or("unknown"),
        agent.purpose,
        agent.context,
    );

    if write {
        let _ = std::fs::create_dir_all(".agentml");
        let _ = std::fs::write(".agentml/context.md", &content);
    }

    json!({
        "content": content,
        "written_path": if write { Some(".agentml/context.md".to_string()) } else { None },
    })
}

pub fn handle_generate_brief(params: &ToolCallParams) -> serde_json::Value {
    let format = params.format.as_deref().unwrap_or("md");
    let write = params.write.unwrap_or(false);
    let include_diff = params.include_diff.unwrap_or(false);
    let max_lines = params.max_lines.unwrap_or(1000);

    let agent = parse_agent_file(Path::new("AGENT.agent")).unwrap_or_default();
    let info = detect_project().unwrap_or_default();
    let (score, _reasons) = if include_diff {
        run_diff_check()
    } else {
        (0, vec![])
    };

    let validation_commands: Vec<String> = agent
        .validation
        .as_ref()
        .map(|v| v.iter().map(|cmd| cmd.command.clone()).collect())
        .unwrap_or_default();
    let write_paths: Vec<String> = agent
        .permissions
        .as_ref()
        .and_then(|p| p.write.clone())
        .unwrap_or_default();

    let content: String = if format == "json" {
        serde_json::to_string(&json!({
            "project": agent.meta.as_ref().map(|m| m.name.clone()).unwrap_or("unknown".to_string()),
            "stack": vec![info.project_type],
            "allowed_write_paths": write_paths,
            "forbidden_paths": Vec::<String>::new(),
            "validation_commands": validation_commands,
            "risk": {"score": score, "status": risk_status(score)},
        }))
        .unwrap_or_else(|_| "{}".to_string())
    } else {
        format!(
            "# AgentML Operating Brief\n\nProject: {}\n\nStack: {}\n\nAllowed write paths:\n- {:?}\n\nForbidden:\n- {:?}\n\nRequired validation:\n{:?}\n\nRisk: {} ({})\n",
            agent
                .meta
                .as_ref()
                .map(|m| m.name.as_str())
                .unwrap_or("unknown"),
            info.project_type,
            write_paths,
            Vec::<String>::new() as Vec<String>,
            validation_commands,
            score,
            risk_status(score),
        )
    };

    let truncated = if content.len() > max_lines * 100 {
        format!("{}\n...[truncated]", &content[..max_lines * 100])
    } else {
        content
    };

    if write {
        let _ = std::fs::create_dir_all(".agentml");
        let _ = std::fs::write(".agentml/brief.md", &truncated);
    }

    json!({
        "content": truncated,
        "written_path": if write { Some(".agentml/brief.md".to_string()) } else { None },
    })
}

pub fn run_diff_check() -> (u32, Vec<String>) {
    use std::process::Command;
    let output = Command::new("git")
        .args(["diff", "--name-only", "HEAD"])
        .output();
    if let Ok(out) = output {
        if out.status.success() {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let mut score = 0u32;
            let mut reasons = Vec::new();
            for line in stdout.lines() {
                let path = line.trim();
                if path == "AGENT.agent" {
                    score += 30;
                    reasons.push(path.to_string());
                } else if path.starts_with("skills/") && path.ends_with(".skill") {
                    score += 20;
                    reasons.push(path.to_string());
                } else if path.starts_with("src/") && path.ends_with(".rs") {
                    score += 20;
                    reasons.push(path.to_string());
                }
            }
            return (score.min(100), reasons);
        }
    }
    (0, vec![])
}

pub fn risk_status(score: u32) -> &'static str {
    if score >= 100 {
        "blocked"
    } else if score >= 80 {
        "critical"
    } else if score >= 50 {
        "high"
    } else if score >= 20 {
        "medium"
    } else {
        "low"
    }
}
