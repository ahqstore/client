# Building
cargo test --features export

# Packaging

$cargotoml = Get-Content ./Cargo.toml -Raw

# Request regexp
$cargotoml -match "version ?= ?(`"|')(?<ver>[0-9.]+)(`"|')" > $null

# Query version
$version = $Matches.ver


# Query package.json
$data = Get-Content ./pkg/package.json | ConvertFrom-Json 

$data.version = $version

# Convert back to json
$out = ConvertTo-Json $data -Depth 100

Set-Content "./pkg/package.json" -Value $out -Encoding utf8

$data.name = "@ahqstore/core-types"

$out = ConvertTo-Json $data -Depth 100

Set-Content "./types/package.json" -Value $out -Encoding utf8

Copy-Item ./README.md ./pkg/README.MD
Copy-Item ./README.md ./types/README.MD

# Publish
if ($env:NO_PUBLISH -ne "true") {
  Set-Location pkg
  npm publish --access public
  Set-Location ../types
  npm publish --access public
  Set-Location ..
}