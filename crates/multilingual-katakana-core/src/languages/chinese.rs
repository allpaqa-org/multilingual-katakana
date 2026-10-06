use regex::Regex;
use std::sync::LazyLock;

use crate::dicts::{
    lookup_char_sorted, CHINESE_COMMON_WORDS, CHINESE_HANZI_MAP, CHINESE_MARKER_PATTERN,
    CHINESE_TAIWAN_PHRASES, HANZI_JAPANESE_ONLY, HANZI_SIMPLIFIED_ONLY, JAPANESE_GUARD_WORDS,
    ZH_CONTEXT_SLANG,
};

static CHINESE_MARKER_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(CHINESE_MARKER_PATTERN).expect("Invalid CHINESE_MARKER_PATTERN"));

static PUNCT_SPACE_BEFORE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[\s\u{3000}]+([、。！？!?,.])").expect("Invalid regex"));

static PUNCT_SPACE_AFTER_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"([、。！？!?,.])[\s\u{3000}]+").expect("Invalid regex"));

static MULTI_SPACE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[\s\u{3000}]+").expect("Invalid regex"));

/// Determines if the text should be treated as Chinese.
///
/// 1. Japanese Absolute Guard: If text contains ANY Hiragana or Katakana, returns `false`.
/// 2. Must contain at least one CJK Ideograph (`\u{4E00}..=\u{9FFF}`).
/// 3. Checks Chinese marker regex pattern (`CHINESE_MARKER_PATTERN`).
/// 4. Checks Chinese common words (`CHINESE_COMMON_WORDS`).
pub fn is_chinese(text: &str) -> bool {
    // 1. Japanese Absolute Guard: If text contains ANY Hiragana or Katakana, 100% Japanese!
    if text
        .chars()
        .any(|c| matches!(c, '\u{3040}'..='\u{309F}' | '\u{30A0}'..='\u{30FF}'))
    {
        return false;
    }

    // 2. Must contain at least one CJK Ideograph (漢字/汉字)
    if !text.chars().any(|c| ('\u{4E00}'..='\u{9FFF}').contains(&c)) {
        return false;
    }

    // 3. Check for distinctive Chinese marker characters / particles
    if CHINESE_MARKER_REGEX.is_match(text) {
        return true;
    }

    // 4. Check for common Chinese phrases/greetings
    for &word in CHINESE_COMMON_WORDS {
        if text.contains(word) {
            return true;
        }
    }

    // 5. Otherwise return false
    false
}

/// Replace Taiwan phrases and greetings (sorted by length descending).
pub fn replace_taiwan_phrases(text: &str) -> String {
    let mut result = text.to_string();
    for &(phrase, katakana) in CHINESE_TAIWAN_PHRASES {
        if result.contains(phrase) {
            result = result.replace(phrase, katakana);
        }
    }
    result
}

/// Classification of one maximal CJK ideograph run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RunClass {
    Zh,
    Ja,
    Mixed,
    Undecided,
}

/// One maximal CJK ideograph run (byte offsets into the comment).
struct HanziRun {
    start: usize,
    end: usize,
    cls: RunClass,
}

#[inline]
fn is_hanzi(c: char) -> bool {
    ('\u{4E00}'..='\u{9FFF}').contains(&c)
}

/// Kana plus the Japanese-only iteration/closing marks 々 and 〆.
#[inline]
fn is_japanese_script(c: char) -> bool {
    matches!(c, '\u{3040}'..='\u{309F}' | '\u{30A0}'..='\u{30FF}' | '\u{3005}' | '\u{3006}')
}

/// Gap characters that end a sentence/clause: runs separated by these are not linked.
#[inline]
fn is_sentence_break(c: char) -> bool {
    matches!(c, '、' | '。' | '！' | '？' | '!' | '?' | '.' | '\n')
}

fn has_char(text: &str, table: &[char]) -> bool {
    text.chars().any(|c| table.binary_search(&c).is_ok())
}

/// Evidence-based class of a run, from its own characters only.
fn classify_by_evidence(run: &str) -> RunClass {
    let zh = has_char(run, HANZI_SIMPLIFIED_ONLY)
        || CHINESE_TAIWAN_PHRASES.iter().any(|&(p, _)| run.contains(p));
    let ja =
        has_char(run, HANZI_JAPANESE_ONLY) || JAPANESE_GUARD_WORDS.iter().any(|&w| run.contains(w));
    match (zh, ja) {
        (true, true) => RunClass::Mixed,
        (true, false) => RunClass::Zh,
        (false, true) => RunClass::Ja,
        (false, false) => RunClass::Undecided,
    }
}

