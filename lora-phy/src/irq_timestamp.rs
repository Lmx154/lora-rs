//! IRQ timestamp tracking functionality for LoRa radio drivers.
//!
//! This module provides utilities to capture and retrieve timestamps when interrupt
//! requests (IRQs) are received from the radio hardware. This is useful for precise
//! timing measurements and diagnostics.

use core::sync::atomic::Ordering;

#[cfg(target_has_atomic = "64")]
use core::sync::atomic::AtomicU64;
#[cfg(not(target_has_atomic = "64"))]
use portable_atomic::AtomicU64;

/// Stores the timestamp of the last recorded IRQ event in microseconds.
static LAST_IRQ_TIMESTAMP_US: AtomicU64 = AtomicU64::new(0);

/// Optional callback function to retrieve the current timestamp.
static mut IRQ_TIMESTAMP_FN: Option<fn() -> u64> = None;

/// Sets a callback function for capturing IRQ timestamps.
///
/// The provided function should return a monotonic timestamp in microseconds.
/// This callback will be invoked internally whenever an IRQ status is read from
/// the radio hardware.
///
/// # Arguments
///
/// * `f` - A function pointer that returns the current time in microseconds
///
/// # Safety
///
/// This function modifies a static mutable variable. The caller must ensure that
/// this function is not called concurrently from multiple threads or interrupt contexts.
pub fn set_irq_timestamp_fn(f: fn() -> u64) {
    unsafe {
        IRQ_TIMESTAMP_FN = Some(f);
    }
}

/// Clears the IRQ timestamp callback function.
///
/// After calling this function, IRQ timestamps will no longer be recorded until
/// [`set_irq_timestamp_fn`] is called again.
///
/// # Safety
///
/// This function modifies a static mutable variable. The caller must ensure that
/// this function is not called concurrently from multiple threads or interrupt contexts.
pub fn clear_irq_timestamp_fn() {
    unsafe {
        IRQ_TIMESTAMP_FN = None;
    }
}

/// Returns the timestamp of the last recorded IRQ event.
///
/// If no IRQ has been recorded yet, or if no timestamp function has been set
/// via [`set_irq_timestamp_fn`], this will return 0.
///
/// # Returns
///
/// The last recorded IRQ timestamp in microseconds as a `u64` value
pub fn last_irq_timestamp_us() -> u64 {
    LAST_IRQ_TIMESTAMP_US.load(Ordering::Relaxed)
}

/// Records the current timestamp when an IRQ status is read.
///
/// This function is called internally by the radio driver when reading IRQ status.
/// If a timestamp function has been set via [`set_irq_timestamp_fn`], it will be
/// invoked and the result stored atomically for later retrieval via [`last_irq_timestamp_us`].
pub(crate) fn record_irq_timestamp() {
    let f = unsafe { IRQ_TIMESTAMP_FN };
    if let Some(f) = f {
        LAST_IRQ_TIMESTAMP_US.store(f(), Ordering::Relaxed);
    }
}
