//! The KMS `yubihsm` subcommand

use abscissa::Callable;

mod detect;
mod help;
mod keys;
mod test;

pub use self::{detect::DetectCommand, help::HelpCommand, keys::KeysCommand, test::TestCommand};

/// The `yubihsm` subcommand
#[derive(Debug, Options)]
pub enum YubihsmCommand {
    #[options(help = "detect all YubiHSM2 devices connected via USB")]
    Detect(DetectCommand),

    #[options(help = "show help for the 'yubihsm' subcommand")]
    Help(HelpCommand),

    #[options(help = "key management subcommands")]
    Keys(KeysCommand),

    #[options(help = "perform a signing test")]
    Test(TestCommand),
}

// TODO: custom derive in abscissa
impl_command!(YubihsmCommand);

// TODO: refactor abscissa internally so this is all part of the proc macro
impl Callable for YubihsmCommand {
    /// Call the given command chosen via the CLI
    fn call(&self) {
        match self {
            YubihsmCommand::Detect(detect) => detect.call(),
            YubihsmCommand::Help(help) => help.call(),
            YubihsmCommand::Keys(keys) => keys.call(),
            YubihsmCommand::Test(test) => test.call(),
        }
    }
}

impl YubihsmCommand {
    pub(super) fn config_path(&self) -> Option<&str> {
        match self {
            YubihsmCommand::Detect(detect) => detect.config.as_ref().map(|s| s.as_ref()),
            YubihsmCommand::Keys(keys) => keys.config_path(),
            _ => None,
        }
    }

    pub(super) fn verbose(&self) -> bool {
        match self {
            YubihsmCommand::Detect(detect) => detect.verbose,
            YubihsmCommand::Test(test) => test.verbose,
            _ => false,
        }
    }
}
