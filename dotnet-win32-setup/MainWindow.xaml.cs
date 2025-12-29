using System.Diagnostics;
using System.Timers;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Animation;
using MdXaml;
using Microsoft.Win32;

namespace AHQStoreWin32Setup;

/// <summary>
/// Interaction logic for MainWindow.xaml
/// </summary>
public partial class MainWindow : Window
{
    public bool installEnabled
    {
        get { return (bool)GetValue(IsButtonEnabledProperty); }
        set { SetValue(IsButtonEnabledProperty, value); }
    }

    public string ArchStatusText
    {
        get { return (string)GetValue(StatusTxtProperty); }
        set { SetValue(StatusTxtProperty, value); }
    }

    public string installingStatusText
    {
        get { return (string)GetValue(InstallerStatusTextProperty); }
        set { SetValue(InstallerStatusTextProperty, value); }
    }



    public static readonly DependencyProperty StatusTxtProperty =
        DependencyProperty.Register(
            "statusText",
            typeof(string),
            typeof(MainWindow),
            new PropertyMetadata("Hang tight!"));

    public static readonly DependencyProperty IsButtonEnabledProperty =
            DependencyProperty.Register(
                "installEnabled",
                typeof(bool),
                typeof(MainWindow),
                new PropertyMetadata(false));

    public static readonly DependencyProperty InstallerStatusTextProperty = DependencyProperty.Register(
        "installingStatusText",
        typeof(string),
        typeof(MainWindow),
        new PropertyMetadata("Downloading..."));

    Markdown engine;
    GitHubService service;

    private readonly PeriodicTimer _timer;
    private CancellationTokenSource _cts = new();

    Urls urls;

    public MainWindow()
    {
        _timer = new PeriodicTimer(TimeSpan.FromSeconds(0.5));

        InitializeComponent();

        Resources["NormalTextBrush"] = Resources["TextFillColorTertiaryBrush"];

        if (OperatingSystem.IsWindowsVersionAtLeast(10, 0, 22000, 0))
        {
            Background = Brushes.Transparent;
        }

        service = new GitHubService();
        urls = new Urls();

        this.DataContext = this;

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

        this.CommandBindings.Add(new CommandBinding(NavigationCommands.GoToPage, (s, e) =>
        {
            Process.Start(new ProcessStartInfo(e.Parameter.ToString()!) { UseShellExecute = true });
        }));

        _ = StartThemeMgr();

        Activate();
    }

    public async Task StartThemeMgr()
    {
        try
        {

            // This loop repeats every interval
            while (await _timer.WaitForNextTickAsync(_cts.Token))
            {
                // Mutate 'this' (the class instance state)
                using (var key = Registry.CurrentUser.OpenSubKey(@"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"))
                {
                    int registryValueObject = (int)key?.GetValue("AppsUseLightTheme")!;

                    var dark = registryValueObject == 0;

                    if (dark)
                    {
                        Resources["NormalTextBrush"] = Resources["TextFillColorTertiaryBrush"];
                    }
                    else
                    {
                        Resources["NormalTextBrush"] = Resources["TextFillColorPrimaryBrush"];
                    }
                }
            }
        }
        catch (OperationCanceledException)
        {
            Console.WriteLine("Timer stopped.");
        }
    }


    private async Task Setup()
    {
        await service.Setup();

        string tos;

        try
        {
            urls = await service.GetMSIUrls();
        }
        catch (Exception)
        {

        }

        try
        {
            tos = await service.FetchTOS();
        }
        catch (Exception)
        {
            tos = "## Error\nFailed to load, visit [https://ahqstore.github.io/tos](https://ahqstore.github.io/tos)";
        }

        FlowDocument tosFlow = engine.Transform(tos);

        foreach (var doc in tosFlow.Blocks.ToList())
        {
            TermsOfService.Blocks.Add(doc);
        }

        string pp;

        try
        {
            pp = await service.FetchPP();
        }
        catch (Exception)
        {
            pp = "## Error\nFailed to load, visit [https://ahqstore.github.io/privacy](https://ahqstore.github.io/privacy)";
        }

        FlowDocument ppFlow = engine.Transform(pp);

        foreach (var doc in ppFlow.Blocks.ToList())
        {
            PrivacyPolicyMarkdown.Blocks.Add(doc);
        }

    }

