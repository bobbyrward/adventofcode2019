use clap::Clap;
use tracing_subscriber::EnvFilter;

use crate::Solutions;

macro_rules! solution {
    ($($day:ident),+) => {
        $(
            mod $day;
        )+

        #[derive(Debug, Clap)]
        pub enum Solutions {
            $(
            #[allow(non_camel_case_types)]
            $day {
                #[clap(subcommand)]
                contents: crate::$day::Args,
            },
            )+
        }

        impl Command for Solutions {
            fn execute(&self) -> anyhow::Result<String> {
                match self {
                    $(Self::$day { contents } => contents.execute(),)+
                }
            }
        }
    }
}

#[derive(Debug, Clap)]
pub struct Args {
    #[clap(long, default_value = "warn")]
    pub logging_filter: String,

    #[clap(subcommand)]
    pub command: Solutions,
}

impl Args {
    pub fn env_filter(&self) -> EnvFilter {
        self.logging_filter.as_str().into()
    }
}
