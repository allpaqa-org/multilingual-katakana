use regex::Regex;
use std::sync::LazyLock;

use crate::dicts::{
    lookup_char_sorted, CHINESE_COMMON_WORDS, CHINESE_HANZI_MAP, CHINESE_MARKER_PATTERN,
    CHINESE_TAIWAN_PHRASES, HANZI_JAPANESE_ONLY, HANZI_ZH_EVIDENCE, JAPANESE_GUARD_WORDS,
    ZH_CONTEXT_SLANG,
};

static CHINESE_MARKER_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(CHINESE_MARKER_PATTERN).expect("Invalid CHINESE_MARKER_PATTERN"));

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
    /// True when the run itself carries Simplified-only / marker / phrase evidence.
    evidence: bool,
    /// Byte length of a Japanese guard word kept at the start / end of a `Zh` run.
    keep_head: usize,
    keep_tail: usize,
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
    matches!(c, '、' | '。' | '！' | '？' | '!' | '?' | '.' | '\n' | '\r')
}

/// Explicit horizontal whitespace class shared with the TypeScript pipeline
/// (no line breaks, no reliance on `char::is_whitespace` / `trim`).
#[inline]
fn is_hws(c: char) -> bool {
    matches!(
        c,
        '\t' | '\u{0B}' | '\u{0C}' | ' ' | '\u{A0}' | '\u{1680}' | '\u{2000}'
            ..='\u{200A}' | '\u{202F}' | '\u{205F}' | '\u{3000}'
    )
}

#[inline]
fn is_punct(c: char) -> bool {
    matches!(c, '、' | '。' | '！' | '？' | '!' | '?' | ',' | '.')
}

fn is_punct_only(part: &str) -> bool {
    !part.is_empty() && part.chars().all(is_punct)
}

fn has_char(text: &str, table: &[char]) -> bool {
    text.chars().any(|c| table.binary_search(&c).is_ok())
}

/// Longest guard word (in bytes) at the start (or end) of `run`, 0 when none.
fn guard_edge(run: &str, at_end: bool) -> usize {
    JAPANESE_GUARD_WORDS
        .iter()
        .filter(|&&w| {
            if at_end {
                run.ends_with(w)
            } else {
                run.starts_with(w)
            }
        })
        .map(|w| w.len())
        .max()
        .unwrap_or(0)
}

/// Evidence-based class of a run, from its own characters only.
/// Japanese-only characters always win (with Chinese evidence too -> mixed).
/// Strong Chinese evidence beats guard words; a guard word at the very start
/// or end of such a run is split off and kept as Japanese.
fn new_run(start: usize, text: &str, relax_guards: bool) -> HanziRun {
    let mut run = HanziRun {
        start,
        end: start + text.len(),
        cls: RunClass::Undecided,
        evidence: false,
        keep_head: 0,
        keep_tail: 0,
    };
    let zh = has_char(text, HANZI_ZH_EVIDENCE)
        || CHINESE_TAIWAN_PHRASES
            .iter()
            .any(|&(p, _)| text.contains(p));
    run.evidence = zh;
    if has_char(text, HANZI_JAPANESE_ONLY) {
        run.cls = if zh { RunClass::Mixed } else { RunClass::Ja };
    } else if zh {
        run.cls = RunClass::Zh;
        run.keep_head = guard_edge(text, false);
        run.keep_tail = if run.keep_head < text.len() {
            guard_edge(&text[run.keep_head..], true)
        } else {
            0
        };
    } else if JAPANESE_GUARD_WORDS.iter().any(|&w| text.contains(w)) {
        // In a kana-free comment that reads as Chinese, a run that itself shows
        // Chinese markers / common words is not forced Japanese by guard words.
        run.cls = if relax_guards && is_chinese(text) {
            RunClass::Undecided
        } else {
            RunClass::Ja
        };
    }
    run
}

fn find_runs(text: &str, relax_guards: bool) -> Vec<HanziRun> {
    let mut runs = Vec::new();
    let mut start: Option<usize> = None;
    for (i, c) in text.char_indices() {
        match (is_hanzi(c), start) {
            (true, None) => start = Some(i),
            (false, Some(s)) => {
                runs.push(new_run(s, &text[s..i], relax_guards));
                start = None;
            }
            _ => {}
        }
    }
    if let Some(s) = start {
        runs.push(new_run(s, &text[s..], relax_guards));
    }
    runs
}

/// Nearest non-horizontal-whitespace characters before `start` and after `end`.
fn nearest_chars(text: &str, start: usize, end: usize) -> (Option<char>, Option<char>) {
    let prev = text[..start].chars().rev().find(|&c| !is_hws(c));
    let next = text[end..].chars().find(|&c| !is_hws(c));
    (prev, next)
}

