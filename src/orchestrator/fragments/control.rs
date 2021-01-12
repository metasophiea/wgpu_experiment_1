use super::super::Orchestrator;

use crate::renderer;




impl Orchestrator {
    pub fn halt(&mut self) {
        println!("\n>> Orchestrator Halting");

        //renderer
            println!(">>> Halting > Renderer");
            self.renderer__communicator.send_message( renderer::library::MessageFromOrchestratorToRenderer::Halt ).ok();
            let mut tmp:Option<std::thread::JoinHandle<()>> = None;
            std::mem::swap(&mut tmp, &mut self.renderer__thread_handle);
            tmp.unwrap().join().unwrap(); //join and make thread wait until join is compelte
            println!(">>> Complete");

        //self
            self.halt = true;

        println!(">> Orchestrator Halt Complete");
    }
}