[assembly: Retry(3)]
[assembly: System.Diagnostics.CodeAnalysis.ExcludeFromCodeCoverage]

namespace ZenEngineTests;

public class GlobalHooks
{
    [Before(TestSession)]
    public static void SetUp()
    {
    }

    [After(TestSession)]
    public static void CleanUp()
    {
    }
}
