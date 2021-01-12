use super::super::Orchestrator;

use crate::library::{
    Communicator,
    Communique,
};
use crate::library::data_type::Point;
use crate::renderer;




impl Orchestrator {
    pub fn new(
        window: &winit::window::Window
    ) -> Orchestrator {
        //renderer
            let (command_channel_sender, command_channel_receiver) = std::sync::mpsc::channel::<Communique<renderer::library::MessageFromOrchestratorToRenderer>>();
            let (message_channel_sender, message_channel_receiver) = std::sync::mpsc::channel::<Communique<renderer::library::MessageFromRendererToOrchestrator>>();

            let wgpu_setup_data = renderer::Renderer::wgpu_setup(&window);
            let renderer__handle = std::thread::spawn(move || {
                renderer::Renderer::new(
                    command_channel_receiver,
                    message_channel_sender,
                    wgpu_setup_data,
                ).ignition();
            });

        Orchestrator {
            //loop
                halt: false,
                tick: 0,
                max_tick: 100,
                heed_max_tick: false,
                next_revolution_time: std::time::Instant::now(),
                revolution_interval: std::time::Duration::from_millis(100),

            //renderer
                renderer__thread_handle: Some(renderer__handle),
                renderer__communicator: Communicator::new(message_channel_receiver, command_channel_sender),

            //human interface
                most_recent_mouse_position: Point::new(0.0, 0.0),
                last_mouse_down_time: None,
                last_mouse_click_time: None,
                double_click_maximum_interval_duration: std::time::Duration::from_secs(1),
                cursor_just_entered: false,
                pressed_modifier_keys: winit::event::ModifiersState::empty(),
        }
    }
}