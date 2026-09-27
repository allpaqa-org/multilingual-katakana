import englishWords from "../dicts/english_words.json";

export function getEnglishWord(word: string): string | undefined {
  return (englishWords as Record<string, string>)[word.toLowerCase()];
}

/**
 * Fallback phonics-based converter for words not in the pre-converted dictionary
 * (slang, character elongations like 'aaaaaaa', usernames, typos).
 */
export function phonicsToKatakana(rawWord: string): string {
  let word = rawWord.toLowerCase();

  // 1. Common suffixes
  word = word
    .replace(/tion/g, "ション")
    .replace(/sion/g, "ジョン")
    .replace(/ture/g, "チャー")
    .replace(/ight/g, "アイト")
    .replace(/ough/g, "オフ")
    .replace(/ing$/g, "イング")
    .replace(/ed$/g, "ド")
    .replace(/er$/g, "アー")
    .replace(/or$/g, "オー")
    .replace(/ar$/g, "アー")
    .replace(/al$/g, "アル");

  // 2. Digraphs & phonics combos
  word = word
    .replace(/ch/g, "チ")
    .replace(/sh/g, "シュ")
    .replace(/ph/g, "フ")
    .replace(/th/g, "ス")
    .replace(/wh/g, "ホ")
    .replace(/ck/g, "ック")
    .replace(/qu/g, "ク")
    .replace(/kn/g, "ナ")
    .replace(/wr/g, "ラ")
    .replace(/ee/g, "イー")
    .replace(/ea/g, "イー")
    .replace(/oo/g, "ウー")
    .replace(/ai/g, "エイ")
    .replace(/ay/g, "エイ")
    .replace(/oa/g, "オー")
    .replace(/ou/g, "アウ")
    .replace(/ow/g, "オウ");

  // 3. Consonant clusters
  word = word
    .replace(/str/g, "ストラ")
    .replace(/st/g, "スト")
    .replace(/sp/g, "スピ")
    .replace(/sk/g, "スク")
    .replace(/sm/g, "スメ")
    .replace(/sn/g, "スネ")
    .replace(/sw/g, "スワ")
    .replace(/gra/g, "グラ")
    .replace(/gri/g, "グリ")
    .replace(/gru/g, "グル")
    .replace(/gre/g, "グレ")
    .replace(/gro/g, "グロ")
    .replace(/gr/g, "グラ")
    .replace(/tra/g, "トラ")
    .replace(/tri/g, "トリ")
    .replace(/tru/g, "トゥルー")
    .replace(/tre/g, "トレ")
    .replace(/tro/g, "トロ")
    .replace(/tr/g, "トラ")
    .replace(/bra/g, "ブラ")
    .replace(/bri/g, "ブリ")
    .replace(/bru/g, "ブル")
    .replace(/bre/g, "ブレ")
    .replace(/bro/g, "ブロ")
    .replace(/br/g, "ブラ")
    .replace(/cra/g, "クラ")
    .replace(/cri/g, "クリ")
    .replace(/cru/g, "クル")
    .replace(/cre/g, "クレ")
    .replace(/cro/g, "クロ")
    .replace(/cr/g, "クラ")
    .replace(/dra/g, "ドラ")
    .replace(/dri/g, "ドリ")
    .replace(/dru/g, "ドラ")
    .replace(/dre/g, "ドレ")
    .replace(/dro/g, "ドロ")
    .replace(/dr/g, "ドラ")
    .replace(/pla/g, "プラ")
    .replace(/pli/g, "プリ")
    .replace(/plu/g, "プル")
    .replace(/ple/g, "プレ")
    .replace(/plo/g, "プロ")
    .replace(/pl/g, "プル")
    .replace(/fla/g, "フラ")
    .replace(/fli/g, "フリ")
    .replace(/flu/g, "フル")
    .replace(/fle/g, "フレ")
    .replace(/flo/g, "フロ")
    .replace(/fl/g, "フル");

  // 4. Silent E rule: vowel + consonant + e -> long vowel
  word = word
    .replace(/a([bcdfghjklmnpqrstvwxyz])e$/g, "エイ$1")
    .replace(/i([bcdfghjklmnpqrstvwxyz])e$/g, "アイ$1")
    .replace(/o([bcdfghjklmnpqrstvwxyz])e$/g, "オー$1")
    .replace(/u([bcdfghjklmnpqrstvwxyz])e$/g, "ユー$1");

  // 5. Standard CV syllables
  const cvTable: [RegExp, string][] = [
    [/kya/g, "キャ"],
    [/kyu/g, "キュ"],
    [/kyo/g, "キョ"],
    [/sha/g, "シャ"],
    [/shu/g, "シュ"],
    [/sho/g, "ショ"],
    [/shi/g, "シ"],
    [/cha/g, "チャ"],
    [/chu/g, "チュ"],
    [/cho/g, "チョ"],
    [/chi/g, "チ"],
    [/tsu/g, "ツ"],
    [/fa/g, "ファ"],
    [/fi/g, "フィ"],
    [/fe/g, "フェ"],
    [/fo/g, "フォ"],
    [/fu/g, "フ"],
    [/ja/g, "ジャ"],
    [/ju/g, "ジュ"],
    [/jo/g, "ジョ"],
    [/ji/g, "ジ"],
    [/va/g, "ヴァ"],
    [/vi/g, "ヴィ"],
    [/vu/g, "ヴ"],
    [/ve/g, "ヴェ"],
    [/vo/g, "ヴォ"],
    [/ka/g, "カ"],
    [/ki/g, "キ"],
    [/ku/g, "ク"],
    [/ke/g, "ケ"],
    [/ko/g, "コ"],
    [/sa/g, "サ"],
    [/si/g, "シ"],
    [/su/g, "ス"],
    [/se/g, "セ"],
    [/so/g, "ソ"],
    [/ta/g, "タ"],
    [/ti/g, "ティ"],
    [/tu/g, "トゥ"],
    [/te/g, "テ"],
    [/to/g, "ト"],
    [/na/g, "ナ"],
    [/ni/g, "ニ"],
    [/nu/g, "ヌ"],
    [/ne/g, "ネ"],
    [/no/g, "ノ"],
    [/ha/g, "ハ"],
    [/hi/g, "ヒ"],
    [/he/g, "ヘ"],
    [/ho/g, "ホ"],
    [/ma/g, "マ"],
    [/mi/g, "ミ"],
    [/mu/g, "ム"],
    [/me/g, "メ"],
    [/mo/g, "モ"],
    [/ya/g, "ヤ"],
    [/yu/g, "ユ"],
    [/yo/g, "ヨ"],
    [/ra/g, "ラ"],
    [/ri/g, "リ"],
    [/ru/g, "ル"],
    [/re/g, "レ"],
    [/ro/g, "ロ"],
    [/la/g, "ラ"],
    [/li/g, "リ"],
    [/lu/g, "ル"],
    [/le/g, "レ"],
    [/lo/g, "ロ"],
    [/wa/g, "ワ"],
    [/wo/g, "ウォ"],
    [/ga/g, "ガ"],
    [/gi/g, "ギ"],
    [/gu/g, "グ"],
    [/ge/g, "ゲ"],
    [/go/g, "ゴ"],
    [/za/g, "ザ"],
    [/zu/g, "ズ"],
    [/ze/g, "ゼ"],
    [/zo/g, "ゾ"],
    [/da/g, "ダ"],
    [/di/g, "ディ"],
    [/du/g, "ドゥ"],
    [/de/g, "デ"],
    [/do/g, "ド"],
    [/ba/g, "バ"],
    [/bi/g, "ビ"],
    [/bu/g, "ブ"],
    [/be/g, "ベ"],
    [/bo/g, "ボ"],
    [/pa/g, "パ"],
    [/pi/g, "ピ"],
    [/pu/g, "プ"],
    [/pe/g, "ペ"],
    [/po/g, "ポ"],
    [/ca/g, "カ"],
    [/cu/g, "ク"],
    [/co/g, "コ"],
    [/ci/g, "シ"],
    [/ce/g, "セ"],
    [/a/g, "ア"],
    [/i/g, "イ"],
    [/u/g, "ウ"],
    [/e/g, "エ"],
    [/o/g, "オ"],
  ];

  for (const [pattern, rep] of cvTable) {
    word = word.replace(pattern, rep);
  }

  // 6. Remaining consonants
  const consonantTable: [RegExp, string][] = [
    [/b/g, "ブ"],
    [/c/g, "ク"],
    [/d/g, "ド"],
    [/f/g, "フ"],
    [/g/g, "グ"],
    [/h/g, "ハ"],
    [/j/g, "ジ"],
    [/k/g, "ク"],
    [/l/g, "ル"],
    [/m/g, "ム"],
    [/n/g, "ン"],
    [/p/g, "プ"],
    [/q/g, "ク"],
    [/r/g, "ル"],
    [/s/g, "ス"],
    [/t/g, "ト"],
    [/v/g, "ヴ"],
    [/w/g, "ウ"],
    [/x/g, "クス"],
    [/y/g, "イ"],
    [/z/g, "ズ"],
  ];

  for (const [pattern, rep] of consonantTable) {
    word = word.replace(pattern, rep);
  }

  word = word.replace(/[a-z]/g, "");
  return word || rawWord;
}
