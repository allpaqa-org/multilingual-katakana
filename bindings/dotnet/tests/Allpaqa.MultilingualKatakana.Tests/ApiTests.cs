using System.Runtime.InteropServices;
using Xunit;

namespace Allpaqa.MultilingualKatakana.Tests;

public class ApiTests
{
    [Fact]
    public void DefaultConvertsEnglish()
    {
        Assert.Equal("ハロー", Katakana.ToKatakana("hello"));
    }

    [Fact]
    public void DisablingEnglishChangesOutput()
    {
        string disabled = Katakana.ToKatakana("hello", new KatakanaOptions { EnableEnglish = false });
        Assert.False(string.IsNullOrEmpty(disabled));
        Assert.NotEqual(Katakana.ToKatakana("hello"), disabled);
    }

    [Fact]
    public void DisablingChinesePassesThrough()
    {
        Assert.Equal("你好", Katakana.ToKatakana("你好", new KatakanaOptions { EnableChinese = false }));
    }

    [Fact]
    public void ExplicitTrueEqualsDefault()
    {
        var all = new KatakanaOptions
        {
            EnableCyrillic = true,
            EnableKorean = true,
            EnableChinese = true,
            EnableSpanish = true,
            EnableFrench = true,
            EnableVietnamese = true,
            EnableThai = true,
            EnableSlang = true,
            EnableEnglish = true,
            NormalizeProsody = true,
        };
        const string input = "Hello guys! GG WP 你好 안녕하세요 привет";
        Assert.Equal(Katakana.ToKatakana(input), Katakana.ToKatakana(input, all));
        Assert.Equal(Katakana.ToKatakana(input), Katakana.ToKatakana(input, new KatakanaOptions()));
    }

    [Theory]
    [InlineData("hello")]
    [InlineData("Hello guys! GG WP")]
    [InlineData("你好！谢谢乾爹")]
    public void ConverterMatchesToKatakana(string input)
    {
        var options = new KatakanaOptions { EnableEnglish = false, NormalizeProsody = false };
        Assert.Equal(Katakana.ToKatakana(input, options), new KatakanaConverter(options).Convert(input));
        Assert.Equal(Katakana.ToKatakana(input), new KatakanaConverter().Convert(input));
    }

    [Fact]
    public void ConverterSnapshotsOptions()
    {
        var options = new KatakanaOptions { EnableChinese = false };
        var converter = new KatakanaConverter(options);
        options.EnableChinese = true;
        Assert.Equal("你好", converter.Convert("你好"));
    }

    [Fact]
    public void NullTextThrows()
    {
        Assert.Throws<ArgumentNullException>(() => Katakana.ToKatakana(null!));
        Assert.Throws<ArgumentNullException>(() => new KatakanaConverter().Convert(null!));
    }

    [Fact]
    public void EmptyReturnsEmpty()
    {
        Assert.Equal(string.Empty, Katakana.ToKatakana(string.Empty));
        Assert.Equal(string.Empty, new KatakanaConverter().Convert(string.Empty));
    }

    [Fact]
    public void NonBmpAndEmojiArePreserved()
    {
        Assert.Equal("🎉", Katakana.ToKatakana("🎉"));
        Assert.Contains("🎉", Katakana.ToKatakana("hello 🎉 𠮷野家"));
        Assert.Contains("𠮷", Katakana.ToKatakana("hello 🎉 𠮷野家"));
    }

    [Fact]
    public void ConcurrentCallsAreConsistent()
    {
        string[] inputs = { "Hello guys! GG WP", "你好！谢谢乾爹", "안녕하세요", "привет", "¡Hola amigos!" };
        string[] expected = inputs.Select(i => Katakana.ToKatakana(i)).ToArray();
        var converter = new KatakanaConverter();
        var results = new string[2000];
        Parallel.For(0, results.Length, i =>
        {
            string input = inputs[i % inputs.Length];
            results[i] = i % 2 == 0 ? Katakana.ToKatakana(input) : converter.Convert(input);
        });
        for (int i = 0; i < results.Length; i++)
        {
            Assert.Equal(expected[i % inputs.Length], results[i]);
        }
    }

    [Fact]
    public void ProcessArchitectureMatchesExpectation()
    {
        // CI sets MK_EXPECT_ARCH on cross-architecture runs (win-x86, .NET Framework x86/x64) so a
        // silent fallback to the runner's native architecture fails loudly instead of passing.
        string? expected = Environment.GetEnvironmentVariable("MK_EXPECT_ARCH");
        if (string.IsNullOrEmpty(expected))
        {
            return;
        }

        Assert.Equal(expected, RuntimeInformation.ProcessArchitecture.ToString().ToLowerInvariant());
    }
}
