use clap::{Args, Parser, ValueEnum};

/// Run sub commands
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// Display format
    #[command(flatten)]
    // #[command(subcommand)]
    pub format: Format,

    /// The shell used to evaluate the commands
    #[arg(long, value_enum, default_value_t = ShellArg::Bash)]
    pub shell: ShellArg,

    // #[arg(value_enum, short, long, default_value_t = FormatArg::Simple)]
    // pub format: FormatArg,
    /// The command to run as many times as there are argument strings
    // #[arg()]
    #[arg(long)]
    pub cmd: String,

    /// The arguments for each call of the given command
    // #[arg(short, long, num_args=1..)]
    #[arg(num_args=1.., required = true, last = true)]
    pub cmd_args: Vec<String>,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum ShellArg {
    Bash,
    Zsh,
}

#[derive(Args, Debug)]
pub struct Format {
    #[command(flatten)]
    pub cmd_format: CommandFormatArgs,

    /// The preset formatter to use for the command output
    #[arg(short, long, value_enum, default_value_t = FormatClass::Simple)]
    pub output_format: FormatClass,

    #[command(flatten)]
    pub header: FormatHeaderArgs,

    #[command(flatten)]
    pub simple: FormatSimpleArgs,

    /// Printed after each command invocation
    #[arg(long, default_value_t = String::from(""))]
    pub format_separator: String,
}

#[derive(Args, Debug)]
pub struct CommandFormatArgs {
    /// The preset formatter to use to display each command (not its output)
    #[arg(long, value_enum, default_value_t = CommandFormatClass::Highlight)]
    pub cmd_format: CommandFormatClass,

    /// Prefix inserted before each command
    #[arg(long, default_value_t = String::from("> "))]
    pub cmd_format_prefix: String,

    /// Which ANSI color code to use when the 'highlight' preset is used
    #[arg(long, default_value_t = 3)]
    pub cmd_format_highlight_color: u8,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum CommandFormatClass {
    /// Display the command name in the same style as its arguments
    Plain,

    /// Print the command's arguments in a different style
    Highlight,

    /// Don't print executed command
    None,
}

#[derive(Args, Debug, Clone)]
pub struct FormatHeaderArgs {
    /// Number of header lines for --format=header
    #[arg(long, default_value_t = 1)]
    pub format_header_size: u8,
}

#[derive(Args, Debug, Clone)]
pub struct FormatSimpleArgs {
    #[arg(long, default_value_t = 243)]
    pub format_simple_color_ansi: u8,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum FormatClass {
    Simple,
    Header,
    None,
}