/// Rules (a) kana adjacency and (b) `、` hint for a SHARED-only run.
fn classify_by_context(text: &str, run: &HanziRun) -> RunClass {
    let adjacent_kana = text[..run.start]
        .chars()
        .next_back()
        .is_some_and(is_japanese_script)
        || text[run.end..]
            .chars()
            .next()
            .is_some_and(is_japanese_script);
    if adjacent_kana {
        return RunClass::Ja;
    }
    let (prev, next) = nearest_chars(text, run.start, run.end);
    if prev == Some('、') || next == Some('、') {
        return RunClass::Ja;
    }
    RunClass::Undecided
}

/// Two consecutive runs are linked when no kana / sentence break lies between them.
fn is_linked(text: &str, left: &HanziRun, right: &HanziRun) -> bool {
    !text[left.end..right.start]
        .chars()
        .any(|c| is_japanese_script(c) || is_sentence_break(c))
}

/// Class a run shows to its neighbour on one side (a kept guard edge is neutral).
fn edge_class(run: &HanziRun, cls: RunClass, right_side: bool) -> RunClass {
    let kept = if right_side {
        run.keep_tail
    } else {
        run.keep_head
    };
    if cls == RunClass::Zh && kept > 0 {
        RunClass::Undecided
    } else {
        cls
    }
}

/// A linked neighbour that is not Japanese can continue the Chinese sentence.
fn continues_sentence(text: &str, left: &HanziRun, right: &HanziRun, other: &HanziRun) -> bool {
    is_linked(text, left, right) && !matches!(other.cls, RunClass::Ja | RunClass::Mixed)
}

/// A guard word at the edge of a Chinese-evidence run is kept Japanese only
/// when nothing non-Japanese is linked on that side (`了解谢谢` keeps 了解,
/// `我是台灣人，感謝你們` converts 感謝 as part of the Chinese sentence).
fn settle_guard_edges(text: &str, runs: &mut [HanziRun]) {
    for i in 0..runs.len() {
        if runs[i].cls != RunClass::Zh {
            continue;
        }
        if i > 0 && continues_sentence(text, &runs[i - 1], &runs[i], &runs[i - 1]) {
            runs[i].keep_head = 0;
        }
        if i + 1 < runs.len() && continues_sentence(text, &runs[i], &runs[i + 1], &runs[i + 1]) {
            runs[i].keep_tail = 0;
        }
    }
}

fn touches_class(
    text: &str,
    runs: &[HanziRun],
    snapshot: &[RunClass],
    i: usize,
    target: RunClass,
) -> bool {
    let left = i > 0
        && is_linked(text, &runs[i - 1], &runs[i])
        && edge_class(&runs[i - 1], snapshot[i - 1], true) == target;
    let right = i + 1 < runs.len()
        && is_linked(text, &runs[i], &runs[i + 1])
        && edge_class(&runs[i + 1], snapshot[i + 1], false) == target;
    left || right
}

/// Spread `target` through linked undecided runs until a fixpoint (snapshot-based).
fn spread_class(text: &str, runs: &mut [HanziRun], target: RunClass) {
    let mut changed = true;
    while changed {
        let snapshot: Vec<RunClass> = runs.iter().map(|r| r.cls).collect();
        changed = false;
        for i in 0..runs.len() {
            if snapshot[i] == RunClass::Undecided && touches_class(text, runs, &snapshot, i, target)
            {
                runs[i].cls = target;
                changed = true;
            }
        }
    }
}

/// Rule (c): order-independent propagation, Japanese first (Safe Kanji Guard).
fn propagate_links(text: &str, runs: &mut [HanziRun]) {
    spread_class(text, runs, RunClass::Ja);
    spread_class(text, runs, RunClass::Zh);
}

/// Comment text without its ja / mixed runs.
fn text_without_japanese_runs(text: &str, runs: &[HanziRun]) -> String {
    let mut rest = String::with_capacity(text.len());
    let mut last = 0;
    for run in runs {
        if matches!(run.cls, RunClass::Ja | RunClass::Mixed) {
            rest.push_str(&text[last..run.start]);
            last = run.end;
        }
    }
    rest.push_str(&text[last..]);
    rest
}

fn beside_fullwidth_comma(text: &str, run: &HanziRun) -> bool {
    let (prev, next) = nearest_chars(text, run.start, run.end);
    prev == Some('，') || next == Some('，')
}

/// Rule (d): whole-comment fallback, never applied to comments containing kana.
/// `，` only breaks the tie when the comment has real Chinese evidence.
fn apply_fallback(text: &str, runs: &mut [HanziRun], comment_has_kana: bool) {
    if !runs.iter().any(|r| r.cls == RunClass::Undecided) {
        return;
    }
    let prior = !comment_has_kana && is_chinese(&text_without_japanese_runs(text, runs));
    let has_evidence =
        !comment_has_kana && runs.iter().any(|r| r.cls == RunClass::Zh && r.evidence);
    for run in runs.iter_mut().filter(|r| r.cls == RunClass::Undecided) {
        run.cls = if prior || (has_evidence && beside_fullwidth_comma(text, run)) {
            RunClass::Zh
        } else {
            RunClass::Ja
        };
    }
}

