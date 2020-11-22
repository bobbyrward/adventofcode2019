#[derive(Debug, Clone)]
pub enum ExecutionState {
    Continue,
    Halt,
    Jump(i64),
    AdjustRelative(i64),
}

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    Halt,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    AdjustRelative,
}
