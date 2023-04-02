use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry, fmt::MakeWriter};
use tracing::subscriber::set_global_default;

/// Compose multiple layers into a tracing's subscriber
/// 
/// # Implementation note
/// 
/// We are using impl Subscriber as return type to avoid having to
/// spell out the actual type of the returned subscriber, which is
/// indeed quite complex
/// 
/// We need to explicitly call out that the returned subscriber is 
/// Send and sync to make it possible to pass it to 'init_subcriber'
/// later on
pub fn get_subscriber<Sink>(
    name: String,
    filter: String,
    sink: Sink    
) -> impl Subscriber + Sync + Send 
    where 
        // This "weird" syntax is a higher-ranked trait bound (HRTB)
        // It basically means that Sink implements the `MakeWriter`
        // trait for all choices of the lifetime parameter `'a`
        // Check out https://doc.rust-lang.org/nomicon/hrtb.html
        // for more details.
        Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    //  We are falling back to printing all spans at info-level or above
    //  if the RUST_LOG environment variable has not been set.
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(filter));

    let formating_layer = BunyanFormattingLayer::new(
        name,
        //  Output the formatter spans to stdout.
        sink
    );

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formating_layer)       
}

/// Register a subscriber as global defaulrt to process span data
/// 
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Redirect all `log`'s events to our subscriber
    LogTracer::init().expect("Failed to set logger");

    // set_global_default can be used by applications to specify
    // what subscriber should be used to process spans.
    set_global_default(subscriber).expect("failed to set subscriber");      
}