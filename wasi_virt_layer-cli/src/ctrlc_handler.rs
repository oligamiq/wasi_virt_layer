use std::sync::{Arc, Mutex, OnceLock};

type CleanupFn = Box<dyn Fn() + Send + Sync>;

static CLEANUPS: OnceLock<Arc<Mutex<Vec<CleanupFn>>>> = OnceLock::new();

/// Initializes the Ctrl-C handler if it hasn't been initialized already.
pub fn init() {
    CLEANUPS.get_or_init(|| {
        let cleanups = Arc::new(Mutex::new(Vec::<CleanupFn>::new()));
        let cleanups_clone = cleanups.clone();

        // We ignore errors if handler is already set
        let _ = ctrlc::set_handler(move || {
            if let Ok(guards) = cleanups_clone.lock() {
                for guard in guards.iter().rev() {
                    guard();
                }
            }
            std::process::exit(130);
        });

        cleanups
    });
}

/// Registers a cleanup function to be executed when a Ctrl-C signal is received.
pub fn register(f: impl Fn() + Send + Sync + 'static) {
    init(); // Ensure init
    if let Some(cleanups) = CLEANUPS.get() {
        cleanups.lock().unwrap().push(Box::new(f));
    }
}
