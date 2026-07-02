use crate::syntax::diagnostics::Diagnostic;
use crate::syntax::lexer::{Lexer, Token, TokenWithPos};
use anyhow::{Result, anyhow};

pub fn parse_agent(content: &str) -> Result<crate::syntax::ast::AgentAst> {
    let mut lexer = Lexer::new(content);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    parser.parse_agent()
}

pub fn parse_skill(content: &str) -> Result<crate::syntax::ast::SkillAst> {
    let mut lexer = Lexer::new(content);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    parser.parse_skill()
}

struct Parser {
    tokens: Vec<TokenWithPos>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<TokenWithPos>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> Option<&TokenWithPos> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Result<TokenWithPos> {
        if self.pos < self.tokens.len() {
            let token = self.tokens[self.pos].clone();
            self.pos += 1;
            Ok(token)
        } else {
            Err(anyhow!("Unexpected end of input"))
        }
    }

    fn expect(&mut self, expected: Token) -> Result<TokenWithPos> {
        let token = self.advance()?;
        if token.token != expected {
            let diag = Diagnostic::new(
                format!("Expected {:?}, got {:?}", expected, token.token),
                token.line,
                token.column,
            )
            .with_suggestion(
                "Check the syntax for this section and ensure all blocks are properly closed.",
            );
            return Err(anyhow!(
                "{}",
                crate::syntax::diagnostics::format_diagnostic(&diag)
            ));
        }
        Ok(token)
    }

    fn parse_agent(&mut self) -> Result<crate::syntax::ast::AgentAst> {
        let _ = self.expect(Token::Agent)?;
        let name_token = self.advance()?;
        let agent = match name_token.token {
            Token::String(s) => s,
            _ => return Err(anyhow!("Expected agent name string")),
        };
        self.expect(Token::LBrace)?;

        let mut ast = crate::syntax::ast::AgentAst {
            agent,
            version: "0.4.0".to_string(),
            ..Default::default()
        };

        while self.current().map(|t| &t.token) != Some(&Token::RBrace) {
            self.parse_agent_section(&mut ast)?;
        }

        Ok(ast)
    }

    fn parse_agent_section(&mut self, ast: &mut crate::syntax::ast::AgentAst) -> Result<()> {
        let token = self.advance()?;
        match token.token {
            Token::Identifier(ref ident) if ident == "version" => {
                let t = self.advance()?;
                ast.version = match t.token {
                    Token::String(s) => s,
                    Token::Number(n) => n.to_string(),
                    _ => return Err(anyhow!("Expected version string")),
                };
            }
            Token::Identifier(ref ident) if ident == "contract_version" => {
                let t = self.advance()?;
                ast.contract_version = match t.token {
                    Token::Number(n) => Some(n),
                    _ => return Err(anyhow!("Expected contract_version number")),
                };
            }
            Token::Identifier(ref ident) if ident == "description" => {
                let t = self.advance()?;
                ast.description = Some(match t.token {
                    Token::String(s) => s,
                    _ => return Err(anyhow!("Expected description string")),
                });
            }
            Token::Identifier(ref ident) if ident == "purpose" => {
                self.expect(Token::LBrace)?;
                ast.purpose = Some(crate::syntax::ast::PurposeAst::default());
                while self.current().map(|t| &t.token) != Some(&Token::RBrace) {
                    self.parse_purpose_field(ast.purpose.as_mut().unwrap())?;
                }
                let _ = self.advance();
            }
            Token::Identifier(ref ident) if ident == "context" => {
                self.expect(Token::LBrace)?;
                ast.context = Some(crate::syntax::ast::ContextAst::default());
                while self.current().map(|t| &t.token) != Some(&Token::RBrace) {
                    self.parse_context_field(ast.context.as_mut().unwrap())?;
                }
                let _ = self.advance();
            }
            Token::Identifier(ref ident) if ident == "permissions" => {
                self.expect(Token::LBrace)?;
                ast.permissions = Some(crate::syntax::ast::PermissionsAst::default());
                while self.current().map(|t| &t.token) != Some(&Token::RBrace) {
                    self.parse_permissions_field(ast.permissions.as_mut().unwrap())?;
                }
                let _ = self.advance();
            }
            Token::Identifier(ref ident) if ident == "validation" => {
                self.expect(Token::LBrace)?;
                ast.validation = Some(crate::syntax::ast::ValidationAst::default());
                while self.current().map(|t| &t.token) != Some(&Token::RBrace) {
                    self.parse_validation_field(ast.validation.as_mut().unwrap())?;
                }
                let _ = self.advance();
            }
            Token::Identifier(ref ident) if ident == "safety" => {
                self.expect(Token::LBrace)?;
                ast.safety = Some(crate::syntax::ast::SafetyAst::default());
                while self.current().map(|t| &t.token) != Some(&Token::RBrace) {
                    self.parse_safety_field(ast.safety.as_mut().unwrap())?;
                }
                let _ = self.advance();
            }
            Token::Identifier(ref ident) if ident == "diff_policy" => {
                self.expect(Token::LBrace)?;
                ast.diff_policy = Some(crate::syntax::ast::DiffPolicyAst::default());
                while self.current().map(|t| &t.token) != Some(&Token::RBrace) {
                    self.parse_diff_policy_field(ast.diff_policy.as_mut().unwrap())?;
                }
                let _ = self.advance();
            }
            Token::Identifier(ref ident) if ident == "output" => {
                self.expect(Token::LBrace)?;
                ast.output = Some(crate::syntax::ast::OutputAst::default());
                while self.current().map(|t| &t.token) != Some(&Token::RBrace) {
                    let t = self.advance()?;
                    if let Token::Identifier(kw) = &t.token
                        && (kw == "required" || kw == "final_report")
                    {
                        let has_colon = self.current().map(|t| &t.token) == Some(&Token::Colon);
                        if has_colon {
                            let _ = self.advance();
                        }
                        self.expect(Token::LBracket)?;
                        while self.current().map(|t| &t.token) != Some(&Token::RBracket) {
                            let str_token = self.advance()?;
                            if let Token::String(s) = str_token.token
                                && let Some(ref mut o) = ast.output
                            {
                                o.final_report.push(s);
                            }
                            if self.current().map(|t| &t.token) == Some(&Token::Comma) {
                                let _ = self.advance();
                            }
                        }
                        let _ = self.advance();
                    }
                    if self.current().map(|t| &t.token) == Some(&Token::Comma) {
                        let _ = self.advance();
                    }
                }
                let _ = self.advance();
            }
            Token::Identifier(_) => {
                let _ = self.advance();
            }
            _ => {}
        }
        Ok(())
    }

