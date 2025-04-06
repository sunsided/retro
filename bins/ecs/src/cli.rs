use std::ffi::OsStr;
use clap;


use clap::{Command, Arg, ArgMatches, ValueHint, Error};
use clap::builder::PossibleValue;
use wgpu::Backends;

pub fn parse_arguments() -> ArgMatches {
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("backend")
                .help("Specifies the rendering backend to use")
                .short('b')
                .long("backend")
                .value_hint(ValueHint::Other)
                .default_value("gl")
                .env("WGPU_BACKEND")
                .value_parser(BackendValueParser)
        )
        .get_matches()
}


/// A custom ValueParser for the backend argument
#[derive(Debug, Clone)]
struct BackendValueParser;

impl clap::builder::TypedValueParser for BackendValueParser {
    type Value = Backends;

    fn parse_ref(&self, _cmd: &Command, _arg: Option<&Arg>, value: &OsStr) -> Result<Self::Value, Error> {
        match value.to_str() {
            Some("gl") => Ok(Backends::GL),
            Some("vulkan") => Ok(Backends::VULKAN),
            Some("dx12") => Ok(Backends::DX12),
            Some("metal") => Ok(Backends::METAL),
            Some("auto") => Ok(Backends::default()),
            Some(_) => Err(Error::raw(
                clap::error::ErrorKind::InvalidValue,
                "Invalid backend. Use one of: gl, vulkan, dx12, metal, or auto",
            )),
            None => Err(Error::raw(
                clap::error::ErrorKind::InvalidUtf8,
                "Value is not valid UTF-8",
            )),
        }
    }

    fn possible_values(&self) -> Option<Box<dyn Iterator<Item=PossibleValue> + '_>> {
        Some(Box::new([
            PossibleValue::new("gl"),
            PossibleValue::new("vulkan"),
            PossibleValue::new("dx"),
            PossibleValue::new("metal")
        ].into_iter()))
    }
}
