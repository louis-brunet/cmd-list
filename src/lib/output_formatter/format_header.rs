use std::io::{self, BufRead, BufReader, Read, Write};

use clap::builder::styling::Style;

use super::stdout_consumer::StdoutConsumer;

pub struct FormatHeader {
    header_style: Style,
    body_style: Style,
    header_size: u8,
}

impl FormatHeader {
    pub fn new(header_style: Style, body_style: Style, header_size: u8) -> Self {
        Self {
            header_style,
            body_style,
            header_size,
        }
    }
}

impl<In: Read, Out: Write> StdoutConsumer<In, Out> for FormatHeader {
    type Error = io::Error;

    fn pipe_stdout(&self, in_stream: &mut In, out_stream: &mut Out) -> Result<(), Self::Error> {
        let mut buf_reader = BufReader::new(in_stream);

        for _ in 0..self.header_size {
            let mut line = String::new();
            buf_reader.read_line(&mut line)?;

            let styled_header = format!(
                "{}{}{}",
                self.header_style,
                line,
                self.header_style.render_reset()
            );
            out_stream.write_all(styled_header.as_bytes())?;
        }

        out_stream.write_all(self.body_style.to_string().as_bytes())?;
        io::copy(&mut buf_reader, out_stream)?;
        out_stream.write_all(self.body_style.render_reset().to_string().as_bytes())?;
        Ok(())
    }
}
