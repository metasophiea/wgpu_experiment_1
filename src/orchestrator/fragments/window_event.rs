use super::super::Orchestrator;




impl Orchestrator {
    pub fn window_event(
        &mut self,
        window: &winit::window::Window,
        event: winit::event::Event<()>,
        control_flow: &mut winit::event_loop::ControlFlow,
    ) {
        //check for halt
            if self.halt {
                println!("window exiting event > {:?}", event);
                *control_flow = winit::event_loop::ControlFlow::Exit;

                if event == winit::event::Event::LoopDestroyed {
                    println!("Orchestrator Exiting\n");
                }

                return;
            }

        //deal with window event
            match event {
                winit::event::Event::WindowEvent { ref event, window_id } if window_id == window.id() => {
                    match event {
                        //window event
                            winit::event::WindowEvent::CloseRequested => self.halt(),

                        //escape key
                            winit::event::WindowEvent::KeyboardInput { input, .. } => { //{ device_id, input, is_synthetic }
                                if let winit::event::KeyboardInput {
                                    state: winit::event::ElementState::Pressed, 
                                    virtual_keycode: Some(winit::event::VirtualKeyCode::Escape), 
                                    .. 
                                } = input {
                                    self.halt();
                                }
                            },

                        _ => {
                            // println!("Unhandled window event: {:?}", event);
                        },
                    }
                },
                _ => {
                    // println!("Unhandled non-window event: {:?}", event);
                },
            }

        //check if its time for a revolution
            if self.next_revolution_time.checked_duration_since( std::time::Instant::now() ).is_none() {
                self.next_revolution_time = std::time::Instant::now().checked_add( self.revolution_interval ).unwrap();


                self.revolution();
                self.tick+=1;

                if self.heed_max_tick && self.tick == self.max_tick {
                    self.halt();
                }
            }

            //tell winit event_loop to wait until the appropriate time to run this code again
            //though it will run early if an event is received, in which case this code will
            //remind the winit event_loop to not run until the appropriate time
            *control_flow = winit::event_loop::ControlFlow::WaitUntil(self.next_revolution_time);
    }
}