use crate::library::Communicator;
use crate::library::data_type::Point;
use crate::renderer;




pub struct Orchestrator {
    //loop
        halt: bool,
        tick: usize,
        max_tick: usize,
        heed_max_tick: bool,
        next_revolution_time: std::time::Instant,
        revolution_interval: std::time::Duration,

    //renderer
        renderer__thread_handle: Option<std::thread::JoinHandle<()>>,
        renderer__communicator: Communicator<renderer::library::MessageFromRendererToOrchestrator, renderer::library::MessageFromOrchestratorToRenderer>,

    //human interface
        most_recent_mouse_position: Point,
        last_mouse_down_time: Option<std::time::Instant>,
        last_mouse_click_time: Option<std::time::Instant>,
        double_click_maximum_interval_duration: std::time::Duration,
        cursor_just_entered: bool,
        pressed_modifier_keys: winit::event::ModifiersState,
}

mod fragments;