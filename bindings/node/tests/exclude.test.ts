import { describe, expect, test } from "bun:test";
import { escapeExcluded, KatakanaConverter, restoreExcluded, toKatakana } from "../src/index";

describe("Text Exclusion / Protection (options.exclude)", () => {
  test("String exclusion: protects specific words from conversion", () => {
    const input = "hello bot nice to meet you";
    const actual = toKatakana(input, { exclude: ["bot", "nice"] });
    expect(actual).toBe("ハロー bot nice トゥーミートユー");
  });

  test("Regex exclusion for URL: protects web URLs", () => {
    const input = "check https://example.com bro";
    const actual = toKatakana(input, { exclude: [/https?:\/\/\S+/] });
    expect(actual).toBe("チェック https://example.com ブロ");
  });

  test("Regex exclusion for mention: protects user handles / mentions", () => {
    const input = "hello @streamer_123 gg";
    const actual = toKatakana(input, { exclude: [/@\w+/] });
    expect(actual).toBe("ハロー @streamer_123 ジージー");
  });

  test("Mixed languages with exclusions: Korean, Spanish, Slang, English", () => {
    const input = "안녕하세요 @streamer_123 gracias bro";
    const actual = toKatakana(input, { exclude: [/@\w+/] });
    expect(actual).toBe("アンニョンハセヨ @streamer_123 グラシアスブロ");
  });

  test("Mixed languages with exclusions: Chinese and URL", () => {
    const input = "你好 https://twitch.tv/example streamer";
    const actual = toKatakana(input, { exclude: [/https?:\/\/\S+/] });
    expect(actual).toBe("ニーハオ https://twitch.tv/example ストリーマー");
  });

  test("Mixed languages with exclusions: Russian and protected token", () => {
    const input = "привет BOT_V1 hello";
    const actual = toKatakana(input, { exclude: ["BOT_V1"] });
    expect(actual).toBe("プリヴィエト BOT_V1 ハロー");
  });

  test("Multiple occurrences of the same excluded word", () => {
    const input = "bot hello bot and bot";
    const actual = toKatakana(input, { exclude: ["bot"] });
    expect(actual).toBe("bot ハロー bot アンド bot");
  });

  test("KatakanaConverter class instance with default exclude option", () => {
    const converter = new KatakanaConverter({ exclude: ["bot"] });
    const actual = converter.convert("hello bot nice");
    expect(actual).toBe("ハロー bot ナイス");
  });

  test("Empty or undefined exclude behaves as normal", () => {
    expect(toKatakana("hello bot", { exclude: [] })).toBe("ハローボット");
    expect(toKatakana("hello bot", {})).toBe("ハローボット");
  });

  test("Katakana token exclusion (preserves Japanese word without collapsing surrounding text)", () => {
    const input = "hello ワラ world";
    const actual = toKatakana(input, { exclude: ["ワラ"] });
    expect(actual).toBe("ハロー ワラ ワールド");
  });

  test("Punctuation-rich token exclusion (preserves dots, plus, and code tokens)", () => {
    const input = "check C++ and node.js now!";
    const actual = toKatakana(input, { exclude: ["C++", "node.js"] });
    expect(actual).toBe("チェック C++ アンド node.js ナウ！");
  });

  test("Case-sensitive string exclusion (preserves exact case, other cases converted)", () => {
    const input = "hello Bot and game bot";
    const actual = toKatakana(input, { exclude: ["Bot"] });
    expect(actual).toBe("ハロー Bot アンドゲームボット");
  });

  test("escapeExcluded and restoreExcluded direct helpers (tokenMap & Record)", () => {
    const { text, tokenMap } = escapeExcluded("hello test world", ["test"]);
    expect(tokenMap.size).toBe(1);
    expect(text).toContain(String.fromCharCode(0xe000));

    const restoredFromMap = restoreExcluded(text, tokenMap);
    expect(restoredFromMap).toBe("hello test world");

    const record = Object.fromEntries(tokenMap);
    const restoredFromRecord = restoreExcluded(text, record);
    expect(restoredFromRecord).toBe("hello test world");
  });

  test("Direct helpers preserve pre-existing PUA when using tokenMap", () => {
    const input = "\uE000 hello test";
    const { text, tokenMap } = escapeExcluded(input, ["test"]);
    // Since \uE000 was in input, \uE001 is allocated for 'test'
    expect(text).toBe("\uE000 hello \uE001");
    expect(tokenMap.get("\uE001")).toBe("test");
    expect(tokenMap.has("\uE000")).toBe(false);

    const restored = restoreExcluded(text, tokenMap);
    expect(restored).toBe("\uE000 hello test");
  });

  test("Pre-existing PUA character in input is strictly preserved", () => {
    const puaChar = "\uE000";
    const input = `${puaChar} hello bot`;
    const actual = toKatakana(input, { exclude: ["bot"] });
    expect(actual).toBe(`${puaChar} ハロー bot`);
  });

  test("Multiple pre-existing PUA characters don't collide with allocated placeholders", () => {
    const input = "\uE000 \uE001 \uE002 test bot";
    const actual = toKatakana(input, { exclude: ["bot"] });
    expect(actual).toBe("\uE000 \uE001 \uE002 テスト bot");
  });

  test("Subsequent broad regex does not swallow earlier placeholders", () => {
    const input = "special prize";
    // Even if /\S+/ is present in exclude along with "special",
    // intervals are determined on original text so tokens are not swallowed
    const actual = toKatakana(input, { exclude: ["special", /\S+/] });
    expect(actual).toBe("special prize");
  });

  test("Overlapping exclusion rules prefer longest match first", () => {
    const input = "hello world gamer";
    const actual1 = toKatakana(input, { exclude: ["hello", "hello world"] });
    expect(actual1).toBe("hello world ゲーマー");

    const actual2 = toKatakana(input, { exclude: ["hello world", "hello"] });
    expect(actual2).toBe("hello world ゲーマー");
  });

  test("Zero-width regex is safely ignored without errors or infinite loops", () => {
    const input = "hello bot world";
    const actual = toKatakana(input, { exclude: [/(?=bot)/, "bot"] });
    expect(actual).toBe("ハロー bot ワールド");
  });

  test("Adjacent excluded tokens are correctly preserved", () => {
    const input = "foo bar baz";
    const actual = toKatakana(input, { exclude: ["foo", "bar"] });
    expect(actual).toBe("foo bar バズ");
  });

  test("Throws an explicit error when PUA codepoints are completely exhausted", () => {
    const allPuaChars: string[] = [];
    for (let code = 0xe000; code <= 0xf8ff; code++) {
      allPuaChars.push(String.fromCharCode(code));
    }
    const input = `${allPuaChars.join("")} hello bot`;
    expect(() => {
      toKatakana(input, { exclude: ["bot"] });
    }).toThrow("Exceeded maximum number of protectable tokens in PUA range");
  });
});
