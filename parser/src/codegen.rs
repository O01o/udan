use std::process::Command;

use crate::analyzer::{Block, State};
pub struct SvelteCodeGenerator {
    blocks: Vec<Block>,
    outputs: Vec<String>,
}

impl SvelteCodeGenerator {
    pub fn new(blocks: Vec<Block>) -> Self {
        SvelteCodeGenerator {
            blocks,
            outputs: Vec::new(),
        }
    }

    pub fn generate(&mut self) -> String {
        for block in &self.blocks {
            match block.state {
                State::MarkDown => {
                    self.outputs.push(format!("<div class=\"markdown\">"));
                    let command = Command::new("pandoc")
                        .arg("-f")
                        .arg("markdown")
                        .arg("-t")
                        .arg("html")
                        .stdin(std::process::Stdio::piped())
                        .stdout(std::process::Stdio::piped())
                        .spawn()
                        .expect("Failed to spawn pandoc process");
                    /*
                    {
                        let stdin = command.stdin.as_mut().expect("Failed to open stdin");
                        stdin
                            .write_all(block.text_lines.join("\n").as_bytes())
                            .expect("Failed to write to stdin");
                    }
                    */
                    let output = command.wait_with_output().expect("Failed to read pandoc output");
                    self.outputs.push(String::from_utf8_lossy(&output.stdout).to_string());
                }
                State::Shell => {
                    self.outputs.push(format!("<div class=\"shell\">"));
                    self.outputs.push(format!("<pre><code>"));
                    self.outputs.push(format!("{}", block.text_lines.join("\n")));
                    self.outputs.push(format!("</code></pre>"));
                }
                State::Plots => {
                    self.outputs.push(format!("<div class=\"plots\">"));
                    self.outputs.push(format!("{}", block.text_lines.join("\n")));
                }
            }
            self.outputs.push(format!("</div>"));
        }
        self.outputs.join("\n")
    }
}