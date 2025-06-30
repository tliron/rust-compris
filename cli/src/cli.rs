use {
    clap::{builder::*, *},
    kutil_cli::clap::*,
    std::path::*,
};

// https://docs.rs/clap/latest/clap/_derive/index.html

//
// CLI
//

/// Query and convert Composite Primitive Schema (CPS) formats
#[derive(Parser)]
#[command(
    name = "compris",
    version,
    propagate_version = true,
    disable_help_flag = true,
    disable_help_subcommand = true,
    disable_version_flag = true,
    styles = clap_styles())
]
pub struct CLI {
    #[command(subcommand)]
    pub subcommand: Option<SubCommand>,

    /// can be a file path or a URL;
    /// when absent will read from stdin
    #[arg(long = "input", short = 'i', verbatim_doc_comment)]
    pub input_path_or_url: Option<String>,

    /// input format;
    /// when absent will use the input path extension if available
    #[arg(long, short = 'F', verbatim_doc_comment, value_enum)]
    pub input_format: Option<InputFormat>,

    /// try to parse numbers as integers;
    /// for "json" format
    #[arg(long, short = 'I', verbatim_doc_comment)]
    pub input_integers: bool,

    /// try to parse numbers as unsigned integers;
    /// for "yaml" and "json" formats;
    /// implies --input-integers
    #[arg(long, short = 'U', verbatim_doc_comment)]
    pub input_unsigned_integers: bool,

    /// accept legacy syntax;
    /// for "yaml" format
    #[arg(long, short = 'L', verbatim_doc_comment)]
    pub input_legacy: bool,

    /// decode input from Base64;
    /// for "cbor" and "messagepack" formats
    #[arg(long, short = 'B', verbatim_doc_comment)]
    pub input_base64: bool,

    /// output file path;
    /// when absent will write to stdout
    #[arg(long = "output", short = 'o', verbatim_doc_comment)]
    pub output_path: Option<PathBuf>,

    /// output format;
    /// when absent will be set to input format
    #[arg(long = "format", short = 'f', verbatim_doc_comment, value_enum)]
    pub output_format: Option<OutputFormat>,

    /// colorize output
    #[arg(long = "colorize", short = 'z', default_value_t = Colorize::True, value_enum)]
    pub output_colorize: Colorize,

    /// plain output;
    /// avoid whitespace and colors
    #[arg(long = "plain", short = 'p')]
    pub output_plain: bool,

    /// encode output to Base64;
    /// for "cbor" and "messagepack" formats
    #[arg(long = "base64", short = 'b', verbatim_doc_comment)]
    pub output_base64: bool,

    /// suppress console output
    #[arg(long, short = 'q', verbatim_doc_comment)]
    pub quiet: bool,

    /// add a log verbosity level;
    /// can be used 3 times
    #[arg(long, short, verbatim_doc_comment, action = ArgAction::Count)]
    pub verbose: u8,

    /// log to file path;
    /// defaults to stderr
    #[arg(long, long = "log", short = 'l', verbatim_doc_comment)]
    pub log_path: Option<PathBuf>,

    /// show this help
    #[arg(long, short = 'h', action = ArgAction::Help)]
    pub help: Option<bool>,
}

//
// InputFormat
//

#[derive(Clone, ValueEnum)]
pub enum InputFormat {
    YAML,
    JSON,
    XJSON,
    XML,
    CBOR,
    #[value(name = "messagepack")]
    MessagePack,
}

impl ToString for InputFormat {
    fn to_string(&self) -> String {
        self.to_possible_value().expect("to_possible_value").get_name().into()
    }
}

//
// OutputFormat
//

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    YAML,
    JSON,
    XJSON,
    XML,
    CBOR,
    #[value(name = "messagepack")]
    MessagePack,
    Debug,
}

impl ToString for OutputFormat {
    fn to_string(&self) -> String {
        self.to_possible_value().expect("to_possible_value").get_name().into()
    }
}

//
// SubCommands
//

#[derive(Subcommand)]
#[command()]
pub enum SubCommand {
    /// show the version of compris
    Version(Version),

    /// output the shell auto-completion script
    Completion(Completion),

    /// output the manual pages (in the troff format)
    Manual(Manual),
}
