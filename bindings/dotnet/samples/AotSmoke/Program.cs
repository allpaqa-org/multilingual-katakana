using System;
using Allpaqa.MultilingualKatakana;

namespace AotSmoke
{
    internal static class Program
    {
        private static int Main()
        {
            string actual = Katakana.ToKatakana("hello");
            string viaConverter = new KatakanaConverter(new KatakanaOptions { EnableChinese = false }).Convert("你好");
            Console.WriteLine(actual);
            if (actual != "ハロー" || viaConverter != "你好")
            {
                Console.Error.WriteLine($"AOT smoke test FAILED: hello -> \"{actual}\", 你好 (Chinese off) -> \"{viaConverter}\"");
                return 1;
            }

            Console.WriteLine("AOT smoke test OK");
            return 0;
        }
    }
}
