pub struct State {
    pub push_lambda_to_stack: bool,
    pub undefine_lambda: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            push_lambda_to_stack: false,
            undefine_lambda: false,
        }
    }

    pub fn snapshot(&self) -> Self {
        Self {
            push_lambda_to_stack: self.push_lambda_to_stack,
            undefine_lambda: self.undefine_lambda,
        }
    }

    pub fn restore(&mut self, snapshot: Self) {
        *self = snapshot;
    }
}
