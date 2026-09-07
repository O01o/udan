#[derive(Debug, Clone, PartialEq)]
pub enum State {
    MarkDown,
    Shell,
    Plots,
}

pub struct Block {
    pub state: State,
    pub text_lines: Vec<String>,
}

pub struct UDANAnalyzer {
    state: State,
    block_text_lines: Vec<String>,
    output: Vec<Block>,
}

impl UDANAnalyzer {
    pub fn new() -> Self {
        UDANAnalyzer {
            state: State::MarkDown,
            block_text_lines: Vec::new(),
            output: Vec::new(),
        }
    }

    fn push_output(&mut self) {
        if !self.block_text_lines.is_empty() {
            self.output.push(Block {
                state: self.state.clone(),
                text_lines: self.block_text_lines.clone(),
            });
        };
        self.block_text_lines.clear();
    }

    pub fn parse(&mut self, input: &str) -> &Vec<Block> {
        for line in input.lines() {
            match line {
                ":::markdown" => {
                    self.state = State::MarkDown;
                    self.push_output();
                }
                ":::shell" => {
                    self.state = State::Shell;
                    self.push_output();
                }
                ":::plots" => {
                    self.state = State::Plots;
                    self.push_output();
                }
                _ => {
                    self.block_text_lines.push(line.to_string());
                }
            }
        }
        &self.output
    }
}