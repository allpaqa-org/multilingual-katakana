using System;

namespace Allpaqa.MultilingualKatakana
{
    /// <summary>
    /// Reusable converter bound to a fixed set of options, for callers that
    /// convert many strings with the same configuration. Thread-safe.
    /// </summary>
    public sealed class KatakanaConverter
    {
        private readonly NativeMethods.MkOptions? _options;

        /// <summary>Create a converter.</summary>
        /// <param name="options">
        /// Conversion options, or <see langword="null"/> for core defaults. The
        /// options are snapshotted here; later changes to the instance have no effect.
        /// </param>
        public KatakanaConverter(KatakanaOptions? options = null)
        {
            _options = options?.ToNative();
        }

        /// <summary>Convert <paramref name="text"/> to Katakana using this converter's options.</summary>
        /// <param name="text">The input text.</param>
        /// <returns>The converted text.</returns>
        /// <exception cref="ArgumentNullException"><paramref name="text"/> is <see langword="null"/>.</exception>
        /// <exception cref="KatakanaException">The native core could not be loaded or reported an error.</exception>
        public string Convert(string text)
        {
            if (text == null)
            {
                throw new ArgumentNullException(nameof(text));
            }

            return Katakana.ConvertCore(text, _options);
        }
    }
}
