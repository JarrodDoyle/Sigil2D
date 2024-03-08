use std::sync::Arc;

use anyhow::Result;
use rune::{
    termcolor::{ColorChoice, StandardStream},
    Context, Diagnostics, Source, Sources, Vm,
};

use super::api;

pub struct Runtime {
    pub vm: Vm,
    pub sources: Vec<String>,
}

impl Runtime {
    pub fn new(source_paths: &[&str]) -> Result<Self> {
        let mut full_source_paths = vec![];

        let source_dir = format!("{}/scripts", env!("CARGO_MANIFEST_DIR"));
        let mut sources = Sources::new();
        for i in 0..source_paths.len() {
            let path = format!("{source_dir}/{}.rn", source_paths[i]);
            full_source_paths.push(path.clone());
            sources.insert(Source::from_path(path)?)?;
        }

        let mut context = Context::with_default_modules()?;
        context.install(api::log::module()?)?;

        let runtime = Arc::new(context.runtime()?);
        let mut diagnostics = Diagnostics::new();

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
            sources: full_source_paths,
        })
    }
}
