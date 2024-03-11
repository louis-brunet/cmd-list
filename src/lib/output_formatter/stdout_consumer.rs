use std::io::{Write, Read};

pub trait StdoutConsumer<In: Read, Out: Write> {
    type Error;

    fn pipe_stdout(&self, in_stream: &mut In, out_stream: &mut Out) -> Result<(), Self::Error>;
}

