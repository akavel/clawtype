@echo off
setlocal
:: set DEFMT_LOG=debug
cd main
cargo run %*
endlocal

