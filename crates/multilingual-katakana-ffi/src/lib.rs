//! Generic, stable C ABI over `multilingual-katakana-core`.
//!
//! This is the shared binding surface for every consumer that is neither
//! napi (Node.js) nor PyO3 (Python): C#/.NET P/Invoke first, later Go, Ruby,
//! Swift, plain C, etc. All symbols are prefixed `mk_`.
//!
//! # Ownership
//!
//! Rust allocates every string it returns and Rust must free it. Callers
//! MUST release buffers from [`mk_to_katakana`] with [`mk_free_string`] (never
//! with the host allocator's `free`). Input buffers stay owned by the caller.
//!
//! # Thread safety
//!
//! The core has no global mutable state, so all functions may be called
//! concurrently from any number of threads.
//!
//! # Panics
//!
//! No panic ever unwinds across the FFI boundary: every entry point is
//! wrapped in `catch_unwind` and reports [`MK_ERR_PANIC`] instead.
//!
//! # Strings
//!
//! Text is passed as UTF-8 bytes plus a length (not NUL-terminated).
//!
//! # Empty output
//!
//! A successful empty result is returned as a non-NULL dangling pointer with
//! length 0. [`mk_free_string`] treats `len == 0` as a no-op, so freeing it
//! is always safe (but unnecessary).

use multilingual_katakana_core::{native_self_check, to_katakana, KatakanaOptionsOverrides};
use std::panic::{catch_unwind, AssertUnwindSafe};

/// ABI version. Bumped only on a breaking change to the exported surface.
pub const MK_ABI_VERSION: u32 = 1;

/// Success.
pub const MK_OK: i32 = 0;
/// A required pointer argument was NULL.
pub const MK_ERR_NULL_POINTER: i32 = 1;
/// The input bytes were not valid UTF-8.
pub const MK_ERR_INVALID_UTF8: i32 = 2;
/// A Rust panic was caught at the FFI boundary.
pub const MK_ERR_PANIC: i32 = 3;

/// Flag bit: `enable_cyrillic`.
pub const MK_FLAG_ENABLE_CYRILLIC: u32 = 1 << 0;
/// Flag bit: `enable_korean`.
pub const MK_FLAG_ENABLE_KOREAN: u32 = 1 << 1;
/// Flag bit: `enable_chinese`.
pub const MK_FLAG_ENABLE_CHINESE: u32 = 1 << 2;
/// Flag bit: `enable_spanish`.
pub const MK_FLAG_ENABLE_SPANISH: u32 = 1 << 3;
/// Flag bit: `enable_french`.
pub const MK_FLAG_ENABLE_FRENCH: u32 = 1 << 4;
/// Flag bit: `enable_vietnamese`.
pub const MK_FLAG_ENABLE_VIETNAMESE: u32 = 1 << 5;
/// Flag bit: `enable_thai`.
pub const MK_FLAG_ENABLE_THAI: u32 = 1 << 6;
/// Flag bit: `enable_slang`.
pub const MK_FLAG_ENABLE_SLANG: u32 = 1 << 7;
/// Flag bit: `enable_english`.
pub const MK_FLAG_ENABLE_ENGLISH: u32 = 1 << 8;
/// Flag bit: `normalize_prosody`.
pub const MK_FLAG_NORMALIZE_PROSODY: u32 = 1 << 9;

/// Conversion options passed by pointer to [`mk_to_katakana`].
///
/// **FROZEN LAYOUT.** This struct will never grow. New options must be
/// allocated from the currently unused bits of the two fields; anything that
/// cannot fit requires new functions and an [`MK_ABI_VERSION`] bump.
///
/// A flag whose bit is not set in `flags_set` keeps the core default,
/// regardless of `flags_value`. If it is set in `flags_set`, the matching bit
/// of `flags_value` is the explicit value. Unknown bits are ignored.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct MkOptions {
    /// Bitmask of flags that are explicitly specified.
    pub flags_set: u32,
    /// Values for the flags selected by `flags_set`.
    pub flags_value: u32,
}

type Setter = fn(&mut KatakanaOptionsOverrides, bool);

/// Flag bit -> override field. Adding a flag is one new row here.
const FLAG_TABLE: [(u32, Setter); 10] = [
    (MK_FLAG_ENABLE_CYRILLIC, |o, v| o.enable_cyrillic = Some(v)),
    (MK_FLAG_ENABLE_KOREAN, |o, v| o.enable_korean = Some(v)),
    (MK_FLAG_ENABLE_CHINESE, |o, v| o.enable_chinese = Some(v)),
    (MK_FLAG_ENABLE_SPANISH, |o, v| o.enable_spanish = Some(v)),
    (MK_FLAG_ENABLE_FRENCH, |o, v| o.enable_french = Some(v)),
    (MK_FLAG_ENABLE_VIETNAMESE, |o, v| {
        o.enable_vietnamese = Some(v)
    }),
    (MK_FLAG_ENABLE_THAI, |o, v| o.enable_thai = Some(v)),
    (MK_FLAG_ENABLE_SLANG, |o, v| o.enable_slang = Some(v)),
    (MK_FLAG_ENABLE_ENGLISH, |o, v| o.enable_english = Some(v)),
    (MK_FLAG_NORMALIZE_PROSODY, |o, v| {
        o.normalize_prosody = Some(v)
    }),
];