    private async void Open_License(object sender, RoutedEventArgs e)
    {
        Welcome.Visibility = Visibility.Collapsed;
        LoadingPrepping.Visibility = Visibility.Visible;

        Storyboard sb = (Storyboard)this.FindResource("SlideAndFadeIn");
        sb.Begin(LoadingPrepping);


        try
        {
            await Setup();

            LoadingPrepping.Visibility = Visibility.Collapsed;
            License.Visibility = Visibility.Visible;

            sb.Begin(License);
        }
        catch (Exception err)
        {
            Console.WriteLine(err);
        }

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

    private bool canCloseWindow = true;

    private void Window_Closing(object sender, System.ComponentModel.CancelEventArgs e)
    {
        if (!canCloseWindow)
        {
            e.Cancel = true;
        }
    }

    private async void Launch_Install(object sender, RoutedEventArgs e)
    {
        Configuration.Visibility = Visibility.Collapsed;
        Installing.Visibility = Visibility.Visible;

        Storyboard sb = (Storyboard)this.FindResource("SlideAndFadeIn");
        sb.Begin(Installing);

        canCloseWindow = false;

        try
        {
            AHQInstaller inst = new AHQInstaller();

            string url;

            int idx = ReleaseChannel.SelectedIndex;

            if (idx == 0)
            {
                url = urls.Release!;
            }
            else
            {
                url = urls.Prerelease!;
            }

            await inst.Install(url, InstallCertificate.IsChecked.GetValueOrDefault(false), (txt, state, status) =>
            {
                installingStatusText = txt;

                switch (state)
                {
                    case Typeof.Downloading:
                        TaskbarInfo.ProgressState = System.Windows.Shell.TaskbarItemProgressState.Normal;
                        TaskbarInfo.ProgressValue = (double)status! / 100.0;

                        InstallingProgressStatus.Value = (double)status!;
                        InstallingProgressStatus.IsIndeterminate = false;
                        break;
                    case Typeof.Indeterminate:
                        TaskbarInfo.ProgressState = System.Windows.Shell.TaskbarItemProgressState.Indeterminate;

                        InstallingProgressStatus.IsIndeterminate = true;
                        break;
                    case Typeof.Installed:
                        TaskbarInfo.ProgressState = System.Windows.Shell.TaskbarItemProgressState.None;

                        Installing.Visibility = Visibility.Collapsed;
                        Installed.Visibility = Visibility.Visible;

                        Storyboard sb = (Storyboard)this.FindResource("SlideAndFadeIn");
                        sb.Begin(Installed);

                        break;
                    case Typeof.Error:
                        TaskbarInfo.ProgressState = System.Windows.Shell.TaskbarItemProgressState.None;
                        InstallingProgressStatus.IsIndeterminate = false;
                        InstallingProgressStatus.Value = 0;
                        break;
                    case Typeof.None:
                        TaskbarInfo.ProgressState = System.Windows.Shell.TaskbarItemProgressState.Indeterminate;

                        InstallingProgressStatus.IsIndeterminate = true;
                        break;
                }

            });
        }
        catch (Exception)
        {

        }
        finally
        {
            canCloseWindow = true;
        }
    }

    private async void ArchitectureSelect(object sender, RoutedEventArgs e)
    {
        PrivacyPolicy.Visibility = Visibility.Collapsed;
        Configuration.Visibility = Visibility.Visible;

        Storyboard sb = (Storyboard)this.FindResource("SlideAndFadeIn");
        sb.Begin(Configuration);

        CheckInstallTask();
        CheckInstallTask();
    }

    private void Close_App(object sender, RoutedEventArgs e)
    {
        Close();
    }

    bool first = true;

    private void CheckInstallTask()
    {
        if (first)
        {
            first = false;
            return;
        }


        int idx = ReleaseChannel.SelectedIndex;

        if ((idx == 0 && urls.Release == null) || (idx == 1 && urls.Prerelease == null))
        {
            DisableInstall("This Release is not available!");
            return;
        }

        EnableInstall();
    }

    private void ReleaseChannel_Changed(object sender, SelectionChangedEventArgs e)
    {
        CheckInstallTask();
    }

    private void DisableInstall(
        string text
    )
    {
        installEnabled = false;
        ArchStatusText = text;
        ProgressStatus.IsIndeterminate = true;
    }

    private void EnableInstall()
    {
        installEnabled = true;
        ArchStatusText = "Great, you can install now!";
    }


    protected override void OnSourceInitialized(EventArgs e)
    {
        base.OnSourceInitialized(e);

        var argLast = Environment.GetCommandLineArgs().Last();

        switch (argLast)
        {
            case "uninstall":
                Launch_UninstallPrep();
                break;
            case "uninstall-step2":
                _ = Launch_UninstallFInal();
                break;
            default:
                Launch_Normal();
                break;
        }
    }

    public string unstStatusText
    {
        get { return (string)GetValue(UnstStatusTxt); }
        set { SetValue(UnstStatusTxt, value); }
    }

    public static readonly DependencyProperty UnstStatusTxt =
        DependencyProperty.Register(
            "unstStatusTxt",
            typeof(string),
            typeof(MainWindow),
            new PropertyMetadata("We are checking a few things..."));

    private async Task Launch_UninstallFInal()
    {
        UninstallPrep.Visibility = Visibility.Visible;

        Storyboard sb = (Storyboard)this.FindResource("SlideAndFadeIn");
        sb.Begin(UninstallPrep);

        unstStatusText = "We're starting...";

        var unst = new AHQStoreCoreUninstaller();

        await unst.Uninstall((cb) =>
        {
            unstStatusText = cb;
        });

        await Task.Delay(1500);
        Environment.Exit(0);
    }

    private async void Launch_UninstallPrep()
    {
        UninstallPrep.Visibility = Visibility.Visible;

        Storyboard sb = (Storyboard)this.FindResource("SlideAndFadeIn");
        sb.Begin(UninstallPrep);

        var unst = new AHQStoreUninstallerEligibility();

        if (!unst.Eligible())
        {
            unstStatusText = "You cannot uninstall because AHQ Store system apps are installed! Uninstall them first from the store app.";
        }
        else
        {
            unstStatusText = "Launching uninstall...";

            unst.RunMainUninstaller();

            await Task.Delay(1500);

            Environment.Exit(0);
        }
    }

    private void Launch_Normal()
    {
        Welcome.Visibility = Visibility.Visible;

        Storyboard sb = (Storyboard)this.FindResource("SlideAndFadeIn");
        sb.Begin(Welcome);

    }
}