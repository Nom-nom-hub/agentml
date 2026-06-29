use crate::types::{AgentFile, SkillFile};
use colored::Colorize;
use serde_yaml::Value;

fn get_purpose_string(purpose: &Option<Value>) -> Option<String> {
    purpose.as_ref().and_then(|v| {
        if let Some(s) = v.as_str() {
            Some(s.to_string())
        } else if let Some(map) = v.as_mapping() {
            let mut parts = Vec::new();
            if let Some(hg) = map.get(Value::String("human_goal".to_string()))
                && let Some(s) = hg.as_str()
            {
                parts.push(s.to_string());
            }
            if let Some(ag) = map.get(Value::String("agent_goal".to_string()))
                && let Some(s) = ag.as_str()
            {
                parts.push(s.to_string());
            }
            if let Some(ng) = map.get(Value::String("non_goals".to_string()))
                && let Some(arr) = ng.as_sequence()
            {
                for item in arr {
                    if let Some(s) = item.as_str() {
                        parts.push(s.to_string());
                    }
                }
            }
            if parts.is_empty() {
                None
            } else {
                Some(parts.join(" "))
            }
        } else {
            None
        }
    })
}

pub fn get_forbidden_paths(safety: &Option<Value>) -> Option<Vec<String>> {
    safety.as_ref().and_then(|v| {
        if let Some(map) = v.as_mapping()
            && let Some(paths) = map.get(Value::String("forbidden_paths".to_string()))
            && let Some(arr) = paths.as_sequence()
        {
            return Some(
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect(),
            );
        }
        None
    })
}

pub fn get_forbidden_actions(safety: &Option<Value>) -> Option<Vec<String>> {
    safety.as_ref().and_then(|v| {
        if let Some(map) = v.as_mapping() {
            if let Some(actions) = map.get(Value::String("forbidden_actions".to_string()))
                && let Some(arr) = actions.as_sequence()
            {
                return Some(
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect(),
                );
            }
            if let Some(da) = map.get(Value::String("destructive_actions".to_string()))
                && let Some(da_map) = da.as_mapping()
                && let Some(commands) = da_map.get(Value::String("commands".to_string()))
                && let Some(arr) = commands.as_sequence()
            {
                return Some(
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect(),
                );
            }
        }
        None
    })
}

pub fn get_require_confirmation(safety: &Option<Value>) -> Option<Vec<String>> {
    safety.as_ref().and_then(|v| {
        if let Some(map) = v.as_mapping()
            && let Some(rc) = map.get(Value::String("require_confirmation".to_string()))
            && let Some(arr) = rc.as_sequence()
        {
            return Some(
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect(),
            );
        }
        None
    })
}

pub fn has_secrets_policy(safety: &Option<Value>) -> bool {
    safety
        .as_ref()
        .map(|v| {
            if let Some(map) = v.as_mapping() {
                if map.contains_key(Value::String("secrets_policy".to_string())) {
                    return true;
                }
                if let Some(policy) = map.get(Value::String("policy".to_string()))
                    && let Some(s) = policy.as_str()
                {
                    return s.to_lowercase().contains("secret");
                }
            }
            false
        })
        .unwrap_or(false)
}

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub code: &'static str,
    pub message: String,
    pub fix: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ValidationWarning {
    pub code: &'static str,
    pub message: String,
    pub suggestion: String,
}

#[derive(Debug, Clone)]
pub struct RiskFactor {
    pub severity: &'static str,
    pub description: String,
}

#[derive(Debug)]
pub struct ValidationReport {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
    pub risk_score: u32,
    pub risk_factors: Vec<RiskFactor>,
}

impl ValidationReport {
    pub fn invalid(errors: Vec<ValidationError>) -> Self {
        Self {
            valid: false,
            errors,
            warnings: Vec::new(),
            risk_score: 0,
            risk_factors: Vec::new(),
        }
    }

