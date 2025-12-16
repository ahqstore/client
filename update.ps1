pnpm up -L

cd src-ahqstore-types
pnpm up -L
cd ..

$dirs = Get-ChildItem -Attributes Directory -Filter src-*

foreach ($dir in $dirs) {
  cd $dir
  cargo update
  cd ..
}