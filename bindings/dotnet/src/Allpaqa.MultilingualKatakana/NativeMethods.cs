using System.Runtime.InteropServices;
#if NET7_0_OR_GREATER
using System.Runtime.CompilerServices;
#endif

namespace Allpaqa.MultilingualKatakana
{
    /// <summary>
    /// Raw P/Invoke surface of the C ABI in
    /// <c>crates/multilingual-katakana-ffi/include/allpaqa_multilingual_katakana.h</c>.
    /// All entry points use the C (cdecl) calling convention; <c>size_t</c> maps to <see cref="nuint"/>.
    /// </summary>
    internal static unsafe partial class NativeMethods
    {
        internal const string LibraryName = "allpaqa_multilingual_katakana";

        internal const uint ExpectedAbiVersion = 1;

        internal const int MkOk = 0;

        /// <summary>Frozen native layout (<c>MkOptions</c>): two <c>uint32_t</c> bit masks.</summary>
        [StructLayout(LayoutKind.Sequential)]
        internal struct MkOptions
        {
            internal uint FlagsSet;
            internal uint FlagsValue;
        }

#if NET7_0_OR_GREATER
        [LibraryImport(LibraryName, EntryPoint = "mk_abi_version")]
        [UnmanagedCallConv(CallConvs = new[] { typeof(CallConvCdecl) })]
        internal static partial uint MkAbiVersion();

        [LibraryImport(LibraryName, EntryPoint = "mk_to_katakana")]
        [UnmanagedCallConv(CallConvs = new[] { typeof(CallConvCdecl) })]
        internal static partial int MkToKatakana(byte* text, nuint textLen, MkOptions* options, byte** outPtr, nuint* outLen);

        [LibraryImport(LibraryName, EntryPoint = "mk_free_string")]
        [UnmanagedCallConv(CallConvs = new[] { typeof(CallConvCdecl) })]
        internal static partial void MkFreeString(byte* ptr, nuint len);

        [LibraryImport(LibraryName, EntryPoint = "mk_self_check")]
        [UnmanagedCallConv(CallConvs = new[] { typeof(CallConvCdecl) })]
        internal static partial int MkSelfCheck();
#else
        [DllImport(LibraryName, EntryPoint = "mk_abi_version", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern uint MkAbiVersion();

        [DllImport(LibraryName, EntryPoint = "mk_to_katakana", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern int MkToKatakana(byte* text, nuint textLen, MkOptions* options, byte** outPtr, nuint* outLen);

        [DllImport(LibraryName, EntryPoint = "mk_free_string", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern void MkFreeString(byte* ptr, nuint len);

        [DllImport(LibraryName, EntryPoint = "mk_self_check", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        internal static extern int MkSelfCheck();

        [DllImport("kernel32", EntryPoint = "LoadLibraryW", CharSet = CharSet.Unicode, ExactSpelling = true, SetLastError = true)]
        internal static extern System.IntPtr LoadLibraryW(string path);

        [DllImport("kernel32", EntryPoint = "GetModuleHandleW", CharSet = CharSet.Unicode, ExactSpelling = true)]
        internal static extern System.IntPtr GetModuleHandleW(string moduleName);
#endif
    }
}
