mod batchlink;

#[cfg(test)]
pub mod tests {
    use log::LevelFilter;
    use std::sync::Once;

    static INIT: Once = Once::new();

    #[cfg(test)]
    #[ctor::ctor]
    fn init() {
        INIT.call_once(|| {
            env_logger::builder()
                .is_test(true)
                .filter_level(LevelFilter::Info)
                .filter_module(env!("CARGO_PKG_NAME"), LevelFilter::Trace)
                .init();
        });
    }
}
