use anchor_lang::prelude::*;

#[error]
pub enum CommonError {
    #[msg("Invalid Signer")]
    InvalidSigner,
    #[msg("Invalid Updater")]
    InvalidUpdater,
}