fn find_runs(text: &str) -> Vec<HanziRun> {
    let mut runs = Vec::new();
    let mut start: Option<usize> = None;
    for (i, c) in text.char_indices() {
        match (is_hanzi(c), start) {
            (true, None) => start = Some(i),
            (false, Some(s)) => {
                runs.push(HanziRun {
                    start: s,
                    end: i,
                    cls: classify_by_evidence(&text[s..i]),
                });
                start = None;
            }
            _ => {}
        }
    }
    if let Some(s) = start {
        runs.push(HanziRun {
            start: s,
            end: text.len(),
            cls: classify_by_evidence(&text[s..]),
        });
    }
    runs
}

/// Rules (a) kana adjacency and (b) delimiter hints for a SHARED-only run.
fn classify_by_context(text: &str, run: &HanziRun, comment_has_kana: bool) -> RunClass {
    let before = &text[..run.start];
    let after = &text[run.end..];
    let adjacent_kana = before.chars().next_back().is_some_and(is_japanese_script)
        || after.chars().next().is_some_and(is_japanese_script);
    if adjacent_kana {
        return RunClass::Ja;
    }
    let prev = before.trim_end().chars().next_back();
    let next = after.trim_start().chars().next();
    if prev == Some('、') || next == Some('、') {
        return RunClass::Ja;
    }
    if !comment_has_kana && (prev == Some('，') || next == Some('，')) {
        return RunClass::Zh;
    }
    RunClass::Undecided
}

/// Two consecutive runs are linked when no kana / sentence break lies between them.
fn is_linked(text: &str, left: &HanziRun, right: &HanziRun) -> bool {
    !text[left.end..right.start]
        .chars()
        .any(|c| is_japanese_script(c) || is_sentence_break(c))
}

fn linked_class(text: &str, runs: &[HanziRun], i: usize) -> RunClass {
    let prev = if i > 0 && is_linked(text, &runs[i - 1], &runs[i]) {
        runs[i - 1].cls
    } else {
        RunClass::Undecided
    };
    let next = if i + 1 < runs.len() && is_linked(text, &runs[i], &runs[i + 1]) {
        runs[i + 1].cls
    } else {
        RunClass::Undecided
    };
    // Japanese wins over Chinese (Safe Kanji Guard); mixed runs never propagate.
    if prev == RunClass::Ja || next == RunClass::Ja {
        RunClass::Ja
    } else if prev == RunClass::Zh || next == RunClass::Zh {
        RunClass::Zh
    } else {
        RunClass::Undecided
    }
}

/// Rule (c): propagate decided classes through linked neighbours until stable.
fn propagate_links(text: &str, runs: &mut [HanziRun]) {
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..runs.len() {
            if runs[i].cls != RunClass::Undecided {
                continue;
            }
            let cls = linked_class(text, runs, i);
            if cls != RunClass::Undecided {
                runs[i].cls = cls;
                changed = true;
            }
        }
    }
}

/// Rule (d): whole-comment prior, never applied to comments containing kana.
fn prior_class(text: &str, runs: &[HanziRun], comment_has_kana: bool) -> RunClass {
    if comment_has_kana {
        return RunClass::Ja;
    }
    let mut rest = String::with_capacity(text.len());
    let mut last = 0;
    for run in runs {
        if matches!(run.cls, RunClass::Ja | RunClass::Mixed) {
            rest.push_str(&text[last..run.start]);
            last = run.end;
        }
    }
    rest.push_str(&text[last..]);
    if is_chinese(&rest) {
        RunClass::Zh
    } else {
        RunClass::Ja
    }
}

/// Classify every maximal CJK run of `text` as zh / ja / mixed.
fn classify_runs(text: &str) -> Vec<HanziRun> {
    let mut runs = find_runs(text);
    let comment_has_kana = text.chars().any(is_japanese_script);
    for run in runs.iter_mut() {
        if run.cls == RunClass::Undecided {
            run.cls = classify_by_context(text, run, comment_has_kana);
        }
    }
    propagate_links(text, &mut runs);
    if runs.iter().any(|r| r.cls == RunClass::Undecided) {
        let prior = prior_class(text, &runs, comment_has_kana);
        for run in runs.iter_mut().filter(|r| r.cls == RunClass::Undecided) {
            run.cls = prior;
        }
    }
    runs
}

