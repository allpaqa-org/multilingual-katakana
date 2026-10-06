using System;
using System.Text;

namespace Allpaqa.MultilingualKatakana
{
    /// <summary>
    /// Converts multilingual text (English, Chinese, Korean, Russian, Spanish,
    /// French, Vietnamese, Thai, streaming slang) into Japanese Katakana for
    /// Japanese TTS engines.
    /// </summary>
    /// <remarks>
    /// Unrecognized languages, emoji and symbols are always passed through
    /// unchanged, never dropped. All members are thread-safe.
    /// </remarks>
    public static class Katakana
    {
        /// <summary>Convert <paramref name="text"/> to Katakana.</summary>
        /// <param name="text">The input text.</param>
        /// <param name="options">Conversion options, or <see langword="null"/> for core defaults.</param>
        /// <returns>The converted text.</returns>
        /// <exception cref="ArgumentNullException"><paramref name="text"/> is <see langword="null"/>.</exception>
        /// <exception cref="KatakanaException">The native core could not be loaded or reported an error.</exception>
        public static string ToKatakana(string text, KatakanaOptions? options = null)
        {
            if (text == null)
            {
                throw new ArgumentNullException(nameof(text));
            }

            if (options == null)
            {
                return ConvertCore(text, null);
            }

            NativeMethods.MkOptions native = options.ToNative();
            return ConvertCore(text, native);
        }

        internal static unsafe string ConvertCore(string text, NativeMethods.MkOptions? options)
        {
            NativeLibraryInit.EnsureLoaded();

            byte[] input = Encoding.UTF8.GetBytes(text);
            NativeMethods.MkOptions opts = options.GetValueOrDefault();
            byte* outPtr = null;
            nuint outLen = 0;
            int status;
            fixed (byte* inPtr = input)
            {
                status = NativeMethods.MkToKatakana(inPtr, (nuint)input.Length, options.HasValue ? &opts : null, &outPtr, &outLen);
            }

            try
            {
                if (status != NativeMethods.MkOk)
                {
                    throw new KatakanaException($"Native conversion failed with status {status} ({Describe(status)}).", status);
                }

                if (outLen == 0)
                {
                    return string.Empty;
                }

                if (outLen > int.MaxValue)
                {
                    throw new KatakanaException("Native conversion produced an output larger than 2 GiB.");
                }

                return Encoding.UTF8.GetString(outPtr, (int)outLen);
            }
            finally
            {
                NativeMethods.MkFreeString(outPtr, outLen);
            }
        }

        private static string Describe(int status)
        {
            switch (status)
            {
                case 1:
                    return "null pointer";
                case 2:
                    return "invalid UTF-8";
                case 3:
                    return "panic in native core";
                default:
                    return "unknown error";
            }
        }
    }
}
