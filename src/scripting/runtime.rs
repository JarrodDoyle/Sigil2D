use std::{path::PathBuf, sync::Arc};

use anyhow::Result;
use rune::{
    termcolor::{ColorChoice, StandardStream},
    Context, Diagnostics, Source, Sources, Vm,
};
use walkdir::WalkDir;

use super::api;

pub struct Runtime {
    pub vm: Vm,
    pub sources: Vec<PathBuf>,
}

impl Runtime {
    pub fn new(source_dir: &str) -> Result<Self> {
        let mut context = Context::with_default_modules()?;
        context.install(api::log::module()?)?;

        let runtime = Arc::new(context.runtime()?);
        let mut diagnostics = Diagnostics::new();

        let (mut sources, source_paths) = Self::get_sources(source_dir)?;
        let unit = rune::prepare(&mut sources)
            .with_context(&context)
            .with_diagnostics(&mut diagnostics)
            .build()?;

        if !diagnostics.is_empty() {
            let mut writer = StandardStream::stderr(ColorChoice::Always);
            diagnostics.emit(&mut writer, &sources)?;
        }

        let vm = Vm::new(runtime, Arc::new(unit));

        Ok(Self {
            vm,
            sources: source_paths,
        })
    }

    fn get_sources(source_dir: &str) -> Result<(Sources, Vec<PathBuf>)> {
        let mut source_paths = vec![];
        let mut sources = Sources::new();
        for entry in WalkDir::new(source_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|e| e == "rn") {
                sources.insert(Source::from_path(path)?)?;
                source_paths.push(path.to_owned());
            }
        }

        log::warn!("Source paths: {source_paths:?}");

        Ok((sources, source_paths))
    }
}