/// Classify every maximal CJK run of `text` as zh / ja / mixed.
fn classify_runs(text: &str) -> Vec<HanziRun> {
    let comment_has_kana = text.chars().any(is_japanese_script);
    let mut runs = find_runs(text, !comment_has_kana && is_chinese(text));
    for run in runs.iter_mut() {
        if run.cls == RunClass::Undecided {
            run.cls = classify_by_context(text, run);
        }
    }
    settle_guard_edges(text, &mut runs);
    propagate_links(text, &mut runs);
    apply_fallback(text, &mut runs, comment_has_kana);
    runs
}

fn convert_hanzi(text: &str, out: &mut String) {
    for c in replace_taiwan_phrases(text).chars() {
        match lookup_char_sorted(CHINESE_HANZI_MAP, c) {
            Some(katakana) if is_hanzi(c) => out.push_str(katakana),
            _ => out.push(c),
        }
    }
}

fn render_run(text: &str, run: &HanziRun, out: &mut String) {
    let run_text = &text[run.start..run.end];
    if run.cls != RunClass::Zh {
        out.push_str(&replace_taiwan_phrases(run_text));
        return;
    }
    let body_end = run_text.len() - run.keep_tail;
    out.push_str(&run_text[..run.keep_head]);
    convert_hanzi(&run_text[run.keep_head..body_end], out);
    out.push_str(&run_text[body_end..]);
}

#[inline]
fn is_digit(c: Option<char>) -> bool {
    c.is_some_and(|c| c.is_ascii_digit() || ('\u{FF10}'..='\u{FF19}').contains(&c))
}

/// A slang token is standalone unless glued to letters/digits, `+`, or `[.,:-]digit`.
fn blocks_slang(near: Option<char>, far: Option<char>) -> bool {
    match near {
        None => false,
        Some(c) if c.is_ascii_alphanumeric() || c == '+' || is_digit(Some(c)) => true,
        Some('.' | ',' | ':' | '-') => is_digit(far),
        Some(_) => false,
    }
}

