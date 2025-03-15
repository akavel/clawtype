@echo off
set P=C:\Users\Mateusz\AppData\Local\Arduino15\packages\teensy\tools\teensy-compile\11.3.1\avr\bin

set F=target\avr-atmega32u4\debug\clawtype
%P%\avr-objcopy -O ihex -R .eeprom -R .fuse -R .lock -R .signature %F%.elf %F%.hex
dir %F%.*

set F=target\avr-atmega32u4\release\clawtype
%P%\avr-objcopy -O ihex -R .eeprom -R .fuse -R .lock -R .signature %F%.elf %F%.hex
dir %F%.*