    pub fn print(&self) {
        println!("\n{}", "══ AgentML Validation Report ══".cyan().bold());
        println!();

        if self.valid {
            println!("{}", "✔ VALID".green().bold());
        } else {
            println!("{}", "✘ INVALID".red().bold());
        }
        println!();

        if !self.errors.is_empty() {
            println!("{}", "Errors:".red().bold());
            for err in &self.errors {
                println!(
                    "  {} {}",
                    format!("[{}]", err.code).red(),
                    err.message.red()
                );
                if let Some(fix) = &err.fix {
                    println!("    {} {}", "Fix:".cyan(), fix);
                }
            }
            println!();
        }

        if !self.warnings.is_empty() {
            println!("{}", "Warnings:".yellow().bold());
            for warn in &self.warnings {
                println!(
                    "  {} {}",
                    format!("[{}]", warn.code).yellow(),
                    warn.message.yellow()
                );
                println!("    {}", warn.suggestion.dimmed());
            }
            println!();
        }

        if !self.risk_factors.is_empty() {
            println!(
                "{}",
                format!("Risk Score: {}/100", self.risk_score)
                    .magenta()
                    .bold()
            );
            for factor in &self.risk_factors {
                let color = match factor.severity {
                    "high" => factor.description.red(),
                    "medium" => factor.description.yellow(),
                    _ => factor.description.dimmed(),
                };
                println!("  [{}] {}", factor.severity.to_uppercase(), color);
            }
        }
    }
}

