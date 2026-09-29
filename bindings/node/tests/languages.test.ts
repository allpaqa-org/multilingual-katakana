import { describe, expect, test } from "bun:test";
import { toKatakana, vietnamesePreprocess } from "../src";

describe("Vietnamese and Thai conversion", () => {
  test("converts dictionary phrases and Thai open syllables", () => {
    expect(toKatakana("xin chào")).toBe("シンチャオ");
    expect(toKatakana("จ้า")).toBe("チャー");
  });

  describe("French dictionary conversion", () => {
    test("converts French greetings, liaison phrases, and thanks", () => {
      expect(toKatakana("Bonjour mon ami! Merci beaucoup.")).toBe(
        "ボンジュールモナミ！メルシーボクー。",
      );
      expect(toKatakana("Comment allez-vous?")).toBe("コマンタレヴ？");
    });

    test("converts the mixed-language demo sentence without French auto-detection", () => {
      expect(toKatakana("Bonjour mon ami! C'est un super stream, merci beaucoup.")).toBe(
        "ボンジュールモナミ！セタンシュペールストリーム、メルシーボクー。",
      );
    });

    test("can disable French dictionary mappings", () => {
      expect(
        toKatakana("merci beaucoup", {
          enableFrench: false,
          enableEnglish: false,
          enableSpanish: false,
          enableVietnamese: false,
          enableSlang: false,
        }),
      ).toBe("merci beaucoup");
    });
  });

  test("normalizes tone-marked vowels after Vietnamese digraphs", () => {
    expect(vietnamesePreprocess("nhà giá trẻ")).toBe("ニャ ジャ チェ");
    expect(vietnamesePreprocess("NHÀ GIÁ TRẺ")).toBe("ニャ ジャ チェ");
  });

  test("preserves ambiguous Thai closed syllables", () => {
    expect(toKatakana("คน")).toBe("คน");
  });

  test("respects the Vietnamese and Thai language options", () => {
    expect(
      toKatakana("xin chào", {
        enableVietnamese: false,
        enableEnglish: false,
        enableSpanish: false,
        enableSlang: false,
      }),
    ).toBe("xin chào");
    expect(toKatakana("สวัสดีครับ", { enableThai: false })).toBe("สวัสดีครับ");
  });
});
