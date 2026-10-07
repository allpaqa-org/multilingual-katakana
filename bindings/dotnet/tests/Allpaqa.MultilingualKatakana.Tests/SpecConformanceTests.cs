using System.Text.Json;
using Xunit;

namespace Allpaqa.MultilingualKatakana.Tests;

public class SpecConformanceTests
{
    private static readonly Lazy<IReadOnlyList<object[]>> Cases = new(LoadCases);

    public static IEnumerable<object[]> AllCases() => Cases.Value;

    [Fact]
    public void LoadsAllSpecCases()
    {
        Assert.Equal(228, Cases.Value.Count);
    }

    [Theory]
    [MemberData(nameof(AllCases))]
    public void MatchesCanonical(string id, string input, string expected, string[] accepted, string optionsJson)
    {
        // Options travel as raw JSON so xUnit can serialize (and list) every case individually.
        // Same contract as the Rust core / Python harnesses: canonical, or one of `accepted`.
        Assert.False(string.IsNullOrEmpty(id));
        KatakanaOptions? options = optionsJson.Length == 0 ? null : MapOptions(JsonDocument.Parse(optionsJson).RootElement);
        string actual = Katakana.ToKatakana(input, options);
        if (!accepted.Contains(actual, StringComparer.Ordinal))
        {
            Assert.Equal(expected, actual);
        }
    }

    private static string FindSpecDir()
    {
        for (var dir = new DirectoryInfo(AppContext.BaseDirectory); dir != null; dir = dir.Parent)
        {
            string candidate = Path.Combine(dir.FullName, "spec", "cases");
            if (Directory.Exists(candidate))
            {
                return candidate;
            }
        }

        throw new DirectoryNotFoundException("Could not locate spec/cases above " + AppContext.BaseDirectory);
    }

    private static IReadOnlyList<object[]> LoadCases()
    {
        var result = new List<object[]>();
        foreach (string file in Directory.GetFiles(FindSpecDir(), "*.json").OrderBy(f => f, StringComparer.Ordinal))
        {
            using JsonDocument doc = JsonDocument.Parse(File.ReadAllText(file));
            foreach (JsonElement c in doc.RootElement.EnumerateArray())
            {
                string optionsJson = c.TryGetProperty("options", out JsonElement o) ? o.GetRawText() : string.Empty;
                (string canonical, string[] accepted) = ParseExpected(c.GetProperty("expected"));
                result.Add(new object[]
                {
                    c.GetProperty("id").GetString()!,
                    c.GetProperty("input").GetString()!,
                    canonical,
                    accepted,
                    optionsJson,
                });
            }
        }

        return result;
    }

    private static (string Canonical, string[] Accepted) ParseExpected(JsonElement e)
    {
        if (e.ValueKind == JsonValueKind.String)
        {
            return (e.GetString()!, Array.Empty<string>());
        }

        string canonical = e.TryGetProperty("canonical", out JsonElement c) ? c.GetString() ?? string.Empty : string.Empty;
        string[] accepted = e.TryGetProperty("accepted", out JsonElement a)
            ? a.EnumerateArray().Select(x => x.GetString()!).ToArray()
            : Array.Empty<string>();
        return (canonical, accepted);
    }

    private static KatakanaOptions MapOptions(JsonElement o)
    {
        bool? Get(string name) => o.TryGetProperty(name, out JsonElement v) && v.ValueKind is JsonValueKind.True or JsonValueKind.False
            ? v.GetBoolean()
            : null;

        return new KatakanaOptions
        {
            EnableCyrillic = Get("enableCyrillic"),
            EnableKorean = Get("enableKorean"),
            EnableChinese = Get("enableChinese"),
            EnableSpanish = Get("enableSpanish"),
            EnableFrench = Get("enableFrench"),
            EnableVietnamese = Get("enableVietnamese"),
            EnableThai = Get("enableThai"),
            EnableSlang = Get("enableSlang"),
            EnableEnglish = Get("enableEnglish"),
            NormalizeProsody = Get("normalizeProsody"),
        };
    }
}
