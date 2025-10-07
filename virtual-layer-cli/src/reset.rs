use eyre::Context as _;

use crate::{
    generator::{Generator, GeneratorCtx, ModuleExternal},
    util::WalrusUtilModule as _,
};

#[derive(Debug, Default)]
pub struct ResetCondition;

impl Generator for ResetCondition {}
