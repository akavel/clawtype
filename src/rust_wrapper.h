// Inspired by:
// - https://friendlyuser.github.io/posts/tech/rust/rust_ffi_with_c_and_cplusplus

#include <inttypes.h>

//#ifdef __cplusplus
extern "C" {
//#endif

void usb_debug_putchar(uint8_t c);
void usb_try_init();

//#ifdef __cplusplus
}
//#endif
