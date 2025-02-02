//! This module defines data-structure into which user `Cargo.toml` is parsed

use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub(crate) struct Root {
    pub(crate) package: Option<Package>,
    pub(crate) lib: Option<Lib>,
}

#[derive(Clone, Deserialize)]
pub(crate) struct Package {
    pub(crate) name: Option<String>,
    pub(crate) version: Option<String>,
}

#[derive(Clone, Deserialize)]
pub(crate) struct Lib {
    #[serde(alias = "crate-type")]
    pub(crate) crate_type: Option<Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum CrateType {
    /// `staticlib` target, which is what you want (really)
    StaticLib,
    /// `cdylib` target (overide `staticlib` target since `staticlib` require no
    /// `cabal-pack` extra step that wouldn't require `cdylib`)
    DynLib,
}

/// From a list a targets return the one that represent the strategy used by
/// `cabal-pack`, return `None` when there is no target usable by `cabal-pack`
/// like `rlib` or `dylib`.
pub(crate) fn get_crate_type(cargo: Root) -> Option<CrateType> {
    let crate_type = cargo.lib?.crate_type?;
    crate_type
        .contains(&"cdylib".to_string())
        .then_some(CrateType::DynLib)
        .or_else(|| {
            crate_type
                .contains(&"staticlib".to_string())
                .then_some(CrateType::StaticLib)
        })
}
