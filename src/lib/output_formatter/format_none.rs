use std::io::{self, Read, Write};

use super::stdout_consumer::StdoutConsumer;

pub struct FormatNone;

impl<In: Read, Out: Write> StdoutConsumer<In, Out> for FormatNone {
    type Error = io::Error;

    fn pipe_stdout(&self, in_stream: &mut In, out_stream: &mut Out) -> Result<(), io::Error> {
        std::io::copy(in_stream, out_stream).map(|_| ())
    }
}
