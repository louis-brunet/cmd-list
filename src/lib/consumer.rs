use std::io::{self, BufRead, BufReader, Read, Write};

use clap::builder::styling::Style;

pub struct CommandFormatter<'prefix> {
    prefix: &'prefix str,
    prefix_style: Style,
    cmd_style: Style,
    args_style: Style,
}

impl<'prefix> CommandFormatter<'prefix>{
    pub fn new(prefix: &'prefix str, prefix_style: Style, cmd_style: Style, args_style: Style) -> Self {
        Self { prefix, prefix_style, cmd_style, args_style }
    }

    pub fn display_command<Out: Write>(
        &self,
        out_stream: &mut Out,
        cmd: &str,
        args: &str,
    ) -> Result<(), io::Error> {
        let prefix_style = self.prefix_style;
        let cmd_style = self.cmd_style;
        let args_style = self.args_style;

        let cmd_str = format!(
            "{prefix_style}{}{prefix_style:#}{cmd_style}{}{cmd_style:#} {args_style}{}{args_style:#}\n",
            self.prefix, cmd, args,
        );
        out_stream.write_all(cmd_str.as_bytes())?;
        Ok(())
    }
}

pub trait StdoutConsumer<In: Read, Out: Write> {
    type Error;

    fn pipe_stdout(&self, in_stream: &mut In, out_stream: &mut Out) -> Result<(), Self::Error>;

    // fn display_command(
    //     &self,
    //     out_stream: &mut Out,
    //     cmd: &str,
    //     args: &str,
    // ) -> Result<(), io::Error> {
    //     let prefix_style = Style::new().dimmed();
    //     let cmd_style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::White)));
    //     let args_style = Style::new()
    //         .bold()
    //         .fg_color(Some(Color::Ansi(AnsiColor::Blue)));
    //     // .bg_color(Some(Color::Ansi(AnsiColor::Black)));
    //
    //     let cmd_str = format!(
    //         "{prefix_style}> {prefix_style:#}{cmd_style}{}{cmd_style:#} {args_style}{}{args_style:#}\n",
    //         cmd, args,
    //     );
    //     out_stream.write_all(cmd_str.as_bytes())?;
    //     Ok(())
    // }
}

pub struct FormatNone;

impl<In: Read, Out: Write> StdoutConsumer<In, Out> for FormatNone {
    type Error = io::Error;

    fn pipe_stdout(&self, in_stream: &mut In, out_stream: &mut Out) -> Result<(), std::io::Error> {
        std::io::copy(in_stream, out_stream).map(|_| ())
    }

    // fn display_command(
    //     &self,
    //     _out_stream: &mut Out,
    //     _cmd: &str,
    //     _args: &str,
    // ) -> Result<(), io::Error> {
    //     Ok(())
    // }
}

pub struct FormatSimple {
    style: Style,
}

// impl Default for FormatSimple {
//     fn default() -> Self {
//         Self { style: Style::new().bold().fg_color(Some(Color::Ansi(AnsiColor::BrightBlack))) }
//     }
// }

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
