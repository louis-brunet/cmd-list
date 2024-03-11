use std::io::{self, Read, Write};

use clap::builder::styling::Style;

use super::stdout_consumer::StdoutConsumer;

pub struct FormatSimple {
    style: Style,
}

impl FormatSimple {
    pub fn new(style: Style) -> Self {
        Self { style }
    }
}

impl<In: Read, Out: Write> StdoutConsumer<In, Out> for FormatSimple {
    type Error = io::Error;

    fn pipe_stdout(&self, in_stream: &mut In, out_stream: &mut Out) -> Result<(), std::io::Error> {
        let style = self.style;
        let style_reset = style.render_reset();

        let style = style.to_string();
        let style_bytes = style.as_bytes();
        let var_name = style_reset.to_string();
        let style_reset_bytes = var_name.as_bytes();

        out_stream.write_all(style_bytes)?;
        std::io::copy(in_stream, out_stream)?;
        out_stream.write_all(style_reset_bytes)?;

        Ok(())
    }
}