    fn parse_purpose_field(&mut self, purpose: &mut crate::syntax::ast::PurposeAst) -> Result<()> {
        let token = self.advance()?;
        match token.token {
            Token::Identifier(ref ident) if ident == "human_goal" => {
                let t = self.advance()?;
                purpose.human_goal = match t.token {
                    Token::String(s) => s,
                    _ => return Err(anyhow!("Expected string")),
                };
            }
            Token::Identifier(ref ident) if ident == "agent_goal" => {
                let t = self.advance()?;
                purpose.agent_goal = match t.token {
                    Token::String(s) => s,
                    _ => return Err(anyhow!("Expected string")),
                };
            }
            Token::Identifier(ref ident) if ident == "non_goals" => {
                self.expect(Token::LBracket)?;
                while self.current().map(|t| &t.token) != Some(&Token::RBracket) {
                    let t = self.advance()?;
                    purpose.non_goals.push(match t.token {
                        Token::String(s) => s,
                        _ => return Err(anyhow!("Expected string in non_goals")),
                    });
                    if self.current().map(|t| &t.token) == Some(&Token::Comma) {
                        let _ = self.advance();
                    }
                }
                let _ = self.advance();
            }
            _ => {}
        }
        Ok(())
    }

    fn parse_context_field(&mut self, context: &mut crate::syntax::ast::ContextAst) -> Result<()> {
        let token = self.advance()?;
        match token.token {
            Token::Identifier(ref ident) if ident == "stack" => {
                if self.current().map(|t| &t.token) == Some(&Token::Colon) {
                    let _ = self.advance();
                }
                let is_bracket = self.current().map(|t| &t.token) == Some(&Token::LBracket);
                if is_bracket {
                    self.expect(Token::LBracket)?;
                    while self.current().map(|t| &t.token) != Some(&Token::RBracket) {
                        let t = self.advance()?;
                        context.stack.push(match t.token {
                            Token::String(s) => s,
                            _ => return Err(anyhow!("Expected string in stack")),
                        });
                        if self.current().map(|t| &t.token) == Some(&Token::Comma) {
                            let _ = self.advance();
                        }
                    }
                    let _ = self.advance();
                } else {
                    let t = self.advance()?;
                    context.stack.push(match t.token {
                        Token::String(s) => s,
                        _ => return Err(anyhow!("Expected string in stack")),
                    });
                }
            }
            Token::Identifier(ref ident) if ident == "important_files" => {
                if self.current().map(|t| &t.token) == Some(&Token::Colon) {
                    let _ = self.advance();
                }
                let is_bracket = self.current().map(|t| &t.token) == Some(&Token::LBracket);
                if is_bracket {
                    self.expect(Token::LBracket)?;
                    while self.current().map(|t| &t.token) != Some(&Token::RBracket) {
                        let t = self.advance()?;
                        context.important_files.push(match t.token {
                            Token::String(s) => s,
                            _ => return Err(anyhow!("Expected string in important_files")),
                        });
                        if self.current().map(|t| &t.token) == Some(&Token::Comma) {
                            let _ = self.advance();
                        }
                    }
                    let _ = self.advance();
                } else {
                    let t = self.advance()?;
                    context.important_files.push(match t.token {
                        Token::String(s) => s,
                        _ => return Err(anyhow!("Expected string in important_files")),
                    });
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn parse_permissions_field(
        &mut self,
        permissions: &mut crate::syntax::ast::PermissionsAst,
    ) -> Result<()> {
        let token = self.advance()?;
        match token.token {
            Token::Identifier(ref ident) if ident == "read" => {
                let has_colon = self.current().map(|t| &t.token) == Some(&Token::Colon);
                if has_colon {
                    let _ = self.advance();
                }
                self.expect(Token::LBracket)?;
                while self.current().map(|t| &t.token) != Some(&Token::RBracket) {
                    let t = self.advance()?;
                    permissions.read.push(match t.token {
                        Token::String(s) => s,
                        _ => return Err(anyhow!("Expected string")),
                    });
                    if self.current().map(|t| &t.token) == Some(&Token::Comma) {
                        let _ = self.advance();
                    }
                }
                let _ = self.advance();
            }
            Token::Identifier(ref ident) if ident == "write" => {
                let has_colon = self.current().map(|t| &t.token) == Some(&Token::Colon);
                if has_colon {
                    let _ = self.advance();
                }
                self.expect(Token::LBracket)?;
                while self.current().map(|t| &t.token) != Some(&Token::RBracket) {
                    let t = self.advance()?;
                    permissions.write.push(match t.token {
                        Token::String(s) => s,
                        _ => return Err(anyhow!("Expected string")),
                    });
                    if self.current().map(|t| &t.token) == Some(&Token::Comma) {
                        let _ = self.advance();
                    }
                }
                let _ = self.advance();
            }
            Token::Identifier(ref ident) if ident == "execute" => {
                let has_colon = self.current().map(|t| &t.token) == Some(&Token::Colon);
                if has_colon {
                    let _ = self.advance();
                }
                self.expect(Token::LBracket)?;
                while self.current().map(|t| &t.token) != Some(&Token::RBracket) {
                    let t = self.advance()?;
                    permissions.execute.push(match t.token {
                        Token::String(s) => s,
                        _ => return Err(anyhow!("Expected string")),
                    });
                    if self.current().map(|t| &t.token) == Some(&Token::Comma) {
                        let _ = self.advance();
                    }
                }
                let _ = self.advance();
            }
            Token::Identifier(ref ident) if ident == "forbidden" => {
                let has_colon = self.current().map(|t| &t.token) == Some(&Token::Colon);
                if has_colon {
                    let _ = self.advance();
                }
                self.expect(Token::LBracket)?;
                while self.current().map(|t| &t.token) != Some(&Token::RBracket) {
                    let t = self.advance()?;
                    permissions.forbidden.push(match t.token {
                        Token::String(s) => s,
                        _ => return Err(anyhow!("Expected string")),
                    });
                    if self.current().map(|t| &t.token) == Some(&Token::Comma) {
                        let _ = self.advance();
                    }
                }
                let _ = self.advance();
            }
            _ => {}
        }
        Ok(())
    }

    fn parse_validation_field(
        &mut self,
        validation: &mut crate::syntax::ast::ValidationAst,
    ) -> Result<()> {
        let token = self.advance()?;
        match token.token {
            Token::Identifier(ref ident) if ident == "command" => {
                let has_colon = self.current().map(|t| &t.token) == Some(&Token::Colon);
                if has_colon {
                    let _ = self.advance();
                }
                let t = self.advance()?;
                validation.commands.push(match t.token {
                    Token::String(s) => s,
                    _ => return Err(anyhow!("Expected string")),
                });
            }
            Token::Identifier(ref ident) if ident == "success" => {
                let has_colon = self.current().map(|t| &t.token) == Some(&Token::Colon);
                if has_colon {
                    let _ = self.advance();
                }
                let t = self.advance()?;
                validation.success.push(match t.token {
                    Token::String(s) => s,
                    _ => return Err(anyhow!("Expected string")),
                });
            }
            _ => {}
        }
        Ok(())
    }

    fn parse_safety_field(&mut self, safety: &mut crate::syntax::ast::SafetyAst) -> Result<()> {
        let token = self.advance()?;
        match token.token {
            Token::Identifier(ref ident) if ident == "forbidden_actions" => {
                let has_colon = self.current().map(|t| &t.token) == Some(&Token::Colon);
                if has_colon {
                    let _ = self.advance();
                }
                self.expect(Token::LBracket)?;
                while self.current().map(|t| &t.token) != Some(&Token::RBracket) {
                    let t = self.advance()?;
                    safety.forbidden_actions.push(match t.token {
                        Token::String(s) => s,
                        _ => return Err(anyhow!("Expected string")),
                    });
                    if self.current().map(|t| &t.token) == Some(&Token::Comma) {
                        let _ = self.advance();
                    }
                }
                let _ = self.advance();
            }
            Token::Identifier(ref ident) if ident == "forbidden_paths" => {
                let has_colon = self.current().map(|t| &t.token) == Some(&Token::Colon);
                if has_colon {
                    let _ = self.advance();
                }
                self.expect(Token::LBracket)?;
                while self.current().map(|t| &t.token) != Some(&Token::RBracket) {
                    let t = self.advance()?;
                    safety.forbidden_paths.push(match t.token {
                        Token::String(s) => s,
                        _ => return Err(anyhow!("Expected string")),
                    });
                    if self.current().map(|t| &t.token) == Some(&Token::Comma) {
                        let _ = self.advance();
                    }
                }
                let _ = self.advance();
            }
            Token::Identifier(ref ident) if ident == "require_approval" => {
                let has_colon = self.current().map(|t| &t.token) == Some(&Token::Colon);
                if has_colon {
                    let _ = self.advance();
                }
                self.expect(Token::LBracket)?;
                while self.current().map(|t| &t.token) != Some(&Token::RBracket) {
                    let t = self.advance()?;
                    safety.require_approval.push(match t.token {
                        Token::String(s) => s,
                        _ => return Err(anyhow!("Expected string")),
                    });
                    if self.current().map(|t| &t.token) == Some(&Token::Comma) {
                        let _ = self.advance();
                    }
                }
                let _ = self.advance();
            }
            Token::Identifier(ref ident) if ident == "policy" => {
                let has_colon = self.current().map(|t| &t.token) == Some(&Token::Colon);
                if has_colon {
                    let _ = self.advance();
                }
                let t = self.advance()?;
                safety.rules.push(match t.token {
                    Token::String(s) => s,
                    _ => return Err(anyhow!("Expected string for policy")),
                });
            }
            Token::Identifier(ref ident) if ident == "rule" => {
                let t = self.advance()?;
                safety.rules.push(match t.token {
                    Token::String(s) => s,
                    _ => return Err(anyhow!("Expected string")),
                });
            }
            _ => {}
        }
        Ok(())
    }

    fn parse_diff_policy_field(
        &mut self,
        diff_policy: &mut crate::syntax::ast::DiffPolicyAst,
    ) -> Result<()> {
        let token = self.advance()?;
        match token.token {
            Token::Identifier(ref ident) if ident == "strict_ci" => {
                let t = self.advance()?;
                diff_policy.strict_ci = match t.token {
                    Token::Bool(b) => b,
                    _ => return Err(anyhow!("Expected boolean for strict_ci")),
                };
            }
            Token::Identifier(ref ident) if ident == "fail_at_risk_score" => {
                let t = self.advance()?;
                diff_policy.fail_at_risk_score = match t.token {
                    Token::Number(n) => n,
                    _ => return Err(anyhow!("Expected number for fail_at_risk_score")),
                };
            }
            Token::Identifier(ref ident) if ident == "require_tests_for_src_changes" => {
                let t = self.advance()?;
                diff_policy.require_tests_for_src_changes = match t.token {
                    Token::Bool(b) => b,
                    _ => {
                        return Err(anyhow!(
                            "Expected boolean for require_tests_for_src_changes"
                        ));
                    }
                };
            }
            Token::Identifier(ref ident) if ident == "watched_path" => {
                let path_token = self.advance()?;
                let path = match path_token.token {
                    Token::String(s) => s,
                    _ => return Err(anyhow!("Expected path string after watched_path")),
                };
                self.expect(Token::LBrace)?;
                let mut watched = crate::syntax::ast::WatchedPath {
                    path,
                    ..Default::default()
                };
                while self.current().map(|t| &t.token) != Some(&Token::RBrace) {
                    let t = self.advance()?;
                    if let Token::Identifier(id) = &t.token {
                        if id == "risk" {
                            let risk_t = self.advance()?;
                            watched.risk = match risk_t.token {
                                Token::Number(n) => n,
                                _ => return Err(anyhow!("Expected number for risk")),
                            };
                        } else if id == "requires" {
                            self.expect(Token::LBracket)?;
                            while self.current().map(|t| &t.token) != Some(&Token::RBracket) {
                                let req = self.advance()?;
                                watched.requires.push(match req.token {
                                    Token::String(s) => s,
                                    _ => return Err(anyhow!("Expected string")),
                                });
                                if self.current().map(|t| &t.token) == Some(&Token::Comma) {
                                    let _ = self.advance();
                                }
                            }
                            let _ = self.advance();
                        }
                    }
                    if self.current().map(|t| &t.token) == Some(&Token::Comma) {
                        let _ = self.advance();
                    }
                }
                let _ = self.advance();
                diff_policy.watched_paths.push(watched);
            }
            _ => {}
        }
        Ok(())
    }

    fn parse_skill(&mut self) -> Result<crate::syntax::ast::SkillAst> {
        let _ = self.expect(Token::Skill)?;
        let name_token = self.advance()?;
        let skill = match name_token.token {
            Token::String(s) => s,
            _ => return Err(anyhow!("Expected skill name string")),
        };
        self.expect(Token::LBrace)?;

        let mut ast = crate::syntax::ast::SkillAst {
            skill,
            version: "0.4.0".to_string(),
            description: String::new(),
            ..Default::default()
        };

        while self.current().map(|t| &t.token) != Some(&Token::RBrace) {
            self.parse_skill_section(&mut ast)?;
        }

        Ok(ast)
    }

    fn parse_skill_section(&mut self, ast: &mut crate::syntax::ast::SkillAst) -> Result<()> {
        let token = self.advance()?;
        match token.token {
            Token::Identifier(ref ident) if ident == "version" => {
                let t = self.advance()?;
                ast.version = match t.token {
                    Token::String(s) => s,
                    Token::Number(n) => n.to_string(),
                    _ => return Err(anyhow!("Expected version")),
                };
            }
            Token::Identifier(ref ident) if ident == "description" => {
                let t = self.advance()?;
                ast.description = match t.token {
                    Token::String(s) => s,
                    _ => return Err(anyhow!("Expected description string")),
                };
            }
            Token::Identifier(ref ident) if ident == "applies_to" => {
                self.expect(Token::LBrace)?;
                ast.applies_to = Some(crate::syntax::ast::AppliesToAst::default());
                while self.current().map(|t| &t.token) != Some(&Token::RBrace) {
                    self.parse_applies_to_field(ast.applies_to.as_mut().unwrap())?;
                }
                let _ = self.advance();
            }
            Token::Identifier(ref ident) if ident == "rules" => {
                self.expect(Token::LBrace)?;
                while self.current().map(|t| &t.token) != Some(&Token::RBrace) {
                    let t = self.advance()?;
                    if let Token::Identifier(ident) = &t.token {
                        if ident == "rule" {
                            let str_token = self.advance()?;
                            ast.rules.push(match str_token.token {
                                Token::String(s) => s,
                                _ => return Err(anyhow!("Expected rule string after 'rule'")),
                            });
                        } else {
                            return Err(anyhow!("Expected 'rule' keyword"));
                        }
                    } else {
                        return Err(anyhow!("Expected 'rule' keyword"));
                    }
                    if self.current().map(|t| &t.token) == Some(&Token::Comma) {
                        let _ = self.advance();
                    }
                }
                let _ = self.advance();
            }
            Token::Identifier(ref ident) if ident == "requires_validation" => {
                self.expect(Token::LBracket)?;
                while self.current().map(|t| &t.token) != Some(&Token::RBracket) {
                    let t = self.advance()?;
                    ast.requires_validation.push(match t.token {
                        Token::String(s) => s,
                        _ => return Err(anyhow!("Expected string")),
                    });
                    if self.current().map(|t| &t.token) == Some(&Token::Comma) {
                        let _ = self.advance();
                    }
                }
                let _ = self.advance();
            }
            Token::Identifier(ref ident) if ident == "success" => {
                self.expect(Token::LBrace)?;
                while self.current().map(|t| &t.token) != Some(&Token::RBrace) {
                    let t = self.advance()?;
                    if let Token::Identifier(ref kw) = t.token {
                        if kw == "item" {
                            let str_token = self.advance()?;
                            ast.success.items.push(match str_token.token {
                                Token::String(s) => s,
                                _ => {
                                    return Err(anyhow!(
                                        "Expected success item string after 'item'"
                                    ));
                                }
                            });
                        }
                    } else {
                        ast.success.items.push(match t.token {
                            Token::String(s) => s,
                            _ => return Err(anyhow!("Expected success item string")),
                        });
                    }
                    if self.current().map(|t| &t.token) == Some(&Token::Comma) {
                        let _ = self.advance();
                    }
                }
                let _ = self.advance();
            }
            Token::Identifier(ref ident) if ident == "output" => {
                self.expect(Token::LBrace)?;
                while self.current().map(|t| &t.token) != Some(&Token::RBrace) {
                    let t = self.advance()?;
                    if let Token::Identifier(kw) = &t.token
                        && (kw == "required" || kw == "final_report")
                    {
                        let has_colon = self.current().map(|t| &t.token) == Some(&Token::Colon);
                        if has_colon {
                            let _ = self.advance();
                        }
                        self.expect(Token::LBracket)?;
                        while self.current().map(|t| &t.token) != Some(&Token::RBracket) {
                            let str_token = self.advance()?;
                            if let Token::String(s) = str_token.token {
                                ast.output.final_report.push(s);
                            }
                            if self.current().map(|t| &t.token) == Some(&Token::Comma) {
                                let _ = self.advance();
                            }
                        }
                        let _ = self.advance();
                    }
                    if self.current().map(|t| &t.token) == Some(&Token::Comma) {
                        let _ = self.advance();
                    }
                }
                let _ = self.advance();
            }
            _ => {}
        }
        Ok(())
    }

    fn parse_applies_to_field(
        &mut self,
        applies_to: &mut crate::syntax::ast::AppliesToAst,
    ) -> Result<()> {
        let token = self.advance()?;
        match token.token {
            Token::Identifier(ref ident) if ident == "paths" => {
                self.expect(Token::LBracket)?;
                while self.current().map(|t| &t.token) != Some(&Token::RBracket) {
                    let t = self.advance()?;
                    applies_to.paths.push(match t.token {
                        Token::String(s) => s,
                        _ => return Err(anyhow!("Expected string")),
                    });
                    if self.current().map(|t| &t.token) == Some(&Token::Comma) {
                        let _ = self.advance();
                    }
                }
                let _ = self.advance();
            }
            Token::Identifier(ref ident) if ident == "stacks" => {
                self.expect(Token::LBracket)?;
                while self.current().map(|t| &t.token) != Some(&Token::RBracket) {
                    let t = self.advance()?;
                    applies_to.stacks.push(match t.token {
                        Token::String(s) => s,
                        _ => return Err(anyhow!("Expected string")),
                    });
                    if self.current().map(|t| &t.token) == Some(&Token::Comma) {
                        let _ = self.advance();
                    }
                }
                let _ = self.advance();
            }
            Token::Identifier(ref ident) if ident == "keywords" => {
                self.expect(Token::LBracket)?;
                while self.current().map(|t| &t.token) != Some(&Token::RBracket) {
                    let t = self.advance()?;
                    applies_to.keywords.push(match t.token {
                        Token::String(s) => s,
                        _ => return Err(anyhow!("Expected string")),
                    });
                    if self.current().map(|t| &t.token) == Some(&Token::Comma) {
                        let _ = self.advance();
                    }
                }
                let _ = self.advance();
            }
            _ => {}
        }
        Ok(())
    }
}
