@echo off
setlocal
set PATH=C:\Users\Mateusz\AppData\Local\Arduino15\packages\teensy\tools\teensy-compile\11.3.1\avr\bin;%PATH%
cd chords
cargo build %*
endlocal
