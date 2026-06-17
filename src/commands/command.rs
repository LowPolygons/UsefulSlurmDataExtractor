use crate::containers::piped_input::{PipedInputHandler, StructOptions};

pub trait CommandCall {
    fn command(&self, structure: &StructOptions) -> Result<(), ()>;

    fn get_piped_input_handler(&self) -> Box<dyn PipedInputHandler>;
}