pub fn validate_agent_file(file: &AgentFile, strict: bool) -> ValidationReport {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let mut risk_factors = Vec::new();
    let mut risk_score: u32 = 0;

    // Rule 1: Must have purpose
    let purpose_str = get_purpose_string(&file.purpose);
    if purpose_str.is_none() || purpose_str.unwrap().trim().is_empty() {
        errors.push(ValidationError {
            code: "MISSING_PURPOSE",
            message: "AGENT.agent must define a 'purpose' field".to_string(),
            fix: Some("Add a purpose field describing what this agent does".to_string()),
        });
    }

    // Rule 2: Must define permissions
    let forbidden_paths = get_forbidden_paths(&file.safety);
    let forbidden_actions = get_forbidden_actions(&file.safety);
    let require_confirmation = get_require_confirmation(&file.safety);

    if file.permissions.is_none() {
        errors.push(ValidationError {
            code: "MISSING_PERMISSIONS",
            message: "AGENT.agent must define a 'permissions' block".to_string(),
            fix: Some("Add permissions with read/write/execute arrays".to_string()),
        });
    } else if let Some(perms) = &file.permissions {
        if perms.write.is_none() || perms.write.as_ref().unwrap().is_empty() {
            warnings.push(ValidationWarning {
                code: "NO_WRITE_PERMISSIONS",
                message: "No write permissions defined; agent may be unable to modify files"
                    .to_string(),
                suggestion: "Define write permissions with appropriate path globs".to_string(),
            });
        }

        // Rule: Reject broad unsafe write permissions unless explicitly marked dangerous
        if let Some(write_perms) = &perms.write {
            for perm in write_perms {
                let broad_patterns = ["/**", "/*", "**/*", ".", "./"];
                let perm_str = perm.as_str();
                let is_broad = broad_patterns
                    .iter()
                    .any(|p| perm_str == *p || perm_str == format!("{}/", *p).as_str());
                if is_broad && !perm.contains("dangerous") {
                    errors.push(ValidationError {
                        code: "UNSAFE_WRITE_PERMISSION",
                        message: format!("Broad write permission '{}' detected without 'dangerous' marker", perm),
                        fix: Some(format!("Restrict write permission to specific paths, or add 'dangerous' marker if truly required: '{}", perm)),
                    });
                    risk_score += 25;
                    risk_factors.push(RiskFactor {
                        severity: "high",
                        description: format!("Unsafe write permission: {}", perm),
                    });
                }
            }

            // Rule: Forbidden paths must not be writable
            if let Some(forbidden) = &forbidden_paths {
                for fb in forbidden {
                    for permit in write_perms {
                        if fb.contains(permit) || permit.contains(fb) {
                            errors.push(ValidationError {
                                code: "FORBIDDEN_PATH_WRITE_OVERLAP",
                                message: format!(
                                    "Write permission '{}' overlaps with forbidden path '{}'",
                                    permit, fb
                                ),
                                fix: Some(format!(
                                    "Remove '{}' from write permissions or from forbidden_paths",
                                    permit
                                )),
                            });
                            risk_score += 15;
                            risk_factors.push(RiskFactor {
                                severity: "high",
                                description: format!(
                                    "Forbidden path write overlap: {} -> {}",
                                    permit, fb
                                ),
                            });
                        }
                    }
                }
            }
        }
    }

    // Rule 3: Must define forbidden paths/actions
    if file.safety.is_none() {
        errors.push(ValidationError {
            code: "MISSING_SAFETY",
            message: "AGENT.agent must define a 'safety' block".to_string(),
            fix: Some("Add safety with forbidden_paths and forbidden_actions".to_string()),
        });
    } else {
        if forbidden_paths.is_none() || forbidden_paths.as_ref().unwrap().is_empty() {
            warnings.push(ValidationWarning {
                code: "MISSING_FORBIDDEN_PATHS",
                message: "No forbidden_paths defined in safety block".to_string(),
                suggestion: "Add forbidden_paths to prevent agent from accessing sensitive files"
                    .to_string(),
            });
            if strict {
                errors.push(ValidationError {
                    code: "STRICT_MISSING_FORBIDDEN_PATHS",
                    message: "Strict mode requires forbidden_paths in safety block".to_string(),
                    fix: None,
                });
            }
        }

        if forbidden_actions.is_none() || forbidden_actions.as_ref().unwrap().is_empty() {
            warnings.push(ValidationWarning {
                code: "MISSING_FORBIDDEN_ACTIONS",
                message: "No forbidden_actions defined in safety block".to_string(),
                suggestion: "Add forbidden_actions (e.g., rm -rf, git push --force)".to_string(),
            });
            if strict {
                errors.push(ValidationError {
                    code: "STRICT_MISSING_FORBIDDEN_ACTIONS",
                    message: "Strict mode requires forbidden_actions in safety block".to_string(),
                    fix: None,
                });
            }
        }

        // Rule: Warn if destructive command has no confirmation requirement
        if let Some(actions) = &forbidden_actions {
            let destructive_patterns = [
                "rm", "mv", "git push", "force", "delete", "drop", "truncate",
            ];
            for action in actions {
                let lower = action.to_lowercase();
                let is_destructive = destructive_patterns.iter().any(|p| lower.contains(p));
                if is_destructive {
                    let has_confirmation = require_confirmation
                        .as_ref()
                        .map(|c| c.iter().any(|rc| lower.contains(&rc.to_lowercase())))
                        .unwrap_or(false);
                    if !has_confirmation {
                        warnings.push(ValidationWarning {
                            code: "DESTRUCTIVE_NO_CONFIRMATION",
                            message: format!("Destructive action '{}' is forbidden but no confirmation requirement is enforced", action),
                            suggestion: "Add matching pattern to require_confirmation to prevent accidental execution".to_string(),
                        });
                        risk_score += 5;
                    }
                }
            }
        }
    }

    // Rule 4: Must define at least one validation command
    if file.validation.is_none() || file.validation.as_ref().unwrap().is_empty() {
        errors.push(ValidationError {
            code: "MISSING_VALIDATION",
            message: "AGENT.agent must define at least one validation command".to_string(),
            fix: Some("Add a validation command like 'npm run lint' or 'cargo test'".to_string()),
        });
    }

    // Rule 5: Must define success criteria (via tasks, workflows, or top-level success_criteria)
    let has_success = file
        .success_criteria
        .as_ref()
        .map(|s| !s.is_empty())
        .unwrap_or(false)
        || file
            .tasks
            .as_ref()
            .map(|t| t.iter().any(|task| task.success.is_some()))
            .unwrap_or(false)
        || file
            .workflows
            .as_ref()
            .map(|w| {
                w.iter()
                    .any(|wf| wf.steps.iter().any(|s| s.success.is_some()))
            })
            .unwrap_or(false);
    if !has_success {
        warnings.push(ValidationWarning {
            code: "MISSING_SUCCESS_CRITERIA",
            message: "No success criteria defined in tasks or workflows".to_string(),
            suggestion: "Define success criteria so the agent knows when tasks are complete"
                .to_string(),
        });
        if strict {
            errors.push(ValidationError {
                code: "STRICT_MISSING_SUCCESS_CRITERIA",
                message: "Strict mode requires success criteria".to_string(),
                fix: None,
            });
        }
    }

    // Rule 6: Must define output requirements
    if file.output.is_none() {
        warnings.push(ValidationWarning {
            code: "MISSING_OUTPUT",
            message: "No output requirements defined; agent output format is unspecified"
                .to_string(),
            suggestion: "Add output block with format and required_sections".to_string(),
        });
        if strict {
            errors.push(ValidationError {
                code: "STRICT_MISSING_OUTPUT",
                message: "Strict mode requires output requirements".to_string(),
                fix: None,
            });
        }
    }

    // Rule 7: Warn if secrets policy is missing
    if !has_secrets_policy(&file.safety) {
        warnings.push(ValidationWarning {
            code: "MISSING_SECRETS_POLICY",
            message: "No secrets policy defined in safety block".to_string(),
            suggestion: "Add policy describing how secrets are handled (env vars, vault, etc)"
                .to_string(),
        });
        risk_score += 10;
        risk_factors.push(RiskFactor {
            severity: "medium",
            description: "Secrets handling policy missing".to_string(),
        });
    }

    // Risk score capping
    if risk_score > 100 {
        risk_score = 100;
    }

    let valid = errors.is_empty();
    ValidationReport {
        valid,
        errors,
        warnings,
        risk_score,
        risk_factors,
    }
}