/// Replace Chinese-context slang tokens such as `886`.
fn replace_context_slang(text: &str) -> String {
    let mut result = text.to_string();
    for &(token, katakana) in ZH_CONTEXT_SLANG {
        let mut out = String::with_capacity(result.len());
        let mut last = 0;
        for (pos, _) in result.match_indices(token) {
            let end = pos + token.len();
            let mut before = result[..pos].chars().rev();
            let mut after = result[end..].chars();
            let blocked = blocks_slang(before.next(), before.next())
                || blocks_slang(after.next(), after.next());
            if !blocked {
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

/// What lies at a gap edge: a converted Chinese run, another run, or the text boundary.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Edge {
    Zh,
    Run,
    Boundary,
}

/// Neighbour of a whitespace segment inside a gap.
#[derive(Clone, Copy)]
enum Side {
    Char(char),
    Edge(Edge),
}

/// Whitespace rule (only next to a converted run): removed between the run
/// and punctuation / the text boundary, collapsed to one space otherwise.
fn clean_whitespace(prev: Side, next: Side) -> Option<&'static str> {
    let punct = |s: Side| matches!(s, Side::Char(c) if is_punct(c));
    let is_edge = |s: Side, e: Edge| matches!(s, Side::Edge(x) if x == e);
    if is_edge(prev, Edge::Zh) && (punct(next) || is_edge(next, Edge::Boundary)) {
        return Some("");
    }
    if is_edge(next, Edge::Zh) && (punct(prev) || is_edge(prev, Edge::Boundary)) {
        return Some("");
    }
    if is_edge(prev, Edge::Zh) || is_edge(next, Edge::Zh) {
        Some(" ")
    } else {
        None
    }
}

/// Split `text` into alternating (non-whitespace, whitespace) segments,
/// starting and ending with a (possibly empty) non-whitespace segment.
fn split_hws(text: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut seg_start = 0;
    let mut in_ws = false;
    for (i, c) in text.char_indices() {
        if is_hws(c) != in_ws {
            parts.push(&text[seg_start..i]);
            seg_start = i;
            in_ws = !in_ws;
        }
    }
    parts.push(&text[seg_start..]);
    if in_ws {
        parts.push("");
    }
    parts
}

/// Normalize one gap between runs: `，` -> `、`, `886` slang, and horizontal
/// whitespace cleanup only next to a converted run (or punctuation attached to it).
/// Line breaks and spacing between non-CJK text are never touched.
fn normalize_gap(gap: &str, left: Edge, right: Edge, out: &mut String) {
    let gap = replace_context_slang(&gap.replace('，', "、"));
    let parts = split_hws(&gap);
    for (i, part) in parts.iter().enumerate() {
        if i % 2 == 0 {
            out.push_str(part);
            continue;
        }
        let prev = match parts[i - 1].chars().next_back() {
            Some(c) => Side::Char(c),
            None => Side::Edge(left),
        };
        let next = match parts[i + 1].chars().next() {
            Some(c) => Side::Char(c),
            None => Side::Edge(right),
        };
        // Punctuation directly attached to a converted run: `谢谢！ 大家` -> `谢谢！大家`.
        let bridged = (i == 1 && left == Edge::Zh && is_punct_only(parts[0]))
            || (i + 2 == parts.len() && right == Edge::Zh && is_punct_only(parts[i + 1]));
        if bridged {
            continue;
        }
        out.push_str(clean_whitespace(prev, next).unwrap_or(part));
    }
}

fn left_edge(run: Option<&HanziRun>) -> Edge {
    match run {
        None => Edge::Boundary,
        Some(r) if r.cls == RunClass::Zh && r.keep_tail == 0 => Edge::Zh,
        Some(_) => Edge::Run,
    }
}

fn right_edge(run: Option<&HanziRun>) -> Edge {
    match run {
        None => Edge::Boundary,
        Some(r) if r.cls == RunClass::Zh && r.keep_head == 0 => Edge::Zh,
        Some(_) => Edge::Run,
    }
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
    for i in 0..=runs.len() {
        let run = runs.get(i);
        let gap_end = run.map_or(text.len(), |r| r.start);
        let prev = if i > 0 { runs.get(i - 1) } else { None };
        normalize_gap(
            &text[last..gap_end],
            left_edge(prev),
            right_edge(run),
            &mut result,
        );
        if let Some(r) = run {
            render_run(text, r, &mut result);
            last = r.end;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dicts::HANZI_SIMPLIFIED_ONLY;

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
            "初見歓迎！ バイバイ シエシエダージア"
        );
        // A kept guard edge never spreads Japanese; Chinese sentences convert fully.
        assert_eq!(
            convert_chinese("我是台灣人，感謝你們"),
            "ウォシー台灣レン、ガンシエニー們"
        );
        assert_eq!(
            convert_chinese("謝謝 大家 感謝你們"),
            "シエシエ ダージア ガンシエニー們"
        );
        assert_eq!(
            convert_chinese("這個真的最高，大家好"),
            "ジャーガージェンドゥズイガオ、ダージアハオ"
        );
        // Japanese spacing next to full-width punctuation is kept.
        assert_eq!(
            convert_chinese("初見です！ よろしく 谢谢"),
            "初見です！ よろしく シエシエ"
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
        // `，` never decides a run by itself (review finding 1).
        for s in [
            "今日，最高",
            "東京，大阪",
            "乾杯，乾杯",
            "新曲，神曲",
            "本日，配信開始",
        ] {
            assert_eq!(convert_chinese(s), s);
        }
        assert_eq!(
            convert_chinese("大家好，晚上好"),
            "ダージアハオ、ワンシャンハオ"
        );
        assert_eq!(convert_chinese("初見です，大家好"), "初見です，大家好");
        // Strong Chinese evidence beats guard words; edge guard words are kept.
        assert_eq!(convert_chinese("了解谢谢"), "了解シエシエ");
        assert_eq!(convert_chinese("谢谢初見"), "シエシエ初見");
        assert_eq!(convert_chinese("我了解这个"), "ウォラジエジャーガー");
        // Cleanup only around converted runs / full-width punctuation.
        assert_eq!(
            convert_chinese("初見です 谢谢 Mr. Smith, hello."),
            "初見です シエシエ Mr. Smith, hello."
        );
        assert_eq!(
            convert_chinese("初見です 谢谢\nよろしく"),
            "初見です シエシエ\nよろしく"
        );
        // 886 boundaries.
        for s in [
            "+886 2 1234 谢谢",
            "886.5 谢谢",
            "8,886 谢谢",
            "８886 谢谢",
            "18860 谢谢",
        ] {
            assert!(convert_chinese(s).contains("886"), "{s}");
        }
        assert_eq!(convert_chinese("谢谢 886"), "シエシエ バイバイ");
        // Order-independent propagation.
        assert_eq!(
            convert_chinese("谢谢 大家 今日 最高"),
            "シエシエ 大家 今日 最高"
        );
        assert_eq!(
            convert_chinese("最高 今日 大家 谢谢"),
            "最高 今日 大家 シエシエ"
        );
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
