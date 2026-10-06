namespace Allpaqa.MultilingualKatakana
{
    /// <summary>
    /// Per-language toggles and prosody normalization for
    /// <see cref="Katakana.ToKatakana(string, KatakanaOptions?)"/> and
    /// <see cref="KatakanaConverter"/>.
    /// </summary>
    /// <remarks>
    /// Every property is a nullable <see cref="bool"/>: <see langword="null"/>
    /// (the default) keeps the native core's default, which is
    /// <see langword="true"/> (enabled) for every option.
    /// <para>
    /// The <c>exclude</c> option of the Node.js binding (user-defined
    /// literal/regex protection) is not yet supported by the .NET binding.
    /// </para>
    /// </remarks>
    public sealed class KatakanaOptions
    {
        /// <summary>Russian/Cyrillic transliteration. <see langword="null"/> keeps the core default (enabled).</summary>
        public bool? EnableCyrillic { get; set; }

        /// <summary>Korean Hangul decomposition. <see langword="null"/> keeps the core default (enabled).</summary>
        public bool? EnableKorean { get; set; }

        /// <summary>Mandarin Pinyin and Taiwan stream slang. <see langword="null"/> keeps the core default (enabled).</summary>
        public bool? EnableChinese { get; set; }

        /// <summary>Spanish accents, inverted punctuation and digraphs. <see langword="null"/> keeps the core default (enabled).</summary>
        public bool? EnableSpanish { get; set; }

        /// <summary>Curated French words and phrases. <see langword="null"/> keeps the core default (enabled).</summary>
        public bool? EnableFrench { get; set; }

        /// <summary>Vietnamese tone marks and phrases. <see langword="null"/> keeps the core default (enabled).</summary>
        public bool? EnableVietnamese { get; set; }

        /// <summary>Thai phrase mappings and open syllables. <see langword="null"/> keeps the core default (enabled).</summary>
        public bool? EnableThai { get; set; }

        /// <summary>Streaming/gaming slang (<c>gg</c>, <c>pog</c>, <c>afk</c>, ...). <see langword="null"/> keeps the core default (enabled).</summary>
        public bool? EnableSlang { get; set; }

        /// <summary>English loanwords and phonics. <see langword="null"/> keeps the core default (enabled).</summary>
        public bool? EnableEnglish { get; set; }

        /// <summary>Collapse Katakana word spacing and normalize punctuation. <see langword="null"/> keeps the core default (enabled).</summary>
        public bool? NormalizeProsody { get; set; }

        /// <summary>Snapshot these options into the frozen native layout.</summary>
        internal NativeMethods.MkOptions ToNative()
        {
            var o = default(NativeMethods.MkOptions);
            Apply(ref o, EnableCyrillic, 0);
            Apply(ref o, EnableKorean, 1);
            Apply(ref o, EnableChinese, 2);
            Apply(ref o, EnableSpanish, 3);
            Apply(ref o, EnableFrench, 4);
            Apply(ref o, EnableVietnamese, 5);
            Apply(ref o, EnableThai, 6);
            Apply(ref o, EnableSlang, 7);
            Apply(ref o, EnableEnglish, 8);
            Apply(ref o, NormalizeProsody, 9);
            return o;
        }

        private static void Apply(ref NativeMethods.MkOptions o, bool? value, int bit)
        {
            if (value is bool v)
            {
                uint mask = 1u << bit;
                o.FlagsSet |= mask;
                if (v)
                {
                    o.FlagsValue |= mask;
                }
            }
        }
    }
}