#[derive(Debug)]
pub struct SkillValidationReport {
    pub valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
}

impl SkillValidationReport {
    pub fn print(&self) {
        println!("\n{}", "══ Skill Validation Report ══".cyan().bold());
        println!();

        if self.valid {
            println!("{}", "✔ VALID".green().bold());
        } else {
            println!("{}", "✘ INVALID".red().bold());
        }
        println!();

        if !self.errors.is_empty() {
            println!("{}", "Errors:".red().bold());
            for err in &self.errors {
                println!(
                    "  {} {}",
                    format!("[{}]", err.code).red(),
                    err.message.red()
                );
                if let Some(fix) = &err.fix {
                    println!("    {} {}", "Fix:".cyan(), fix);
                }
            }
            println!();
        }

        if !self.warnings.is_empty() {
            println!("{}", "Warnings:".yellow().bold());
            for warn in &self.warnings {
                println!(
                    "  {} {}",
                    format!("[{}]", warn.code).yellow(),
                    warn.message.yellow()
                );
                println!("    {}", warn.suggestion.dimmed());
            }
            println!();
        }
    }
}

pub fn validate_skill_file(skill: &SkillFile) -> SkillValidationReport {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    if skill.skill.trim().is_empty() {
        errors.push(ValidationError {
            code: "MISSING_SKILL_NAME",
            message: ".skill file must define a 'skill' field".to_string(),
            fix: Some("Add a skill field with the capability name".to_string()),
        });
    }

    if skill.version.trim().is_empty() {
        errors.push(ValidationError {
            code: "MISSING_VERSION",
            message: ".skill file must define a 'version' field".to_string(),
            fix: Some("Add a semantic version (e.g. 1.0.0)".to_string()),
        });
    }

    if skill.description.trim().is_empty() {
        warnings.push(ValidationWarning {
            code: "MISSING_DESCRIPTION",
            message: ".skill file should have a description".to_string(),
            suggestion: "Add a description of what this skill provides".to_string(),
        });
    }

    if skill.actions.is_none() || skill.actions.as_ref().unwrap().is_empty() {
        errors.push(ValidationError {
            code: "MISSING_ACTIONS",
            message: ".skill file must define at least one action".to_string(),
            fix: Some("Add an actions array describing what the skill does".to_string()),
        });
    }

    if skill.success.is_none() || skill.success.as_ref().unwrap().trim().is_empty() {
        errors.push(ValidationError {
            code: "MISSING_SUCCESS",
            message: ".skill file must define success criteria".to_string(),
            fix: Some("Add a success field describing when the skill succeeds".to_string()),
        });
    }

    if skill.output.is_none() || skill.output.as_ref().unwrap().trim().is_empty() {
        errors.push(ValidationError {
            code: "MISSING_OUTPUT",
            message: ".skill file must define output requirements".to_string(),
            fix: Some("Add an output field describing expected output format".to_string()),
        });
    }

    SkillValidationReport {
        valid: errors.is_empty(),
        errors,
        warnings,
    }
}
