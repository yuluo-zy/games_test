//! Supporting compiler probe only; this does not replace original B bytes.
#[no_mangle]
pub fn fallback_enabled() -> bool {
    std::env::var("TG_DISABLE_LANG_FALLBACK").is_err()
}
