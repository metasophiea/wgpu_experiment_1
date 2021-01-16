use super::super::Orchestrator;

use crate::renderer;




impl Orchestrator {
    pub fn revolution(&mut self) {
        // println!("Orchestrator tick: {}", self.tick);

        //message collection
            //renderer
                // for item in self.renderer__communicator.collect_messages() {
                //     println!(" > Orchestrator renderer__communicator got: {:?}", item);
                // }

        //logic
            if self.tick == 1 {
                self.renderer__communicator.send_message( renderer::library::MessageFromOrchestratorToRenderer::Test(2) ).ok();
            }
    }
}