impl From<&MkOptions> for KatakanaOptionsOverrides {
    fn from(opts: &MkOptions) -> Self {
        let mut out = KatakanaOptionsOverrides::default();
        for (bit, set) in FLAG_TABLE {
            if opts.flags_set & bit != 0 {
                set(&mut out, opts.flags_value & bit != 0);
            }
        }
        out
    }
}

/// Run `f`, converting a panic into [`MK_ERR_PANIC`]. Every entry point goes
/// through this so no unwind can cross the FFI boundary.
fn guard<F: FnOnce() -> i32>(f: F) -> i32 {
    catch_unwind(AssertUnwindSafe(f)).unwrap_or(MK_ERR_PANIC)
}

/// Write the failure state (`NULL`, `0`) to whichever out-pointers are non-null.
///
/// # Safety
/// Non-null pointers must be valid for writes.
unsafe fn reset_out(out_ptr: *mut *mut u8, out_len: *mut usize) {
    if !out_ptr.is_null() {
        *out_ptr = std::ptr::null_mut();
    }
    if !out_len.is_null() {
        *out_len = 0;
    }
}

/// Validate pointers and borrow the input as `&str`.
///
/// # Safety
/// `text_ptr` must be valid for reads of `text_len` bytes when non-null.
unsafe fn read_input<'a>(text_ptr: *const u8, text_len: usize) -> Result<&'a str, i32> {
    if text_len == 0 {
        return Ok("");
    }
    if text_ptr.is_null() {
        return Err(MK_ERR_NULL_POINTER);
    }
    let bytes = std::slice::from_raw_parts(text_ptr, text_len);
    std::str::from_utf8(bytes).map_err(|_| MK_ERR_INVALID_UTF8)
}

/// Core of [`mk_to_katakana`], run inside [`guard`].
///
/// # Safety
/// Same contract as [`mk_to_katakana`]; `out_ptr`/`out_len` are non-null.
unsafe fn convert_into(
    text_ptr: *const u8,
    text_len: usize,
    options: *const MkOptions,
    out_ptr: *mut *mut u8,
    out_len: *mut usize,
) -> i32 {
    let text = match read_input(text_ptr, text_len) {
        Ok(t) => t,
        Err(code) => return code,
    };
    let resolved = options
        .as_ref()
        .map(|o| KatakanaOptionsOverrides::from(o).resolve());
    let result = to_katakana(text, resolved.as_ref());
    let boxed = result.into_bytes().into_boxed_slice();
    *out_len = boxed.len();
    *out_ptr = Box::into_raw(boxed) as *mut u8;
    MK_OK
}

/// Returns the ABI version ([`MK_ABI_VERSION`]) this library implements.
#[no_mangle]
pub extern "C" fn mk_abi_version() -> u32 {
    MK_ABI_VERSION
}

/// Convert UTF-8 `text` to Katakana.
///
/// On success returns [`MK_OK`] and stores a Rust-allocated UTF-8 buffer
/// (not NUL-terminated) in `*out_ptr` / `*out_len`; release it with
/// [`mk_free_string`]. On any error `*out_ptr = NULL` and `*out_len = 0`.
/// Empty output is a non-NULL dangling pointer with length 0.
///
/// `options` may be NULL, meaning all core defaults.
///
/// # Safety
/// - `text_ptr` must be valid for reads of `text_len` bytes; it may be NULL
///   only when `text_len == 0`.
/// - `options`, if non-NULL, must point to a valid [`MkOptions`].
/// - `out_ptr` and `out_len` must be non-NULL and valid for writes
///   (otherwise [`MK_ERR_NULL_POINTER`] is returned).
#[no_mangle]
pub unsafe extern "C" fn mk_to_katakana(
    text_ptr: *const u8,
    text_len: usize,
    options: *const MkOptions,
    out_ptr: *mut *mut u8,
    out_len: *mut usize,
) -> i32 {
    reset_out(out_ptr, out_len);
    if out_ptr.is_null() || out_len.is_null() {
        return MK_ERR_NULL_POINTER;
    }
    let code = guard(|| convert_into(text_ptr, text_len, options, out_ptr, out_len));
    if code != MK_OK {
        reset_out(out_ptr, out_len);
    }
    code
}

/// Free a buffer returned by [`mk_to_katakana`]. NULL (or `len == 0`) is a no-op.
///
/// # Safety
/// `ptr`/`len` must be exactly the pair returned by a successful
/// [`mk_to_katakana`] call, and must not be freed twice.
#[no_mangle]
pub unsafe extern "C" fn mk_free_string(ptr: *mut u8, len: usize) {
    if ptr.is_null() || len == 0 {
        return;
    }
    let _ = guard(|| {
        drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(ptr, len)));
        MK_OK
    });
}

/// Run the core's native self check. Returns 1 if healthy, 0 otherwise
/// (including if the check panics).
#[no_mangle]
pub extern "C" fn mk_self_check() -> i32 {
    let mut ok = 0;
    guard(|| {
        ok = i32::from(native_self_check());
        MK_OK
    });
    ok
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guard_converts_panic_to_err_panic() {
        assert_eq!(guard(|| panic!("boom")), MK_ERR_PANIC);
        assert_eq!(guard(|| MK_OK), MK_OK);
    }

    #[test]
    fn flag_mapping_respects_set_mask_and_ignores_unknown_bits() {
        let o = KatakanaOptionsOverrides::from(&MkOptions {
            flags_set: MK_FLAG_ENABLE_ENGLISH | 1 << 31,
            flags_value: 1 << 31,
        });
        assert_eq!(o.enable_english, Some(false));
        assert_eq!(o.enable_korean, None);
    }
}
