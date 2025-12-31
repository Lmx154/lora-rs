use core::sync::atomic::{AtomicU64, Ordering};

static LAST_IRQ_TIMESTAMP_US: AtomicU64 = AtomicU64::new(0);
static mut IRQ_TIMESTAMP_FN: Option<fn() -> u64> = None;

/// Sets a callback function to be used for capturing IRQ timestamps.
///
/// The provided function should return a timestamp in microseconds. This function
/// will be called internally whenever an IRQ status is read from the radio.
///
/// # Arguments
///
/// * `f` - A function pointer that returns the current time in microseconds
///
/// # Safety
///
/// This function uses unsafe code to set a static mutable variable. The caller must
/// ensure that this function is not called concurrently from multiple threads.
pub fn set_irq_timestamp_fn(f: fn() -> u64) {
    unsafe {
        IRQ_TIMESTAMP_FN = Some(f);
    }
}

/// Clears the previously set IRQ timestamp callback function.
///
/// After calling this function, IRQ timestamps will no longer be recorded.
///
/// # Safety
///
/// This function uses unsafe code to modify a static mutable variable. The caller must
/// ensure that this function is not called concurrently from multiple threads.
pub fn clear_irq_timestamp_fn() {
    unsafe {
        IRQ_TIMESTAMP_FN = None;
    }
}

/// Returns the timestamp (in microseconds) of the last recorded IRQ event.
///
/// If no IRQ has been recorded yet, or if no timestamp function has been set,
/// this will return 0.
///
/// # Returns
///
/// The last recorded IRQ timestamp in microseconds
pub fn last_irq_timestamp_us() -> u64 {
    LAST_IRQ_TIMESTAMP_US.load(Ordering::Relaxed)
}

/// Records the current timestamp when an IRQ status is read.
///
/// This function is called internally by the radio driver when reading IRQ status.
/// If a timestamp function has been set via `set_irq_timestamp_fn`, it will be
/// called and the result stored for later retrieval via `last_irq_timestamp_us`.
pub(crate) fn record_irq_timestamp() {
    let f = unsafe { IRQ_TIMESTAMP_FN };
    if let Some(f) = f {
        LAST_IRQ_TIMESTAMP_US.store(f(), Ordering::Relaxed);
    }
}
