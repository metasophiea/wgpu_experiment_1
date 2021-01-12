#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]

mod library;
mod orchestrator;
mod renderer;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();

    let window:winit::window::Window = winit::window::WindowBuilder::new()
        .with_title("Experiemnt")
        .with_decorations(true)
        .with_resizable(!false)
        .with_transparent(true)
        .with_inner_size( winit::dpi::LogicalSize { height:400.0, width:500.0 } )
        .build(&event_loop).unwrap();

    let mut orchestrator = orchestrator::Orchestrator::new(
        &window
    );

    event_loop.run(move |event, _, control_flow| {
        orchestrator.window_event( &window, event, control_flow );
    });
}