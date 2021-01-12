#[derive(Debug)]
pub enum MessageFromOrchestratorToRenderer {
    Halt,

    Test(usize),
}

#[derive(Debug)]
pub enum MessageFromRendererToOrchestrator {
    Halted,
}