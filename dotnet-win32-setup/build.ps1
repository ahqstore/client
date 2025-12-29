$items = Get-ChildItem -Path "C:\Program Files (x86)\Windows Kits\10\bin\" -Filter "10.*"
$item = $items[-1].Name

dotnet publish -c Release -r win-x64
dotnet publish -c Release -r win-arm64

$signtoolDir = "C:\Program Files (x86)\Windows Kits\10\bin\${item}\x64"

$env:PATH = "$env:PATH;$signtoolDir"

$signtool = "C:\Program Files (x86)\Windows Kits\10\bin\${item}\x64\signtool.exe"

$env:SIGNTOOL_PATH = $signtool

$sha = "8B85C3AE8D2243C2B62D53D14479A99D413F52A7"

&$signtool sign /sha1 "$sha" /fd SHA256 /t http://timestamp.digicert.com "./bin/Release/net10.0-windows/win-x64/publish/AHQStoreWin32Setup.exe"
&$signtool sign /sha1 "$sha" /fd SHA256 /t http://timestamp.digicert.com "./bin/Release/net10.0-windows/win-arm64/publish/AHQStoreWin32Setup.exe"

Set-Location .\ahqstore_setup

cargo build --target x86_64-pc-windows-msvc --release
cargo build --target aarch64-pc-windows-msvc --release

Set-Location ..

&$signtool sign /sha1 "$sha" /fd SHA256 /t http://timestamp.digicert.com "./ahqstore_setup/target/x86_64-pc-windows-msvc/release/ahqstore_setup.exe"
&$signtool sign /sha1 "$sha" /fd SHA256 /t http://timestamp.digicert.com "./ahqstore_setup/target/aarch64-pc-windows-msvc/release/ahqstore_setup.exe"

mkdir outputs
Copy-Item "./ahqstore_setup/target/x86_64-pc-windows-msvc/release/ahqstore_setup.exe" ./outputs/ahqstore_setup_x64.exe
Copy-Item "./ahqstore_setup/target/aarch64-pc-windows-msvc/release/ahqstore_setup.exe" ./outputs/ahqstore_setup_arm64.exe
