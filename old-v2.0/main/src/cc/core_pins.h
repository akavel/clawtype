#ifndef _core_pins_h_
#define _core_pins_h_

#include <avr/io.h>

#if (GCC_VERSION >= 40300) && (GCC_VERSION < 40302)
#error "Buggy GCC 4.3.0 compiler, please upgrade!"
#endif



#ifdef __cplusplus
extern "C"{
#endif


void _reboot_Teensyduino_(void) __attribute__((noreturn));
void _restart_Teensyduino_(void) __attribute__((noreturn));


#if defined(__AVR_AT90USB162__)
#define analogReference(mode)
#else
extern uint8_t w_analog_reference;
static inline void analogReference(uint8_t mode)
{
	w_analog_reference = (mode << 6);
}
#endif

void yield(void);

extern void delay(uint32_t);

extern volatile uint32_t timer0_millis_count;

static inline uint32_t millis(void) __attribute__((always_inline, unused));
static inline uint32_t millis(void)
{
	uint32_t out;
	asm volatile(
		"in	__tmp_reg__, __SREG__"		"\n\t"
		"cli"					"\n\t"
		"lds	%A0, timer0_millis_count"	"\n\t"
		"lds	%B0, timer0_millis_count+1"	"\n\t"
		"lds	%C0, timer0_millis_count+2"	"\n\t"
		"lds	%D0, timer0_millis_count+3"	"\n\t"
		"out	__SREG__, __tmp_reg__"
		: "=r" (out) : : "r0"
	);
	return out;
}

extern uint32_t _micros(void) __attribute__((noinline));

static inline uint32_t micros(void) __attribute__((always_inline, unused));
static inline uint32_t micros(void)
{
	register uint32_t out asm("r22");
	asm volatile("call _micros" : "=d" (out) : : "r0");
	return out;
}


static inline void delayMicroseconds(uint16_t) __attribute__((always_inline, unused));
static inline void delayMicroseconds(uint16_t usec)
{
	if (__builtin_constant_p(usec)) {
		#if F_CPU == 16000000L
		uint16_t tmp = usec * 4;
		#elif F_CPU == 8000000L
		uint16_t tmp = usec * 2;
		#elif F_CPU == 4000000L
		uint16_t tmp = usec;
		#elif F_CPU == 2000000L
		uint16_t tmp = usec / 2;
		if (usec == 1) {
			asm volatile("rjmp L%=\nL%=:\n" ::);
		}
		#elif F_CPU == 1000000L
		uint16_t tmp = usec / 4;
		if (usec == 1) {
			asm volatile("nop\n");
		} else if (usec == 2) {
			asm volatile("rjmp L%=\nL%=:\n" ::);
		} else if (usec == 3) {
			asm volatile("rjmp L%=\nL%=:\n" ::);
			asm volatile("nop\n");
		}
		#else
		#error "Clock must be 16, 8, 4, 2 or 1 MHz"
		#endif
		if (tmp > 0) {
			if (tmp < 256) {
				uint8_t tmp2 = tmp;
				asm volatile(
				"L_%=_loop:"				// 1 to load
					"subi	%0, 1"		"\n\t"	// 1
					"nop"			"\n\t"	// 1
					"brne	L_%=_loop"	"\n\t"	// 2 (1 on last)
					: "=d" (tmp2)
					: "0" (tmp2)
				);
			} else {
				asm volatile(
				"L_%=_loop:"				// 2 to load
					"sbiw	%A0, 1"		"\n\t"	// 2
					"brne	L_%=_loop"	"\n\t"	// 2 (1 on last)
					: "=w" (tmp)
					: "0" (tmp)
				);
			}
		}
	} else {
		asm volatile(
		#if F_CPU == 16000000L
			"sbiw	%A0, 2"			"\n\t"	// 2
			"brcs	L_%=_end"		"\n\t"	// 1
			"breq	L_%=_end"		"\n\t"	// 1
			"lsl	%A0"			"\n\t"	// 1
			"rol	%B0"			"\n\t"	// 1
			"lsl	%A0"			"\n\t"	// 1
			"rol	%B0"			"\n\t"	// 1  overhead: (8)/4 = 2us
		#elif F_CPU == 8000000L
			"sbiw	%A0, 3"			"\n\t"	// 2
			"brcs	L_%=_end"		"\n\t"	// 1
			"breq	L_%=_end"		"\n\t"	// 1
			"lsl	%A0"			"\n\t"	// 1
			"rol	%B0"			"\n\t"	// 1  overhead: (6)/2 = 3 us
		#elif F_CPU == 4000000L
			"sbiw	%A0, 4"			"\n\t"	// 2
			"brcs	L_%=_end"		"\n\t"	// 1
			"breq	L_%=_end"		"\n\t"	// 1  overhead: (4) = 4 us
		#elif F_CPU == 2000000L
			"sbiw	%A0, 12"		"\n\t"	// 2
			"brcs	L_%=_end"		"\n\t"	// 1
			"breq	L_%=_end"		"\n\t"	// 1
			"lsr	%B0"			"\n\t"	// 1
			"ror	%A0"			"\n\t"	// 1  overhead: (6)*2 = 12 us
		#elif F_CPU == 1000000L
			"sbiw	%A0, 32"		"\n\t"	// 2
			"brcs	L_%=_end"		"\n\t"	// 1
			"breq	L_%=_end"		"\n\t"	// 1
			"lsr	%B0"			"\n\t"	// 1
			"ror	%A0"			"\n\t"	// 1
			"lsr	%B0"			"\n\t"	// 1
			"ror	%A0"			"\n\t"	// 1  overhead: (8)*4 = 32 us
		#endif
		"L_%=_loop:"
			"sbiw	%A0, 1"			"\n\t"	// 2
			"brne	L_%=_loop"		"\n\t"	// 2
		"L_%=_end:"
			: "=w" (usec)
			: "0" (usec)
		);
	}
}


#ifdef __cplusplus
} // extern "C"

#endif // __cplusplus
#endif
