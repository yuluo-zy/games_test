//! B `crates/tiny-glade/src/load_language_library.rs`（Steam 语言 → lang-lib locale）。
//!
//! 恢复依据（fresh Ghidra shared 2026-10-17，证据在
//! `recovery/agent-team/work/restorer-tinyglade-C/`）：
//! `tiny_glade::load_language_library` `0x140323230`（994 B）。
//!
//! ## 逐语句复原（fresh 反汇编 0x140323420-0x14032368a）
//! 1. 参数：`steam: Option<&Client-like>`（in_RCX，None → 默认 english）与
//!    `lang_override: Option<&str>`（rdi=ptr/rsi=len；Some 时 **直接** 作为
//!    locale 代码，绕过 Steam 与白名单——`cmovne rcx,rdi`）。
//! 2. Steam 存在：`GetCurrentGameLanguage()` → `to_string_lossy` →
//!    `to_lowercase()`。
//! 3. 15 项白名单（长度 6/7/8/9 分支后逐字节比较）→ 映射到 locale 代码。
//!    映射从反汇编的 `lea rcx, [locale_table + offset]` 逐一确认：
//!    english→en-US、italian→it-IT、koreana→ko-KR、russian→ru-RU、
//!    spanish→es-ES、swedish→sv-SE、turkish→tr-TR、french→fr-FR、
//!    german→de-DE、polish→pl-PL、japanese→ja-JP、schinese→zh-CN、
//!    brazilian→pt-BR、norwegian→no-NO、ukrainian→uk-UA
//!    （`.rdata 0x1429bd63b` en-US / 0x1429bd709 起 14 个 5 字节代码）。
//! 4. 未命中：`log::warn!("Unsupported Steam language {}. Falling back to {}",
//!    language, "en-US")`（`.rdata 0x1429bd650` 27 B + 0x1429bd66b 18 B；
//!    级别门 `1 < max_level` = Warn），回落 en-US。
//! 5. `lang_lib::load_locale(<code>).unwrap()`（失败 panic；0x140b9c300）。
//!
//! ## 平台缝
//! B 的第一参数是 `steamworks::Client`。`steamworks` 未在本 crate 启用，
//! 因此以 [`SteamAppsLanguage`] 保留 `GetCurrentGameLanguage` 边界；控制流、
//! 白名单与映射不变。

/// Steam 语言来源（B `steamworks::Client` 的 `ISteamApps::GetCurrentGameLanguage`）。
pub trait SteamAppsLanguage {
    /// 返回 Steam 原生语言名（如 `english`、`schinese`）。
    fn current_game_language(&self) -> String;
}

/// B 函数内联的 Steam 语言白名单（O：15 项，顺序 = 反编译比较顺序）。
pub const SUPPORTED_STEAM_LANGUAGES: [&str; 15] = [
    "english",
    "italian",
    "koreana",
    "russian",
    "spanish",
    "swedish",
    "turkish",
    "french",
    "german",
    "polish",
    "japanese",
    "schinese",
    "brazilian",
    "norwegian",
    "ukrainian",
];

/// 内联白名单谓词（判等提取；输入须已 `to_lowercase()`）。
pub fn is_supported_steam_language(language: &str) -> bool {
    SUPPORTED_STEAM_LANGUAGES.contains(&language)
}

/// Steam 语言名 → lang-lib locale 代码。
///
/// 逐项来自反汇编 `lea rcx, [0x1429bd...]` 的跳转目标；
/// 15 个代码均为 5 字节（`cmove rsi, 5` 的隐含约束）。
pub fn steam_language_to_locale(language: &str) -> Option<&'static str> {
    Some(match language {
        "english" => "en-US",
        "italian" => "it-IT",
        "koreana" => "ko-KR",
        "russian" => "ru-RU",
        "spanish" => "es-ES",
        "swedish" => "sv-SE",
        "turkish" => "tr-TR",
        "french" => "fr-FR",
        "german" => "de-DE",
        "polish" => "pl-PL",
        "japanese" => "ja-JP",
        "schinese" => "zh-CN",
        "brazilian" => "pt-BR",
        "norwegian" => "no-NO",
        "ukrainian" => "uk-UA",
        _ => return None,
    })
}

/// `tiny_glade::load_language_library` `0x140323230`。
///
/// `lang_override` 为 `Some` 时直接使用该 locale 代码（不 lower-case、不查
/// 白名单）；否则用 Steam 语言名映射，未支持时 warn 并回落 `en-US`。
/// 最后 `lang_lib::load_locale(code).unwrap()`（原版失败即 panic）。
pub fn load_language_library(
    steam: Option<&dyn SteamAppsLanguage>,
    lang_override: Option<&str>,
) {
    let language: &str = if let Some(overridden) = lang_override {
        overridden
    } else {
        match steam {
            None => "en-US",
            Some(client) => {
                let language = client.current_game_language().to_lowercase();
                match steam_language_to_locale(&language) {
                    Some(locale) => locale,
                    None => {
                        log::warn!(
                            "Unsupported Steam language {}. Falling back to {}",
                            language,
                            "en-US"
                        );
                        "en-US"
                    }
                }
            }
        }
    };
    lang_lib::load_locale(language).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitelist_matches_recovered_bytes() {
        for ok in SUPPORTED_STEAM_LANGUAGES {
            assert!(is_supported_steam_language(ok));
        }
        for bad in [
            "english_us",
            "English",
            "korean",
            "zh-cn",
            "pt-br",
            "",
            "en",
            "spanish ",
        ] {
            assert!(!is_supported_steam_language(bad), "{bad:?}");
        }
    }

    #[test]
    fn every_whitelisted_language_maps_to_a_five_byte_code() {
        for language in SUPPORTED_STEAM_LANGUAGES {
            let code = steam_language_to_locale(language).expect(language);
            assert_eq!(code.len(), 5, "all codes are 5 bytes: {code}");
            assert!(is_supported_steam_language(language));
        }
        assert_eq!(steam_language_to_locale("korean"), None);
    }
}