fn convert_hanzi_run(run: &str, out: &mut String) {
    for c in replace_taiwan_phrases(run).chars() {
        match lookup_char_sorted(CHINESE_HANZI_MAP, c) {
            Some(katakana) if is_hanzi(c) => out.push_str(katakana),
            _ => out.push(c),
        }
    }
}

/// Replace Chinese-context slang tokens such as `886` (bounded by non-alphanumerics).
fn replace_context_slang(text: &str) -> String {
    let mut result = text.to_string();
    for &(token, katakana) in ZH_CONTEXT_SLANG {
        let mut out = String::with_capacity(result.len());
        let mut last = 0;
        for (pos, _) in result.match_indices(token) {
            let end = pos + token.len();
            let bounded_before = !result[..pos]
                .chars()
                .next_back()
                .is_some_and(|c| c.is_ascii_alphanumeric());
            let bounded_after = !result[end..]
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_alphanumeric());
            if bounded_before && bounded_after {
                out.push_str(&result[last..pos]);
                out.push_str(katakana);
                last = end;
            }
        }
        out.push_str(&result[last..]);
        result = out;
    }
    result
}

fn normalize_chinese_punctuation(text: &str) -> String {
    let result = text.replace('，', "、");
    let result = PUNCT_SPACE_BEFORE_RE.replace_all(&result, "$1");
    let result = PUNCT_SPACE_AFTER_RE.replace_all(&result, "$1");
    MULTI_SPACE_RE.replace_all(&result, " ").trim().to_string()
}

