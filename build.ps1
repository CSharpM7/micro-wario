$installVanilla=$true
$installMod=$true

If ($args[0] -like "*vanilla*") {
    $installMod=$false
}
elseif ($args[0] -like "*mod*") {
    $installVanilla=$false
}

$pluginLIB = "target/aarch64-skyline-switch/release/libsmashline_warioland.nro"
$pluginNRO = "target/aarch64-skyline-switch/release/plugin.nro"
if (Test-Path $pluginNRO) {
    Remove-Item $pluginNRO
}

cargo skyline build --release
Move-Item -Path $pluginLIB -Destination $pluginNRO

Invoke-Item "target/aarch64-skyline-switch/release"