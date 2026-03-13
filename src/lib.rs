use zed_extension_api as zed;

struct OpenSpecExtension;

impl zed::Extension for OpenSpecExtension {
    fn new() -> Self {
        Self
    }

    fn run_slash_command(
        &self,
        command: zed::SlashCommand,
        args: Vec<String>,
        _worktree: Option<&zed::Worktree>,
    ) -> Result<zed::SlashCommandOutput, String> {
        let command_name = command.name.as_str();
        let args_str = args.join(" ");

        // Validate required arguments for commands that need them
        if Self::command_requires_arg(command_name) && args.is_empty() {
            return Err(format!(
                "The /opsx {} command requires an argument. Please provide a change name or description.",
                command_name
            ));
        }

        let (text, label) = match command_name {
            "propose" => (
                if args_str.is_empty() {
                    "I want to propose a new change. Please help me brainstorm and create planning artifacts for this change.".to_string()
                } else {
                    format!("I want to propose a new change called '{}'. Please help me create comprehensive planning artifacts including requirements, design considerations, and implementation tasks.", args_str)
                },
                "OpenSpec: Propose Change",
            ),
            "explore" => (
                if args_str.is_empty() {
                    "I want to explore ideas and investigate problems. Please help me think through different approaches and considerations.".to_string()
                } else {
                    format!("I want to explore ideas around '{}'. Please help me investigate problems, consider different approaches, and think through potential solutions.", args_str)
                },
                "OpenSpec: Explore Ideas",
            ),
            "new" => (
                if args_str.is_empty() {
                    "I want to start a new change scaffold. Please help me create the initial structure and planning documents.".to_string()
                } else {
                    format!("I want to start a new change scaffold for '{}'. Please help me create the initial structure including change specification, tasks, and planning documents.", args_str)
                },
                "OpenSpec: New Change",
            ),
            "continue" => (
                if args_str.is_empty() {
                    "I want to create the next artifact in the dependency chain. Please help me identify what should come next and draft it.".to_string()
                } else {
                    format!("I want to create the next artifact in the dependency chain for '{}'. Please help me identify the next logical step and draft the appropriate artifact.", args_str)
                },
                "OpenSpec: Continue Change",
            ),
            "ff" => (
                if args_str.is_empty() {
                    "I want to fast-forward and create all planning artifacts at once. Please help me generate a complete set of planning documents.".to_string()
                } else {
                    format!("I want to fast-forward and create all planning artifacts for '{}'. Please help me generate a complete set including requirements, design, tasks, and validation criteria.", args_str)
                },
                "OpenSpec: Fast-Forward",
            ),
            "verify" => (
                if args_str.is_empty() {
                    "I want to validate that my implementation matches the artifacts. Please help me review the code against the specifications.".to_string()
                } else {
                    format!("I want to validate that my implementation for '{}' matches the planning artifacts. Please help me review the code against the specifications and identify any discrepancies.", args_str)
                },
                "OpenSpec: Verify Implementation",
            ),
            "sync" => (
                if args_str.is_empty() {
                    "I want to merge delta specs into main specs. Please help me consolidate the changes and update the main specifications.".to_string()
                } else {
                    format!("I want to merge delta specs into main specs for '{}'. Please help me consolidate the changes and update the main specifications accordingly.", args_str)
                },
                "OpenSpec: Sync Specs",
            ),
            "apply" => (
                if args_str.is_empty() {
                    "I want to implement tasks from the active change. Please help me write code that fulfills the specified tasks.".to_string()
                } else {
                    format!("I want to implement tasks from the active change '{}'. Please help me write code that fulfills the specified tasks and requirements.", args_str)
                },
                "OpenSpec: Apply Tasks",
            ),
            "archive" => (
                if args_str.is_empty() {
                    "I want to archive a completed change. Please help me document the final state and lessons learned.".to_string()
                } else {
                    format!("I want to archive the completed change '{}'. Please help me document the final state, outcomes, and lessons learned for future reference.", args_str)
                },
                "OpenSpec: Archive Change",
            ),
            "bulk-archive" => (
                "I want to archive multiple completed changes. Please help me identify completed changes and document them appropriately.".to_string(),
                "OpenSpec: Bulk Archive",
            ),
            "onboard" => (
                "I want to go through the OpenSpec guided tutorial. Please walk me through the OpenSpec workflow step by step, explaining each concept and how to use it effectively.".to_string(),
                "OpenSpec: Onboard Tutorial",
            ),
            _ => return Err(format!("Unknown slash command: {}", command_name)),
        };

        let range = 0..text.len();
        Ok(zed::SlashCommandOutput {
            text,
            sections: vec![zed::SlashCommandOutputSection {
                range: range.into(),
                label: label.to_string(),
            }],
        })
    }

    fn complete_slash_command_argument(
        &self,
        command: zed::SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<zed::SlashCommandArgumentCompletion>, String> {
        let command_name = command.name.as_str();

        match command_name {
            "propose" | "new" | "continue" | "ff" | "verify" | "sync" | "apply" | "archive" => {
                // Return common change names as suggestions
                Ok(vec![
                    zed::SlashCommandArgumentCompletion {
                        label: "add-new-feature".to_string(),
                        new_text: "add-new-feature".to_string(),
                        run_command: false,
                    },
                    zed::SlashCommandArgumentCompletion {
                        label: "fix-bug".to_string(),
                        new_text: "fix-bug".to_string(),
                        run_command: false,
                    },
                    zed::SlashCommandArgumentCompletion {
                        label: "update-dependencies".to_string(),
                        new_text: "update-dependencies".to_string(),
                        run_command: false,
                    },
                    zed::SlashCommandArgumentCompletion {
                        label: "refactor-module".to_string(),
                        new_text: "refactor-module".to_string(),
                        run_command: false,
                    },
                ])
            }
            _ => Ok(vec![]),
        }
    }
}

impl OpenSpecExtension {
    fn command_requires_arg(command: &str) -> bool {
        matches!(
            command,
            "new" | "continue" | "ff" | "verify" | "sync" | "apply" | "archive"
        )
    }
}

zed::register_extension!(OpenSpecExtension);
