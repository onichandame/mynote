use std::error::Error;

pub async fn setup_trace() -> Result<(), Box<dyn Error + Send + Sync>> {
    #[cfg(debug_assertions)]
    {
        tracing_subscriber::fmt::init();
    }
    /// TODO: setup Cloud Trace
    #[cfg(not(debug_assertions))]
    {
        tracing_subscriber::fmt::init();
        //let tracer = stdout::new_pipeline().install_simple();
        //let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
        //tracing_subscriber::registry()
        //    .with(opentelemetry)
        //    .try_init()?;
    }
    Ok(())
}
