using System;

namespace Allpaqa.MultilingualKatakana
{
    /// <summary>
    /// Thrown when the native core cannot be loaded (unsupported platform,
    /// missing or mismatched native library) or reports an error.
    /// </summary>
    public sealed class KatakanaException : Exception
    {
        /// <summary>Native status code for conversion failures, or <see langword="null"/> for load/initialization failures.</summary>
        /// <remarks>1 = null pointer, 2 = invalid UTF-8, 3 = panic caught in the native core.</remarks>
        public int? StatusCode { get; }

        /// <summary>Create an exception with a message.</summary>
        /// <param name="message">The error message.</param>
        public KatakanaException(string message)
            : base(message)
        {
        }

        /// <summary>Create an exception with a message and an inner exception.</summary>
        /// <param name="message">The error message.</param>
        /// <param name="innerException">The underlying cause, if any.</param>
        public KatakanaException(string message, Exception? innerException)
            : base(message, innerException)
        {
        }

        /// <summary>Create an exception for a native status code.</summary>
        /// <param name="message">The error message.</param>
        /// <param name="statusCode">The native status code returned by the core.</param>
        public KatakanaException(string message, int statusCode)
            : base(message)
        {
            StatusCode = statusCode;
        }
    }
}
