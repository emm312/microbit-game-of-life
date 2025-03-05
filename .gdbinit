# Connects GDB to OpenOCD server port
target remote :3333

monitor arm semihosting enable

# (optional) Unmangle function names when debugging
set print asm-demangle on
monitor arm semihosting enable
load
