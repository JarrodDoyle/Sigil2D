pub mod log {
    use anyhow::Result;
    use rune::Module;

    #[rune::module(::log)]
    pub fn module() -> Result<Module> {
        let mut module = Module::from_meta(self::module_meta)?.with_unique("log");
        module.function_meta(info)?;
        module.function_meta(warn)?;
        module.function_meta(error)?;
        module.function_meta(trace)?;
        module.function_meta(debug)?;

        Ok(module)
    }

    #[rune::function]
    pub fn info(message: &str) {
        log::info!("{message}");
    }

    #[rune::function]
    pub fn warn(message: &str) {
        log::warn!("{message}");
    }

    #[rune::function]
    pub fn error(message: &str) {
        log::error!("{message}");
    }

    #[rune::function]
    pub fn trace(message: &str) {
        log::trace!("{message}");
    }

    #[rune::function]
    pub fn debug(message: &str) {
        log::debug!("{message}");
    }
}
