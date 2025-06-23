use super::{cli::*, errors::*};

use {
    clap::*,
    compris::{annotation::*, normal::*, ser::*, *},
    kutil_cli::{debug::*, run::*},
    read_url::*,
    std::{
        fs::*,
        io::{self, IsTerminal},
        path,
    },
};

impl CLI {
    /// Convert.
    pub fn convert(&self) -> Result<(), MainError> {
        let (content, input_format) = self.read::<WithAnnotations>()?;
        self.write(content, input_format)
    }

    fn get_reader(&self) -> Result<(ReadRef, Option<String>), MainError> {
        match &self.input_path_or_url {
            Some(input_url) => {
                let url_context = UrlContext::new();

                #[cfg(feature = "file")]
                let base_urls = url_context.working_dir_url_vec()?;
                #[cfg(not(feature = "file"))]
                let base_urls = Vec::new();

                let context = url_context.with_base_urls(base_urls);

                let url = context.url_or_file_path(input_url)?;

                tracing::info!("reading from URL: {}", url);

                let input_url_extension = {
                    if let Some(extension) = path::Path::new(input_url).extension()
                        && let Some(extension) = extension.to_str()
                    {
                        Some(extension.into())
                    } else {
                        None
                    }
                };

                Ok((Box::new(io::BufReader::new(url.open()?)), input_url_extension))
            }

            None => {
                let stdin = io::stdin();
                if stdin.is_terminal() {
                    CLI::command().print_help()?;
                    return Err(ExitError::success().into());
                }

                tracing::info!("reading from stdin");
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
                tracing::info!("forced input format: {}", format);
                format
            }

            None => match input_url_extension {
                Some(format) => {
                    tracing::info!("input format from URL extension: {}", format);
                    format.into()
                }

                None => {
                    panic!("cannot determine input format; specify it explicitly with --input-format/-n");
                }
            },
        };

        input_format.parse()
    }

    fn read<AnnotationsT>(&self) -> Result<(Value<AnnotationsT>, Format), MainError>
    where
        AnnotationsT: Annotated + Clone + Default,
    {
        let (mut reader, input_url_extension) = self.get_reader()?;
        let input_format = self.get_input_format(&input_url_extension)?;

        Ok((
            compris::parse::Parser::new(input_format.clone())
                .with_try_integers(self.input_integers)
                .with_try_unsigned_integers(self.input_unsigned_integers)
                .with_allow_legacy_words(self.input_legacy)
                .with_allow_legacy_types(self.input_legacy)
                .with_base64(self.input_base64)
                .parse(&mut reader)?,
            input_format,
        ))
    }

    fn get_output_format(&self, input_format: &compris::Format) -> Option<compris::Format> {
        match &self.output_format {
            Some(output_format) => {
                let output_format = output_format.to_string();
                tracing::info!("output format: {}", output_format);
                match &*output_format {
                    "debug" => None,
                    _ => Some(output_format.parse().expect("format")),
                }
            }

            None => {
                tracing::info!("output format set to input format: {}", input_format);
                Some(input_format.clone())
            }
        }
    }

    fn get_writer(&self, output_format: &Option<compris::Format>) -> Box<dyn io::Write> {
        match &self.output_path {
            Some(output_path) => {
                tracing::info!("writing to file: {}", output_path.display());
                Box::new(io::BufWriter::new(File::create(output_path).expect("file create")))
            }

            None => {
                if self.quiet {
                    tracing::info!("writing to empty");
                    return Box::new(io::empty());
                } else {
                    if let Some(output_format) = &output_format
                        && output_format.is_binary()
                        && !self.output_base64
                    {
                        tracing::info!("writing to stdout (raw)");
                        return Box::new(io::stdout());
                    }

                    tracing::info!("writing to stdout");
                    Box::new(anstream::stdout())
                }
            }
        }
    }

    fn write<AnnotationsT>(&self, content: Value<AnnotationsT>, input_format: compris::Format) -> Result<(), MainError>
    where
        AnnotationsT: Annotated + Clone + Default,
    {
        let output_format = self.get_output_format(&input_format);
        let mut writer = self.get_writer(&output_format);

        match output_format {
            Some(output_format) => {
                let serializer = compris::ser::Serializer::new(output_format.clone())
                    .with_pretty(!self.output_plain)
                    .with_base64(self.output_base64);

                match output_format {
                    compris::Format::YAML => {
                        let serialization_mode = compris::ser::SerializationMode::for_yaml();
                        let content = content.modal(&serialization_mode, &serializer);
                        serializer.write(&content, &mut writer)?;
                    }

                    compris::Format::JSON => {
                        let serialization_mode = compris::ser::SerializationMode::for_json();
                        let content = content.modal(&serialization_mode, &serializer);
                        serializer.write(&content, &mut writer)?;
                    }

                    compris::Format::XJSON => {
                        let serialization_mode = compris::ser::SerializationMode::for_xjson();
                        let content = content.modal(&serialization_mode, &serializer);
                        serializer.write(&content, &mut writer)?;
                    }

                    _ => {
                        serializer.write(&content, &mut writer)?;
                    }
                }
            }

            None => {
                content.write_debug(&mut writer)?;
            }
        }

        Ok(())
    }
}