/// Convert Chinese text to Katakana for Japanese TTS.
///
/// The unit of judgement is each maximal CJK ideograph run (issue #36):
/// only runs classified as Chinese are converted, so Japanese Kanji in a
/// mixed comment (e.g. `初見歓迎！ 谢谢`) are preserved.
pub fn convert_chinese(text: &str) -> String {
    let runs = classify_runs(text);
    if !runs.iter().any(|r| r.cls == RunClass::Zh) {
        // No Chinese run: only exact Taiwan phrase matches are replaced (as before).
        return replace_taiwan_phrases(text);
    }

    // Taiwan phrases are replaced first inside every run; only `zh` runs are
    // then converted character by character. `ja` and `mixed` runs keep their
    // Kanji untouched (Safe Kanji Guard / safe failure).
    let mut result = String::with_capacity(text.len() * 2);
    let mut last = 0;
    for run in &runs {
        result.push_str(&text[last..run.start]);
        let run_text = &text[run.start..run.end];
        if run.cls == RunClass::Zh {
            convert_hanzi_run(run_text, &mut result);
        } else {
            result.push_str(&replace_taiwan_phrases(run_text));
        }
        last = run.end;
    }
    result.push_str(&text[last..]);

    normalize_chinese_punctuation(&replace_context_slang(&result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kanji_guard() {
        // Safe Kanji Guard:
        assert!(!is_chinese("了解しました"));
        assert!(!is_chinese("初見歓迎です"));
        assert!(!is_chinese("神回"));
        assert!(!is_chinese("配信開始！"));
        assert!(!is_chinese("了解"));
        assert!(!is_chinese("初見歓迎"));
        assert!(!is_chinese("感謝"));
        assert!(!is_chinese("最高"));
        assert!(!is_chinese("優勝"));

        assert_eq!(convert_chinese("了解しました"), "了解しました");
        assert_eq!(convert_chinese("初見歓迎です"), "初見歓迎です");
        assert_eq!(convert_chinese("神回"), "神回");
        assert_eq!(convert_chinese("配信開始！"), "配信開始！");
    }

    #[test]
    fn test_chinese_conversion() {
        assert_eq!(convert_chinese("你好"), "ニーハオ");
        assert_eq!(convert_chinese("謝謝"), "シエシエ");
        assert_eq!(convert_chinese("笑死"), "シアオスー");
        assert_eq!(
            convert_chinese("我喜欢这个直播"),
            "ウォ喜ホアンジャーガージー播"
        );
    }

    #[test]
    fn test_spec_cases_chinese() {
        assert_eq!(
            convert_chinese("你好！玩得很好，加油！"),
            "ニーハオ！ワンドゥヘンハオ、ジャーヨウ！"
        );
        assert_eq!(
            convert_chinese("大家安安！實況主太強了，謝謝乾爹！"),
            "ダージアアンアン！シークアンジュータイチャンラ、シエシエガンディエ！"
        );
        assert_eq!(
            convert_chinese("這個是垃圾桶"),
            "ジャーガーシーレースートン"
        );
        assert_eq!(
            convert_chinese("實況主太厲害了吧"),
            "シークアンジュータイリーハイラバー"
        );
        assert_eq!(convert_chinese("謝謝乾爹"), "シエシエガンディエ");
        assert_eq!(convert_chinese("早安！"), "ザオアン！");
        assert_eq!(convert_chinese("晚安！"), "ワンアン！");
        assert_eq!(
            convert_chinese("笑死，太好笑了！"),
            "シアオスー、タイハオシアオラ！"
        );
        assert_eq!(
            convert_chinese("辛苦了！明天見！"),
            "シンクーラ！ミンティエンジエン！"
        );
        assert_eq!(
            convert_chinese("歡迎大家！請按讚！"),
            "ホワンインダージア！チンアンザン！"
        );
    }

    #[test]
    fn test_run_classification_mixed_comments() {
        // Issue #36: Japanese Kanji runs are kept, Simplified Chinese runs converted.
        assert_eq!(convert_chinese("初見歓迎！ 谢谢"), "初見歓迎！シエシエ");
        assert_eq!(
            convert_chinese("初見歓迎！ 886 谢谢大家"),
            "初見歓迎！バイバイ シエシエダージア"
        );
        assert_eq!(
            convert_chinese("初見です 谢谢大家"),
            "初見です シエシエダージア"
        );
        assert_eq!(convert_chinese("神回！谢谢"), "神回！シエシエ");
        // A run mixing Japanese-only and Simplified-only evidence is left as is.
        assert_eq!(convert_chinese("初見歓迎这个"), "初見歓迎这个");
        // `886` is only slang inside a Chinese comment.
        assert_eq!(convert_chinese("886"), "886");
        assert_eq!(convert_chinese("谢谢 8866"), "シエシエ 8866");
        // Full-width comma hints Chinese for Kanji shared by both languages.
        assert_eq!(
            convert_chinese("大家好，晚上好"),
            "ダージアハオ、ワンシャンハオ"
        );
        // Kana adjacency and `、` keep shared Kanji Japanese.
        assert_eq!(convert_chinese("初見です，大家好"), "初見です，大家好");
        assert_eq!(convert_chinese("大家好"), "大家好");
    }

    #[test]
    fn test_classification_tables_are_sorted_and_disjoint() {
        assert!(HANZI_SIMPLIFIED_ONLY.windows(2).all(|w| w[0] < w[1]));
        assert!(HANZI_JAPANESE_ONLY.windows(2).all(|w| w[0] < w[1]));
        assert!(!HANZI_JAPANESE_ONLY
            .iter()
            .any(|c| HANZI_SIMPLIFIED_ONLY.binary_search(c).is_ok()));
        for c in "们这说谢欢发东对么个时吗".chars() {
            assert!(HANZI_SIMPLIFIED_ONLY.binary_search(&c).is_ok(), "{c}");
        }
        for c in "歓対図売読気楽駅実県広沢険験釈択拡恵戦様総弁辺込畑峠辻働枠".chars()
        {
            assert!(HANZI_JAPANESE_ONLY.binary_search(&c).is_ok(), "{c}");
        }
        for c in "万台后大家好神回了解初見迎配信感謝最高優勝".chars() {
            assert!(HANZI_SIMPLIFIED_ONLY.binary_search(&c).is_err(), "{c}");
            assert!(HANZI_JAPANESE_ONLY.binary_search(&c).is_err(), "{c}");
        }
    }

    #[test]
    fn test_replace_taiwan_phrases() {
        assert_eq!(replace_taiwan_phrases("你好"), "ニーハオ");
        assert_eq!(replace_taiwan_phrases("謝謝"), "シエシエ");
        assert_eq!(replace_taiwan_phrases("笑死"), "シアオスー");
        assert_eq!(replace_taiwan_phrases("按讚"), "アンザン");
        assert_eq!(replace_taiwan_phrases("辛苦了"), "シンクーラ");
        assert_eq!(replace_taiwan_phrases("加油"), "ジャーヨウ");
    }
}
