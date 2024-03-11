use std::{
    io::{self, Stdout, Write},
    process::ChildStdout,
};

use args::{Format, FormatClass, ShellArg};
use clap::{
    builder::styling::{Ansi256Color, AnsiColor, Color, Style},
    Parser,
};
use cmd_list::{
    command_formatter::CommandFormatter,
    output_formatter::{FormatHeader, FormatNone, FormatSimple, StdoutConsumer},
    runner::CommandRunner,
};

use crate::args::CliArgs;

mod args;

fn get_output_formatter(
    format: &Format,
) -> Box<dyn StdoutConsumer<ChildStdout, Stdout, Error = io::Error>> {
    match format.output_format {
        FormatClass::Simple => {
            let simple_color =
                Color::Ansi256(Ansi256Color::from(format.simple.format_simple_color_ansi));
            let simple_style = Style::new().fg_color(Some(simple_color));

            Box::new(FormatSimple::new(simple_style))
        }
        FormatClass::Header => {
            let header_style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::BrightBlack)));
            let body_style = Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green)));

            Box::new(FormatHeader::new(
                header_style,
                body_style,
                format.header.format_header_size,
            ))
        }
        FormatClass::None => Box::new(FormatNone),
    }
}

fn get_command_formatter(cmd_format: &args::CommandFormatArgs) -> CommandFormatter {
    match cmd_format.cmd_format {
        args::CommandFormatClass::Highlight => {
            let prefix_style = Style::new().dimmed();
            let cmd_style = Style::new();
            let args_style =
                Style::new()
                    .bold()
                    .fg_color(Some(Color::Ansi256(Ansi256Color::from(
                        cmd_format.cmd_format_highlight_color,
                    ))));

            CommandFormatter::new(
                cmd_format.cmd_format_prefix.as_str(),
                prefix_style,
                cmd_style,
                args_style,
            )
        }

        args::CommandFormatClass::Plain => {
            let prefix_style = Style::new().dimmed();
            let cmd_style = Style::new();
            let args_style = cmd_style;

            CommandFormatter::new(
                cmd_format.cmd_format_prefix.as_str(),
                prefix_style,
                cmd_style,
                args_style,
            )
        }

        args::CommandFormatClass::None => todo!(),
    }
}

fn main() {
    let args = CliArgs::parse();

    let command_formatter = get_command_formatter(&args.format.cmd_format);
    let output_formatter = get_output_formatter(&args.format);
    let separator = args
        .format
        .format_separator
        .replace("\\n", "\n")
        .replace("\\t", "\t");
    let shell = match args.shell {
        ShellArg::Bash => "bash",
        ShellArg::Zsh => "zsh",
    };

    let mut out_stream = std::io::stdout();
    for arg in args.cmd_args {
        command_formatter
            .display_command(&mut out_stream, args.cmd.as_str(), arg.as_str())
            .expect("display_command");

        let o = CommandRunner::run_command(shell, args.cmd.as_str(), arg.as_str())
            .expect("run_command failed");

        output_formatter
            .pipe_stdout(&mut o.stdout.expect("child stdout"), &mut out_stream)
            .expect("pipe_stdout");

        out_stream
            .write_all(separator.as_bytes())
            .expect("separator");
    }
}
