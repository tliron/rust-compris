use super::{cli::*, errors::*};

use {
    clap::CommandFactory,
    compris::WriteDebugDyn,
    kutil_cli::*,
    read_url::*,
    std::{
        fs::File,
        io::{empty, stdin, stdout, BufReader, BufWriter, IsTerminal, Read, Write},
        path::*,
    },
    tracing::info,
};

impl CLI {
    pub fn convert(&self) -> Result<(), MainError> {
        let (content, input_format) = self.read()?;
        self.write(content, input_format)
    }

    fn get_reader(&self) -> Result<(Box<dyn Read>, Option<String>), MainError> {
        match &self.input_path_or_url {
            Some(input_url) => {
                let context = Context::new();
                let bases = context.new_working_dir_url_vec()?;
                let url = context.new_valid_any_or_file_url(input_url, &bases)?;

                info!("reading from URL: {}", url);

                let input_url_extension = {
                    if let Some(extension) = Path::new(input_url).extension() {
                        if let Some(extension) = extension.to_str() {
                            Some(extension.into())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                };

                Ok((Box::new(BufReader::new(url.open()?)), input_url_extension))
            }

            None => {
                let stdin = stdin();
                if stdin.is_terminal() {
                    CLI::command().print_help()?;
                    return Err(MainError::Exit(Exit::success()));
                }

                info!("reading from stdin");
                Ok((Box::new(stdin), None))
            }
        }
    }

    fn get_input_format(
        &self,
        input_url_extension: &Option<String>,
    ) -> Result<compris::Format, compris::UnknownFormatError> {
        let input_format = match &self.input_format {
            Some(format) => {
                let format = format.to_string();
                info!("forced input format: {}", format);
                format
            }

            None => match input_url_extension {
                Some(format) => {
                    info!("input format from URL extension: {}", format);
                    format.into()
                }

                None => {
                    panic!("cannot determine input format; specify it explicitly with --input-format/-n");
                }
            },
        };

        compris::Format::try_from(&*input_format.as_str())
    }

    fn read(&self) -> Result<(compris::Value, compris::Format), MainError> {
        let (reader, input_url_extension) = self.get_reader()?;
        let reader = Box::leak(reader);
        let input_format = self.get_input_format(&input_url_extension)?;

        Ok((
            compris::read::Reader::new(reader, input_format.clone())
                .with_try_integers(self.input_integers)
                .with_try_unsigned_integers(self.input_unsigned_integers)
                .with_allow_legacy_words(self.input_legacy)
                .with_allow_legacy_types(self.input_legacy)
                .with_base64(self.input_base64)
                .read()?,
            input_format,
        ))
    }

    fn get_output_format(&self, input_format: &compris::Format) -> Option<compris::Format> {
        match &self.output_format {
            Some(output_format) => {
                let output_format = output_format.to_string();
                info!("output format: {}", output_format);
                match &*output_format {
                    "debug" => None,
                    _ => Some(compris::Format::try_from(&*output_format).unwrap()),
                }
            }

            None => {
                info!("output format set to input format: {}", input_format);
                Some(input_format.clone())
            }
        }
    }

    fn get_writer(&self, output_format: &Option<compris::Format>) -> Box<dyn Write> {
        match &self.output_path {
            Some(output_url) => {
                let output_path = Path::new(&*output_url);
                info!("writing to file: {}", output_path.display());
                Box::new(BufWriter::new(File::create(output_path).unwrap()))
            }

            None => {
                if self.quiet {
                    info!("writing to empty");
                    return Box::new(empty());
                } else {
                    if let Some(output_format) = &output_format {
                        if output_format.is_binary() && !self.output_base64 {
                            info!("writing to stdout (raw)");
                            return Box::new(stdout());
                        }
                    }

                    info!("writing to stdout");
                    Box::new(anstream::stdout())
                }
            }
        }
    }

    fn write(&self, content: compris::Value, input_format: compris::Format) -> Result<(), MainError> {
        let output_format = self.get_output_format(&input_format);
        let writer = Box::leak(self.get_writer(&output_format));

        match output_format {
            Some(output_format) => {
                let mut serialize = compris::ser::Serializer::new(writer)
                    .with_format(output_format.clone())
                    .with_pretty(!self.output_plain)
                    .with_strict(self.output_strict)
                    .with_base64(self.output_base64);

                match output_format {
                    compris::Format::YAML => {
                        let serialization_mode = compris::ser::SerializationMode::for_yaml();
                        let content = content.with_serialization_mode(&serialization_mode);
                        serialize.write(&content)?;
                    }

                    compris::Format::JSON => {
                        let serialization_mode = compris::ser::SerializationMode::for_json();
                        let content = content.with_serialization_mode(&serialization_mode);
                        serialize.write(&content)?;
                    }

                    compris::Format::XJSON => {
                        let serialization_mode = compris::ser::SerializationMode::for_xjson();
                        let content = content.with_serialization_mode(&serialization_mode);
                        serialize.write(&content)?;
                    }

                    _ => {
                        serialize.write(&content)?;
                    }
                }
            }

            None => {
                content.write_debug_dyn(writer)?;
            }
        }

        Ok(())
    }
}
