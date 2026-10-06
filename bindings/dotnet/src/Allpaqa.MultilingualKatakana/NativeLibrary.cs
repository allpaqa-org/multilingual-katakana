using System;
using System.IO;
using System.Runtime.InteropServices;

namespace Allpaqa.MultilingualKatakana
{
    /// <summary>
    /// One-time, thread-safe native library initialization with fail-fast
    /// semantics. Deliberately not a static constructor so that a load
    /// failure surfaces as a clear <see cref="KatakanaException"/> (on every
    /// call) rather than a <see cref="TypeInitializationException"/>.
    /// </summary>
    internal static class NativeLibraryInit
    {
        private sealed class Failure
        {
            internal Failure(string message, Exception? inner)
            {
                Message = message;
                Inner = inner;
            }

            internal string Message { get; }

            internal Exception? Inner { get; }
        }

        private static readonly Lazy<Failure?> State =
            new Lazy<Failure?>(Initialize, System.Threading.LazyThreadSafetyMode.ExecutionAndPublication);

        /// <summary>Ensure the native library is loaded and healthy, or throw the cached failure.</summary>
        internal static void EnsureLoaded()
        {
            Failure? failure = State.Value;
            if (failure != null)
            {
                // A fresh instance per call keeps stack traces meaningful; message and cause are identical.
                throw new KatakanaException(failure.Message, failure.Inner);
            }
        }

        private static Failure? Initialize()
        {
            try
            {
#if !NET7_0_OR_GREATER
                PreloadWindowsArchitectureSubfolder();
#endif
                uint abi = NativeMethods.MkAbiVersion();
                if (abi != NativeMethods.ExpectedAbiVersion)
                {
                    return new Failure(
                        $"Native library '{NativeMethods.LibraryName}' has ABI version {abi}, but this assembly requires {NativeMethods.ExpectedAbiVersion}. "
                        + "Make sure the native library and the Allpaqa.MultilingualKatakana assembly come from the same package version.",
                        null);
                }

                if (NativeMethods.MkSelfCheck() != 1)
                {
                    return new Failure(
                        $"Native library '{NativeMethods.LibraryName}' failed its self check on {DescribePlatform()}.",
                        null);
                }

                return null;
            }
            catch (Exception ex) when (ex is DllNotFoundException || ex is EntryPointNotFoundException || ex is BadImageFormatException)
            {
                return new Failure(
                    $"Failed to load native library '{NativeMethods.LibraryName}' ({ex.GetType().Name}): unsupported platform or missing native asset for {DescribePlatform()}. "
                    + "Supported RIDs: win-x64, win-x86, win-arm64, linux-x64, linux-arm64, linux-musl-x64, linux-musl-arm64, osx-x64, osx-arm64. "
                    + "On .NET Framework, the native DLL must be in an x86\\, x64\\ or arm64\\ subfolder next to the application (copied automatically by the NuGet package's build targets).",
                    ex);
            }
        }

        private static string ArchitectureName()
        {
            switch (RuntimeInformation.ProcessArchitecture)
            {
                case Architecture.X86:
                    return "x86";
                case Architecture.X64:
                    return "x64";
                case Architecture.Arm64:
                    return "arm64";
                case Architecture.Arm:
                    return "arm";
                default:
                    return RuntimeInformation.ProcessArchitecture.ToString().ToLowerInvariant();
            }
        }

        private static string DescribePlatform()
        {
            string os = RuntimeInformation.IsOSPlatform(OSPlatform.Windows) ? "win"
                : RuntimeInformation.IsOSPlatform(OSPlatform.OSX) ? "osx"
                : RuntimeInformation.IsOSPlatform(OSPlatform.Linux) ? "linux"
                : "unknown";
            return $"RID {os}-{ArchitectureName()} ({RuntimeInformation.OSDescription}, {RuntimeInformation.FrameworkDescription})";
        }

#if !NET7_0_OR_GREATER
        /// <summary>
        /// .NET Framework does not resolve <c>runtimes/{rid}/native</c>, so the package's
        /// build targets copy the Windows DLLs into <c>x86\</c>/<c>x64\</c>/<c>arm64\</c>
        /// subfolders. Preload the one matching the process architecture (if it is not
        /// already loaded) so the <c>[DllImport]</c> by name binds to it.
        /// </summary>
        private static void PreloadWindowsArchitectureSubfolder()
        {
            // .NET Core / .NET 5+ consuming the netstandard2.0 build resolve
            // runtimes/{rid}/native themselves; only .NET Framework needs help.
            if (!RuntimeInformation.IsOSPlatform(OSPlatform.Windows)
                || !RuntimeInformation.FrameworkDescription.StartsWith(".NET Framework", StringComparison.Ordinal))
            {
                return;
            }

            const string fileName = NativeMethods.LibraryName + ".dll";
            if (NativeMethods.GetModuleHandleW(fileName) != IntPtr.Zero)
            {
                return;
            }

            string arch = ArchitectureName();
            foreach (string? baseDir in new[] { AssemblyDirectory(), AppDomain.CurrentDomain.BaseDirectory })
            {
                if (string.IsNullOrEmpty(baseDir))
                {
                    continue;
                }

                string candidate = Path.Combine(Path.Combine(baseDir, arch), fileName);
                if (File.Exists(candidate) && NativeMethods.LoadLibraryW(candidate) != IntPtr.Zero)
                {
                    return;
                }
            }

            // Not found in a subfolder: fall back to the default DLL search order
            // (e.g. the DLL next to the executable, or runtimes/ on .NET Core).
        }

        private static string? AssemblyDirectory()
        {
            try
            {
                string location = typeof(NativeLibraryInit).Assembly.Location;
                return string.IsNullOrEmpty(location) ? null : Path.GetDirectoryName(location);
            }
            catch (NotSupportedException)
            {
                return null;
            }
        }
#endif
    }
}
