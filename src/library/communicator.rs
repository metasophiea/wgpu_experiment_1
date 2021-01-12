use crate::library::Communique;

//declaration
    pub struct Communicator<IN,OUT> {
        id_counter: usize,
        receiver: std::sync::mpsc::Receiver<Communique<IN>>,
        sender: std::sync::mpsc::Sender<Communique<OUT>>,
    }

//creation
    impl<IN,OUT> Communicator<IN,OUT> {
        pub fn new(
            command_channel_receiver: std::sync::mpsc::Receiver<Communique<IN>>,
            message_channel_sender: std::sync::mpsc::Sender<Communique<OUT>>,
        ) -> Communicator<IN,OUT> {
            Communicator {
                id_counter: 0,
                receiver: command_channel_receiver,
                sender: message_channel_sender,
            }
        }
    }

//sending
    impl<IN,OUT> Communicator<IN,OUT> {
        fn generate_id(&mut self) -> usize {
            let new_id = self.id_counter;
            self.id_counter += 1;
            new_id
        }
        pub fn send_message(
            &self,
            message: OUT,
        ) -> std::result::Result<(), std::sync::mpsc::SendError<Communique<OUT>>> {
            self.sender.send(
                Communique::new_no_id(
                    message
                )
            )
        }
        pub fn send_message_with_id(
            &mut self,
            id: usize,
            message: OUT,
        ) -> std::result::Result<(), std::sync::mpsc::SendError<Communique<OUT>>> {
            self.sender.send(
                Communique::new(
                    id, message
                )
            )
        }
        pub fn send_message_with_auto_id(
            &mut self,
            message: OUT,
        ) -> std::result::Result<usize, std::sync::mpsc::SendError<Communique<OUT>>> {
            let id = self.generate_id();

            match self.send_message_with_id( 
                id, 
                message,
            ) {
                Err(e) => Err(e),
                Ok(_) => Ok(id),
            }
        }
    }

//receiving
    impl<IN,OUT> Communicator<IN,OUT> {
        pub fn collect_messages(&self) -> Vec<Communique<IN>> {
            self.receiver.try_iter().collect()
        }
    }