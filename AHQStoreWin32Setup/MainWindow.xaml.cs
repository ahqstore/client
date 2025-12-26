using System.Diagnostics;
using System.Windows;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Markup;
using System.Windows.Media;
using System.Windows.Media.Animation;
using System.Windows.Navigation;
using MdXaml;

namespace AHQStoreWin32Setup;

/// <summary>
/// Interaction logic for MainWindow.xaml
/// </summary>
public partial class MainWindow : Window
{
    Markdown engine;

    public MainWindow()
    {
        InitializeComponent();

        LicenseText.Text = """"""
MIT License

Copyright(c) 2024 AHQ Softwares

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files(the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/ or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
"""""";

        engine = new Markdown();

        var h1 = new Style();
        h1.Setters.Add(new Setter(FlowDocument.FontFamilyProperty, new FontFamily("Segoe UI Variable Text")));
        h1.Setters.Add(new Setter(FlowDocument.FontSizeProperty, 24.0));
        h1.Setters.Add(new Setter(FlowDocument.FontWeightProperty, FontWeights.Bold));
        engine.Heading1Style = h1;

        var h2 = new Style();
        h2.Setters.Add(new Setter(FlowDocument.FontFamilyProperty, new FontFamily("Segoe UI Variable Text")));
        h2.Setters.Add(new Setter(FlowDocument.FontSizeProperty, 20.0));
        h2.Setters.Add(new Setter(FlowDocument.FontWeightProperty, FontWeights.Bold));
        engine.Heading2Style = h2;

        var link = new Style();
        link.Setters.Add(new Setter(FlowDocument.ForegroundProperty, new SolidColorBrush(Color.FromRgb(0, 120, 212))));
        engine.LinkStyle = link;

        //         PrivacyPolicyMarkdown.Markdown = """
        // # Hello World
        // This is our privacy policy
        // """;

        this.CommandBindings.Add(new CommandBinding(NavigationCommands.GoToPage, (s, e) =>
        {
            Process.Start(new ProcessStartInfo(e.Parameter.ToString()!) { UseShellExecute = true });
        }));

        _ = Setup();

        Activate();
    }

    private async Task Setup()
    {
        GitHubService service = new GitHubService();

        await service.Setup();
        string tos = await service.FetchTOS();

        FlowDocument tosFlow = engine.Transform(tos);

        foreach (var doc in tosFlow.Blocks.ToList())
        {
            TermsOfService.Blocks.Add(doc);
        }

        string pp = await service.FetchPP();

        FlowDocument ppFlow = engine.Transform(tos);

        foreach (var doc in tosFlow.Blocks.ToList())
        {
            PrivacyPolicyMarkdown.Blocks.Add(doc);
        }

    }

    private void Open_License(object sender, RoutedEventArgs e)
    {
        Welcome.Visibility = Visibility.Collapsed;
        License.Visibility = Visibility.Visible;

        Storyboard sb = (Storyboard)this.FindResource("SlideAndFadeIn");
        sb.Begin(License);

    }

    private async void Open_Terms(object sender, RoutedEventArgs e)
    {
        License.Visibility = Visibility.Collapsed;
        Terms.Visibility = Visibility.Visible;

        Storyboard sb = (Storyboard)this.FindResource("SlideAndFadeIn");
        sb.Begin(Terms);
    }

    private async void Launch_PP(object sender, RoutedEventArgs e)
    {
        Terms.Visibility = Visibility.Collapsed;
        PrivacyPolicy.Visibility = Visibility.Visible;

        Storyboard sb = (Storyboard)this.FindResource("SlideAndFadeIn");
        sb.Begin(PrivacyPolicy);
    }

    private async void ArchitectureSelect(object sender, RoutedEventArgs e)
    {
        PrivacyPolicy.Visibility = Visibility.Collapsed;
        Configuration.Visibility = Visibility.Visible;

        Storyboard sb = (Storyboard)this.FindResource("SlideAndFadeIn");
        sb.Begin(Configuration);
    }


    protected override void OnSourceInitialized(EventArgs e)
    {
        base.OnSourceInitialized(e);

        Welcome.Visibility = Visibility.Visible;

        Storyboard sb = (Storyboard)this.FindResource("SlideAndFadeIn");
        sb.Begin(Welcome);
    }
}