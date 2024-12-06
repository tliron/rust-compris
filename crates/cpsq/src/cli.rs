use {
    clap::{builder::*, *},
    clap_complete_command::*,
    common_cli::*,
};

// https://docs.rs/clap/latest/clap/_derive/index.html

//
// CLI
//

/// Query and convert Composite Primitive Schema (CPS) formats
#[derive(Parser)]
#[command(
    name = "cpsq",
    version,
    propagate_version = true,
    disable_help_flag = true,
    disable_help_subcommand = true,
    disable_version_flag = true,
    styles = clap_styles())
]
pub(crate) struct CLI {
    /// can be a file path or a URL;
    /// when absent will read from stdin
    #[arg(id = "INPUT", verbatim_doc_comment)]
    pub(crate) input_path_or_url: Option<String>,

    /// show this help
    #[arg(long, short = 'h', action = ArgAction::Help)]
    pub(crate) help: Option<bool>,

    /// suppress output;
    /// if you only need the exit code
    #[arg(long, short = 'q', verbatim_doc_comment)]
    pub(crate) quiet: bool,

    /// log to file path;
    /// defaults to stderr, applying --colorize
    #[arg(long, long = "log", short = 'l', verbatim_doc_comment)]
    pub(crate) log_path: Option<String>,

    /// add a log verbosity level;
    /// can be used 3 times
    #[arg(long, short, verbatim_doc_comment, action = ArgAction::Count)]
    pub(crate) verbose: u8,

    /// timeout in seconds;
    /// 0 for no timeout
    #[arg(long, short = 't', verbatim_doc_comment, default_value_t = 0.0)]
    pub(crate) timeout: f64,

    /// force input format
    #[arg(long, short = 'F', value_enum)]
    pub(crate) input_format: Option<InputFormat>,

    /// try to parse numbers as integers;
    /// for "json" format
    /// false implies --input-unsigned-integers=false
    #[arg(long, short = 'I', verbatim_doc_comment)]
    pub(crate) input_integers: bool,

    /// try to parse numbers as unsigned integers;
    /// for "yaml" and "json" formats;
    /// true implies --input-integers=true
    #[arg(long, short = 'U', verbatim_doc_comment)]
    pub(crate) input_unsigned_integers: bool,

    /// accept legacy syntax;
    /// for "yaml" format
    #[arg(long, short = 'L', verbatim_doc_comment)]
    pub(crate) input_legacy: bool,

    /// decode input from Base64;
    /// for "cbor" and "messagepack" formats
    #[arg(long, short = 'B', verbatim_doc_comment)]
    pub(crate) input_base64: bool,

    /// output file path;
    /// when empty will write to stdout
    #[arg(long = "output", short = 'o', verbatim_doc_comment)]
    pub(crate) output_path: Option<String>,

    /// output format;
    /// when absent will be set to input format
    #[arg(long = "format", short = 'f', verbatim_doc_comment, value_enum)]
    pub(crate) output_format: Option<OutputFormat>,

    /// colorize output
    #[arg(long = "colorize", short = 'z', default_value_t = Colorize::True, value_enum)]
    pub(crate) output_colorize: Colorize,

    /// prettify output
    #[arg(long = "pretty", short = 'p')]
    pub(crate) output_pretty: bool,

    /// strict output;
    /// for "yaml" format
    #[arg(long = "strict", short = 's', verbatim_doc_comment)]
    pub(crate) output_strict: bool,

    /// encode output to Base64;
    /// for "cbor" and "messagepack" formats
    #[arg(long = "base64", short = 'b', verbatim_doc_comment)]
    pub(crate) output_base64: bool,

    #[command(subcommand)]
    pub(crate) subcommand: Option<SubCommand>,
}

//
//
// Output

//
// InputFormat
//

#[derive(ValueEnum, Clone)]
pub(crate) enum InputFormat {
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
        self.to_possible_value().unwrap().get_name().into()
    }
}

//
// OutputFormat
//

#[derive(ValueEnum, Clone)]
pub(crate) enum OutputFormat {
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
        self.to_possible_value().unwrap().get_name().into()
    }
}

//
// SubCommands
//

#[derive(Subcommand)]
#[command()]
pub(crate) enum SubCommand {
    /// show the version of cpsq
    #[command(action = ArgAction::Version)]
    Version,

    /// output the shell autocompletion script
    Complete {
        /// shell
        #[arg(value_enum)]
        shell: Shell,
    },
}
