$modPath = Get-Item -Path .\modPath.txt | Get-Content -Tail 1

cargo skyline install --install-path $modPath