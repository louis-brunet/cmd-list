use std::io::{self, Write};

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


