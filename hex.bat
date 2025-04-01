@echo off

cargo objcopy --release -- -O ihex clawtype.hex
dir *.hex
