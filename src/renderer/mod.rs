use crate::library::{
    Communicator,
    Communique,
};

pub mod library;




//declaration
    pub struct Renderer {
        //loop
            halt: bool,
            tick: usize,
            max_tick: usize,
            heed_max_tick: bool,
            tick_duration: std::time::Duration,

        //communication with the orchestrator
            communicator_to_orchestrator: Communicator<library::MessageFromOrchestratorToRenderer, library::MessageFromRendererToOrchestrator>,

        //wgpu
            instance: wgpu::Instance, 
            surface: wgpu::Surface, 
            size: winit::dpi::PhysicalSize<u32>,
            initial_device_pixel_density_ratio: f64,
    }

//pre-creation setup
    //initial wgpu usage of window
        //you can't pass a reference to the window to this thread, so this function does the actions that the window is needed for
        //and returns the resultant values, which can be passed to a between threads. These values are then passed into the "new"
        //constructor function
        pub struct WgpuSetupDataBlock {
            instance: wgpu::Instance, 
            surface: wgpu::Surface, 
            size: winit::dpi::PhysicalSize<u32>,
            initial_device_pixel_density_ratio: f64,
        }
        impl Renderer {
                pub fn wgpu_setup(
                    window: &winit::window::Window,
                ) -> WgpuSetupDataBlock {
                    let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
                    let surface = unsafe { instance.create_surface(window) };
                    let size = window.inner_size();

                    WgpuSetupDataBlock {
                        instance: instance,
                        surface: surface,
                        size: size,
                        initial_device_pixel_density_ratio: window.scale_factor(),
                    }
                }
        }

//creation
    impl Renderer {
        pub fn new(
            command_channel_receiver: std::sync::mpsc::Receiver<Communique<library::MessageFromOrchestratorToRenderer>>,
            message_channel_sender: std::sync::mpsc::Sender<Communique<library::MessageFromRendererToOrchestrator>>,
            wgpu_setup_data: WgpuSetupDataBlock,
        ) -> Renderer {
            Renderer {
                //loop
                    halt: false,
                    tick: 0,
                    max_tick: 1_000_000,
                    heed_max_tick: false,
                    tick_duration: std::time::Duration::from_millis(1),

                //communication with the orchestrator
                    communicator_to_orchestrator: Communicator::new(command_channel_receiver, message_channel_sender),

                //wgpu
                    instance: wgpu_setup_data.instance,
                    surface: wgpu_setup_data.surface,
                    size: wgpu_setup_data.size,
                    initial_device_pixel_density_ratio: wgpu_setup_data.initial_device_pixel_density_ratio,
            }
        }
    }

//control
    impl Renderer {
        pub fn ignition(&mut self) {
            while !self.halt && (!self.heed_max_tick || self.tick < self.max_tick) {
                self.revolution();
                self.tick+=1;
                std::thread::sleep(self.tick_duration);
            }

            self.communicator_to_orchestrator.send_message(
                library::MessageFromRendererToOrchestrator::Halted
            ).ok();
        }
        pub fn halt(&mut self) {
            self.halt = true;
        }
    }

//revolution
    impl Renderer {
        pub fn revolution(&mut self) {
            //message collection
                let tmp = self.communicator_to_orchestrator.collect_messages();
                for item in tmp {
                    match item.open() {
                        //revolution control
                            library::MessageFromOrchestratorToRenderer::Halt => self.halt(),

                        //test request
                            library::MessageFromOrchestratorToRenderer::Test(number) => {
                                match number {
                                    1 => self.test1(),
                                    2 => self.test2(),
                                    3 => self.test3(),
                                    _ => {},
                                }
                            },
                    }
                }
        }
    }

mod test;