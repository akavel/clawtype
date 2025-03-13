#ifndef _core_pins_h_
#define _core_pins_h_

#include <avr/io.h>

#if (GCC_VERSION >= 40300) && (GCC_VERSION < 40302)
#error "Buggy GCC 4.3.0 compiler, please upgrade!"
#endif


// This will speed up digitalWrite slightly, but it
// adds to your code size.  However, if you don't
// have many digitalWrites, or most are compile time
// constant inputs, then this might be worthwhile
//#define DIGITAL_WRITE_EXPENSIVE_INLINE_OPTIMIZATION

// This removes checking in digitalWrite for pin numbers
// beyond the maximum number of pins.  Your program
// WILL CRASH is digitalWrite is called with a pin number
// too large, when this is enabled.  However, if you don't
// make such mistakes, you'll get a minor speedup.
//#define DIGITAL_WRITE_RISKY_OMIT_OVERFLOW_CHECK

#define HIGH 0x1
#define LOW  0x0

#define INPUT 0x0
#define OUTPUT 0x1
#define INPUT_PULLUP 0x2

#define LSBFIRST 0
#define MSBFIRST 1

#ifndef _BV
#define _BV(n) (1<<(n))
#endif

////////////////////////////////////
// Teensy 2.0
////////////////////////////////////
#if defined(__AVR_ATmega32U4__)
#define CORE_NUM_TOTAL_PINS	25
#define CORE_NUM_DIGITAL	13
#define CORE_NUM_ANALOG		12
#define CORE_NUM_PWM		7
#define CORE_NUM_INTERRUPT	4
#define PIN_B0		0
#define PIN_B1		1
#define PIN_B2		2
#define PIN_B3		3
#define PIN_B7		4
#define PIN_D0		5
#define PIN_D1		6
#define PIN_D2		7
#define PIN_D3		8
#define PIN_C6		9
#define PIN_C7		10
#define PIN_D6		11
#define PIN_D7		12
#define PIN_B4		13
#define PIN_B5		14
#define PIN_B6		15
#define PIN_F7		16
#define PIN_F6		17
#define PIN_F5		18
#define PIN_F4		19
#define PIN_F1		20
#define PIN_F0		21
#define PIN_D4		22
#define PIN_D5		23
#define PIN_E6		24
#define CORE_PIN0_BIT		0
#define CORE_PIN1_BIT		1
#define CORE_PIN2_BIT		2
#define CORE_PIN3_BIT		3
#define CORE_PIN4_BIT		7
#define CORE_PIN5_BIT		0
#define CORE_PIN6_BIT		1
#define CORE_PIN7_BIT		2
#define CORE_PIN8_BIT		3
#define CORE_PIN9_BIT		6
#define CORE_PIN10_BIT		7
#define CORE_PIN11_BIT		6
#define CORE_PIN12_BIT		7
#define CORE_PIN13_BIT		4
#define CORE_PIN14_BIT		5
#define CORE_PIN15_BIT		6
#define CORE_PIN16_BIT		7
#define CORE_PIN17_BIT		6
#define CORE_PIN18_BIT		5
#define CORE_PIN19_BIT		4
#define CORE_PIN20_BIT		1
#define CORE_PIN21_BIT		0
#define CORE_PIN22_BIT		4
#define CORE_PIN23_BIT		5
#define CORE_PIN24_BIT		6
#define CORE_PIN0_BITMASK	_BV(CORE_PIN0_BIT)
#define CORE_PIN1_BITMASK	_BV(CORE_PIN1_BIT)
#define CORE_PIN2_BITMASK	_BV(CORE_PIN2_BIT)
#define CORE_PIN3_BITMASK	_BV(CORE_PIN3_BIT)
#define CORE_PIN4_BITMASK	_BV(CORE_PIN4_BIT)
#define CORE_PIN5_BITMASK	_BV(CORE_PIN5_BIT)
#define CORE_PIN6_BITMASK	_BV(CORE_PIN6_BIT)
#define CORE_PIN7_BITMASK	_BV(CORE_PIN7_BIT)
#define CORE_PIN8_BITMASK	_BV(CORE_PIN8_BIT)
#define CORE_PIN9_BITMASK	_BV(CORE_PIN9_BIT)
#define CORE_PIN10_BITMASK	_BV(CORE_PIN10_BIT)
#define CORE_PIN11_BITMASK	_BV(CORE_PIN11_BIT)
#define CORE_PIN12_BITMASK	_BV(CORE_PIN12_BIT)
#define CORE_PIN13_BITMASK	_BV(CORE_PIN13_BIT)
#define CORE_PIN14_BITMASK	_BV(CORE_PIN14_BIT)
#define CORE_PIN15_BITMASK	_BV(CORE_PIN15_BIT)
#define CORE_PIN16_BITMASK	_BV(CORE_PIN16_BIT)
#define CORE_PIN17_BITMASK	_BV(CORE_PIN17_BIT)
#define CORE_PIN18_BITMASK	_BV(CORE_PIN18_BIT)
#define CORE_PIN19_BITMASK	_BV(CORE_PIN19_BIT)
#define CORE_PIN20_BITMASK	_BV(CORE_PIN20_BIT)
#define CORE_PIN21_BITMASK	_BV(CORE_PIN21_BIT)
#define CORE_PIN22_BITMASK	_BV(CORE_PIN22_BIT)
#define CORE_PIN23_BITMASK	_BV(CORE_PIN23_BIT)
#define CORE_PIN24_BITMASK	_BV(CORE_PIN24_BIT)
#define CORE_PIN0_PORTREG	PORTB
#define CORE_PIN1_PORTREG	PORTB
#define CORE_PIN2_PORTREG	PORTB
#define CORE_PIN3_PORTREG	PORTB
#define CORE_PIN4_PORTREG	PORTB
#define CORE_PIN5_PORTREG	PORTD
#define CORE_PIN6_PORTREG	PORTD
#define CORE_PIN7_PORTREG	PORTD
#define CORE_PIN8_PORTREG	PORTD
#define CORE_PIN9_PORTREG	PORTC
#define CORE_PIN10_PORTREG	PORTC
#define CORE_PIN11_PORTREG	PORTD
#define CORE_PIN12_PORTREG	PORTD
#define CORE_PIN13_PORTREG	PORTB
#define CORE_PIN14_PORTREG	PORTB
#define CORE_PIN15_PORTREG	PORTB
#define CORE_PIN16_PORTREG	PORTF
#define CORE_PIN17_PORTREG	PORTF
#define CORE_PIN18_PORTREG	PORTF
#define CORE_PIN19_PORTREG	PORTF
#define CORE_PIN20_PORTREG	PORTF
#define CORE_PIN21_PORTREG	PORTF
#define CORE_PIN22_PORTREG	PORTD
#define CORE_PIN23_PORTREG	PORTD
#define CORE_PIN24_PORTREG	PORTE
#define CORE_PIN0_DDRREG	DDRB
#define CORE_PIN1_DDRREG	DDRB
#define CORE_PIN2_DDRREG	DDRB
#define CORE_PIN3_DDRREG	DDRB
#define CORE_PIN4_DDRREG	DDRB
#define CORE_PIN5_DDRREG	DDRD
#define CORE_PIN6_DDRREG	DDRD
#define CORE_PIN7_DDRREG	DDRD
#define CORE_PIN8_DDRREG	DDRD
#define CORE_PIN9_DDRREG	DDRC
#define CORE_PIN10_DDRREG	DDRC
#define CORE_PIN11_DDRREG	DDRD
#define CORE_PIN12_DDRREG	DDRD
#define CORE_PIN13_DDRREG	DDRB
#define CORE_PIN14_DDRREG	DDRB
#define CORE_PIN15_DDRREG	DDRB
#define CORE_PIN16_DDRREG	DDRF
#define CORE_PIN17_DDRREG	DDRF
#define CORE_PIN18_DDRREG	DDRF
#define CORE_PIN19_DDRREG	DDRF
#define CORE_PIN20_DDRREG	DDRF
#define CORE_PIN21_DDRREG	DDRF
#define CORE_PIN22_DDRREG	DDRD
#define CORE_PIN23_DDRREG	DDRD
#define CORE_PIN24_DDRREG	DDRE
#define CORE_PIN0_PINREG	PINB
#define CORE_PIN1_PINREG	PINB
#define CORE_PIN2_PINREG	PINB
#define CORE_PIN3_PINREG	PINB
#define CORE_PIN4_PINREG	PINB
#define CORE_PIN5_PINREG	PIND
#define CORE_PIN6_PINREG	PIND
#define CORE_PIN7_PINREG	PIND
#define CORE_PIN8_PINREG	PIND
#define CORE_PIN9_PINREG	PINC
#define CORE_PIN10_PINREG	PINC
#define CORE_PIN11_PINREG	PIND
#define CORE_PIN12_PINREG	PIND
#define CORE_PIN13_PINREG	PINB
#define CORE_PIN14_PINREG	PINB
#define CORE_PIN15_PINREG	PINB
#define CORE_PIN16_PINREG	PINF
#define CORE_PIN17_PINREG	PINF
#define CORE_PIN18_PINREG	PINF
#define CORE_PIN19_PINREG	PINF
#define CORE_PIN20_PINREG	PINF
#define CORE_PIN21_PINREG	PINF
#define CORE_PIN22_PINREG	PIND
#define CORE_PIN23_PINREG	PIND
#define CORE_PIN24_PINREG	PINE
#define CORE_ADC0_PIN		PIN_F0
#define CORE_ADC1_PIN		PIN_F1
#define CORE_ADC4_PIN		PIN_F4
#define CORE_ADC5_PIN		PIN_F5
#define CORE_ADC6_PIN		PIN_F6
#define CORE_ADC7_PIN		PIN_F7
#define CORE_ADC8_PIN		PIN_D4
#define CORE_ADC9_PIN		PIN_D6
#define CORE_ADC10_PIN		PIN_D7
#define CORE_ADC11_PIN		PIN_B4
#define CORE_ADC12_PIN		PIN_B5
#define CORE_ADC13_PIN		PIN_B6
#define CORE_RXD1_PIN           PIN_D2
#define CORE_TXD1_PIN           PIN_D3
#define CORE_XCK1_PIN           PIN_D5
#define CORE_SDA0_PIN           PIN_D1
#define CORE_SCL0_PIN           PIN_D0
#define CORE_INT0_PIN           PIN_D0
#define CORE_INT1_PIN           PIN_D1
#define CORE_INT2_PIN           PIN_D2
#define CORE_INT3_PIN           PIN_D3
#define CORE_SS0_PIN            PIN_B0
#define CORE_MOSI0_PIN          PIN_B2
#define CORE_MISO0_PIN          PIN_B3
#define CORE_SCLK0_PIN          PIN_B1
#define CORE_T0_PIN             PIN_D7
#define CORE_T1_PIN             PIN_D6
#define CORE_ICP1_PIN           PIN_D4
#define CORE_ICP3_PIN           PIN_C7
#define CORE_OC0A_PIN           PIN_B7
#define CORE_OC0B_PIN           PIN_D0
#define CORE_OC1A_PIN           PIN_B5
#define CORE_OC1B_PIN           PIN_B6
#define CORE_OC1C_PIN           PIN_B7
#define CORE_OC3A_PIN           PIN_C6
#define CORE_OC4A_PIN           PIN_C7
#define CORE_OC4AN_PIN          PIN_C6
#define CORE_OC4B_PIN           PIN_B6
#define CORE_OC4BN_PIN          PIN_B5
#define CORE_OC4D_PIN           PIN_D7
#define CORE_OC4DN_PIN          PIN_D6
#define CORE_PCINT0_PIN		PIN_B0
#define CORE_PCINT1_PIN		PIN_B1
#define CORE_PCINT2_PIN		PIN_B2
#define CORE_PCINT3_PIN		PIN_B3
#define CORE_PCINT4_PIN		PIN_B4
#define CORE_PCINT5_PIN		PIN_B5
#define CORE_PCINT6_PIN		PIN_B6
#define CORE_PCINT7_PIN		PIN_B7
#define CORE_LED0_PIN		PIN_D6
#define CORE_PWM0_PIN		CORE_OC1C_PIN	// B7  4
#define CORE_PWM1_PIN		CORE_OC0B_PIN	// D0  5
#define CORE_PWM2_PIN		CORE_OC3A_PIN	// C6  9
#define CORE_PWM3_PIN		CORE_OC4A_PIN	// C7  10
#define CORE_PWM4_PIN		CORE_OC4D_PIN	// D7  12
#define CORE_PWM5_PIN		CORE_OC1A_PIN	// B5  14
#define CORE_PWM6_PIN		CORE_OC1B_PIN	// B6  15
#define CORE_ANALOG0_PIN	PIN_F0	// 21  ADC0
#define CORE_ANALOG1_PIN	PIN_F1	// 20  ADC1
#define CORE_ANALOG2_PIN	PIN_F4	// 19  ADC4
#define CORE_ANALOG3_PIN	PIN_F5	// 18  ADC5
#define CORE_ANALOG4_PIN	PIN_F6	// 17  ADC6
#define CORE_ANALOG5_PIN	PIN_F7	// 16  ADC7
#define CORE_ANALOG6_PIN	PIN_B6	// 15  ADC13
#define CORE_ANALOG7_PIN	PIN_B5	// 14  ADC12
#define CORE_ANALOG8_PIN	PIN_B4	// 13  ADC11
#define CORE_ANALOG9_PIN	PIN_D7	// 12  ADC10
#define CORE_ANALOG10_PIN	PIN_D6  // 11  ADC9
#define CORE_ANALOG11_PIN	PIN_D4	// 22  ADC8


////////////////////////////////////
// Teensy 1.0
////////////////////////////////////
#elif defined(__AVR_AT90USB162__)
#define CORE_NUM_TOTAL_PINS	21
#define CORE_NUM_DIGITAL	21
#define CORE_NUM_ANALOG		0
#define CORE_NUM_PWM		4
#define CORE_NUM_INTERRUPT	8
#define PIN_D0		0
#define PIN_D1		1
#define PIN_D2		2
#define PIN_D3		3
#define PIN_D4		4
#define PIN_D5		5
#define PIN_D6		6
#define PIN_D7		7
#define PIN_SS		8
#define PIN_SCLK	9
#define PIN_MOSI	10
#define PIN_MISO	11
#define PIN_B0		8
#define PIN_B1		9
#define PIN_B2		10
#define PIN_B3		11
#define PIN_B4		12
#define PIN_B5		13
#define PIN_B6		14
#define PIN_B7		15
#define PIN_C7		16
#define PIN_C6		17
#define PIN_C5		18
#define PIN_C4		19
#define PIN_C2		20
#define CORE_PIN0_BIT		0
#define CORE_PIN1_BIT		1
#define CORE_PIN2_BIT		2
#define CORE_PIN3_BIT		3
#define CORE_PIN4_BIT		4
#define CORE_PIN5_BIT		5
#define CORE_PIN6_BIT		6
#define CORE_PIN7_BIT		7
#define CORE_PIN8_BIT		0
#define CORE_PIN9_BIT		1
#define CORE_PIN10_BIT		2
#define CORE_PIN11_BIT		3
#define CORE_PIN12_BIT		4
#define CORE_PIN13_BIT		5
#define CORE_PIN14_BIT		6
#define CORE_PIN15_BIT		7
#define CORE_PIN16_BIT		7
#define CORE_PIN17_BIT		6
#define CORE_PIN18_BIT		5
#define CORE_PIN19_BIT		4
#define CORE_PIN20_BIT		2
#define CORE_PIN0_BITMASK	_BV(CORE_PIN0_BIT)
#define CORE_PIN1_BITMASK	_BV(CORE_PIN1_BIT)
#define CORE_PIN2_BITMASK	_BV(CORE_PIN2_BIT)
#define CORE_PIN3_BITMASK	_BV(CORE_PIN3_BIT)
#define CORE_PIN4_BITMASK	_BV(CORE_PIN4_BIT)
#define CORE_PIN5_BITMASK	_BV(CORE_PIN5_BIT)
#define CORE_PIN6_BITMASK	_BV(CORE_PIN6_BIT)
#define CORE_PIN7_BITMASK	_BV(CORE_PIN7_BIT)
#define CORE_PIN8_BITMASK	_BV(CORE_PIN8_BIT)
#define CORE_PIN9_BITMASK	_BV(CORE_PIN9_BIT)
#define CORE_PIN10_BITMASK	_BV(CORE_PIN10_BIT)
#define CORE_PIN11_BITMASK	_BV(CORE_PIN11_BIT)
#define CORE_PIN12_BITMASK	_BV(CORE_PIN12_BIT)
#define CORE_PIN13_BITMASK	_BV(CORE_PIN13_BIT)
#define CORE_PIN14_BITMASK	_BV(CORE_PIN14_BIT)
#define CORE_PIN15_BITMASK	_BV(CORE_PIN15_BIT)
#define CORE_PIN16_BITMASK	_BV(CORE_PIN16_BIT)
#define CORE_PIN17_BITMASK	_BV(CORE_PIN17_BIT)
#define CORE_PIN18_BITMASK	_BV(CORE_PIN18_BIT)
#define CORE_PIN19_BITMASK	_BV(CORE_PIN19_BIT)
#define CORE_PIN20_BITMASK	_BV(CORE_PIN20_BIT)
#define CORE_PIN0_PORTREG	PORTD
#define CORE_PIN1_PORTREG	PORTD
#define CORE_PIN2_PORTREG	PORTD
#define CORE_PIN3_PORTREG	PORTD
#define CORE_PIN4_PORTREG	PORTD
#define CORE_PIN5_PORTREG	PORTD
#define CORE_PIN6_PORTREG	PORTD
#define CORE_PIN7_PORTREG	PORTD
#define CORE_PIN8_PORTREG	PORTB
#define CORE_PIN9_PORTREG	PORTB
#define CORE_PIN10_PORTREG	PORTB
#define CORE_PIN11_PORTREG	PORTB
#define CORE_PIN12_PORTREG	PORTB
#define CORE_PIN13_PORTREG	PORTB
#define CORE_PIN14_PORTREG	PORTB
#define CORE_PIN15_PORTREG	PORTB
#define CORE_PIN16_PORTREG	PORTC
#define CORE_PIN17_PORTREG	PORTC
#define CORE_PIN18_PORTREG	PORTC
#define CORE_PIN19_PORTREG	PORTC
#define CORE_PIN20_PORTREG	PORTC
#define CORE_PIN0_DDRREG	DDRD
#define CORE_PIN1_DDRREG	DDRD
#define CORE_PIN2_DDRREG	DDRD
#define CORE_PIN3_DDRREG	DDRD
#define CORE_PIN4_DDRREG	DDRD
#define CORE_PIN5_DDRREG	DDRD
#define CORE_PIN6_DDRREG	DDRD
#define CORE_PIN7_DDRREG	DDRD
#define CORE_PIN8_DDRREG	DDRB
#define CORE_PIN9_DDRREG	DDRB
#define CORE_PIN10_DDRREG	DDRB
#define CORE_PIN11_DDRREG	DDRB
#define CORE_PIN12_DDRREG	DDRB
#define CORE_PIN13_DDRREG	DDRB
#define CORE_PIN14_DDRREG	DDRB
#define CORE_PIN15_DDRREG	DDRB
#define CORE_PIN16_DDRREG	DDRC
#define CORE_PIN17_DDRREG	DDRC
#define CORE_PIN18_DDRREG	DDRC
#define CORE_PIN19_DDRREG	DDRC
#define CORE_PIN20_DDRREG	DDRC
#define CORE_PIN0_PINREG	PIND
#define CORE_PIN1_PINREG	PIND
#define CORE_PIN2_PINREG	PIND
#define CORE_PIN3_PINREG	PIND
#define CORE_PIN4_PINREG	PIND
#define CORE_PIN5_PINREG	PIND
#define CORE_PIN6_PINREG	PIND
#define CORE_PIN7_PINREG	PIND
#define CORE_PIN8_PINREG	PINB
#define CORE_PIN9_PINREG	PINB
#define CORE_PIN10_PINREG	PINB
#define CORE_PIN11_PINREG	PINB
#define CORE_PIN12_PINREG	PINB
#define CORE_PIN13_PINREG	PINB
#define CORE_PIN14_PINREG	PINB
#define CORE_PIN15_PINREG	PINB
#define CORE_PIN16_PINREG	PINC
#define CORE_PIN17_PINREG	PINC
#define CORE_PIN18_PINREG	PINC
#define CORE_PIN19_PINREG	PINC
#define CORE_PIN20_PINREG	PINC
#define CORE_T1_PIN		PIN_B4
#define CORE_ICP1_PIN		PIN_C7
#define CORE_OC0A_PIN		PIN_B7
#define CORE_OC0B_PIN		PIN_D0
#define CORE_OC1A_PIN		PIN_C6
#define CORE_OC1B_PIN		PIN_C5
#define CORE_OC1C_PIN		PIN_B7
#define CORE_RXD1_PIN		PIN_D2
#define CORE_TXD1_PIN		PIN_D3
#define CORE_XCK1_PIN		PIN_D5
#define CORE_INT0_PIN		PIN_D0
#define CORE_INT1_PIN		PIN_D1
#define CORE_INT2_PIN		PIN_D2
#define CORE_INT3_PIN		PIN_D3
#define CORE_INT4_PIN		PIN_C2
#define CORE_INT5_PIN		PIN_D4
#define CORE_INT6_PIN		PIN_D6
#define CORE_INT7_PIN		PIN_D7
#define CORE_SS0_PIN		PIN_B0
#define CORE_MOSI0_PIN		PIN_B2
#define CORE_MISO0_PIN		PIN_B3
#define CORE_SCLK0_PIN		PIN_B1
#define CORE_AIN0_PIN		PIN_D1
#define CORE_AIN1_PIN		PIN_D2
#define CORE_PCINT0_PIN		PIN_B0
#define CORE_PCINT1_PIN		PIN_B1
#define CORE_PCINT2_PIN		PIN_B2
#define CORE_PCINT3_PIN		PIN_B3
#define CORE_PCINT4_PIN		PIN_B4
#define CORE_PCINT5_PIN		PIN_B5
#define CORE_PCINT6_PIN		PIN_B6
#define CORE_PCINT7_PIN		PIN_B7
#define CORE_PCINT8_PIN		PIN_C6
#define CORE_PCINT9_PIN		PIN_C5
#define CORE_PCINT10_PIN	PIN_C4
#define CORE_PCINT11_PIN	PIN_C2
#define CORE_PCINT12_PIN	PIN_D5
#define CORE_LED0_PIN		PIN_D6
#define CORE_PWM0_PIN		CORE_OC0B_PIN	// D0, 0
#define CORE_PWM1_PIN		CORE_OC1C_PIN	// B7, 15
#define CORE_PWM2_PIN		CORE_OC1A_PIN	// C6, 17
#define CORE_PWM3_PIN		CORE_OC1B_PIN	// C5, 18


////////////////////////////////////
// Teensy++ 1.0 & 2.0
////////////////////////////////////
#elif defined(__AVR_AT90USB646__) || defined(__AVR_AT90USB1286__)
#define CORE_NUM_TOTAL_PINS	46
#define CORE_NUM_DIGITAL	38
#define CORE_NUM_ANALOG		8
#define CORE_NUM_PWM		9
#define CORE_NUM_INTERRUPT	8
#define PIN_D0		0
#define PIN_D1		1
#define PIN_D2		2
#define PIN_D3		3
#define PIN_D4		4
#define PIN_D5		5
#define PIN_D6		6
#define PIN_D7		7
#define PIN_E0		8
#define PIN_E1		9
#define PIN_C0		10
#define PIN_C1		11
#define PIN_C2		12
#define PIN_C3		13
#define PIN_C4		14
#define PIN_C5		15
#define PIN_C6		16
#define PIN_C7		17
#define PIN_E6		18
#define PIN_E7		19
#define PIN_SS		20
#define PIN_SCLK	21
#define PIN_MOSI	22
#define PIN_MISO	23
#define PIN_B0		20
#define PIN_B1		21
#define PIN_B2		22
#define PIN_B3		23
#define PIN_B4		24
#define PIN_B5		25
#define PIN_B6		26
#define PIN_B7		27
#define PIN_A0		28
#define PIN_A1		29
#define PIN_A2		30
#define PIN_A3		31
#define PIN_A4		32
#define PIN_A5		33
#define PIN_A6		34
#define PIN_A7		35
#define PIN_E4		36
#define PIN_E5		37
#define PIN_F0		38
#define PIN_F1		39
#define PIN_F2		40
#define PIN_F3		41
#define PIN_F4		42
#define PIN_F5		43
#define PIN_F6		44
#define PIN_F7		45
#define CORE_PIN0_BIT		0
#define CORE_PIN1_BIT		1
#define CORE_PIN2_BIT		2
#define CORE_PIN3_BIT		3
#define CORE_PIN4_BIT		4
#define CORE_PIN5_BIT		5
#define CORE_PIN6_BIT		6
#define CORE_PIN7_BIT		7
#define CORE_PIN8_BIT		0
#define CORE_PIN9_BIT		1
#define CORE_PIN10_BIT		0
#define CORE_PIN11_BIT		1
#define CORE_PIN12_BIT		2
#define CORE_PIN13_BIT		3
#define CORE_PIN14_BIT		4
#define CORE_PIN15_BIT		5
#define CORE_PIN16_BIT		6
#define CORE_PIN17_BIT		7
#define CORE_PIN18_BIT		6
#define CORE_PIN19_BIT		7
#define CORE_PIN20_BIT		0
#define CORE_PIN21_BIT		1
#define CORE_PIN22_BIT		2
#define CORE_PIN23_BIT		3
#define CORE_PIN24_BIT		4
#define CORE_PIN25_BIT		5
#define CORE_PIN26_BIT		6
#define CORE_PIN27_BIT		7
#define CORE_PIN28_BIT		0
#define CORE_PIN29_BIT		1
#define CORE_PIN30_BIT		2
#define CORE_PIN31_BIT		3
#define CORE_PIN32_BIT		4
#define CORE_PIN33_BIT		5
#define CORE_PIN34_BIT		6
#define CORE_PIN35_BIT		7
#define CORE_PIN36_BIT		4
#define CORE_PIN37_BIT		5
#define CORE_PIN38_BIT		0
#define CORE_PIN39_BIT		1
#define CORE_PIN40_BIT		2
#define CORE_PIN41_BIT		3
#define CORE_PIN42_BIT		4
#define CORE_PIN43_BIT		5
#define CORE_PIN44_BIT		6
#define CORE_PIN45_BIT		7
#define CORE_PIN0_BITMASK	_BV(CORE_PIN0_BIT)
#define CORE_PIN1_BITMASK	_BV(CORE_PIN1_BIT)
#define CORE_PIN2_BITMASK	_BV(CORE_PIN2_BIT)
#define CORE_PIN3_BITMASK	_BV(CORE_PIN3_BIT)
#define CORE_PIN4_BITMASK	_BV(CORE_PIN4_BIT)
#define CORE_PIN5_BITMASK	_BV(CORE_PIN5_BIT)
#define CORE_PIN6_BITMASK	_BV(CORE_PIN6_BIT)
#define CORE_PIN7_BITMASK	_BV(CORE_PIN7_BIT)
#define CORE_PIN8_BITMASK	_BV(CORE_PIN8_BIT)
#define CORE_PIN9_BITMASK	_BV(CORE_PIN9_BIT)
#define CORE_PIN10_BITMASK	_BV(CORE_PIN10_BIT)
#define CORE_PIN11_BITMASK	_BV(CORE_PIN11_BIT)
#define CORE_PIN12_BITMASK	_BV(CORE_PIN12_BIT)
#define CORE_PIN13_BITMASK	_BV(CORE_PIN13_BIT)
#define CORE_PIN14_BITMASK	_BV(CORE_PIN14_BIT)
#define CORE_PIN15_BITMASK	_BV(CORE_PIN15_BIT)
#define CORE_PIN16_BITMASK	_BV(CORE_PIN16_BIT)
#define CORE_PIN17_BITMASK	_BV(CORE_PIN17_BIT)
#define CORE_PIN18_BITMASK	_BV(CORE_PIN18_BIT)
#define CORE_PIN19_BITMASK	_BV(CORE_PIN19_BIT)
#define CORE_PIN20_BITMASK	_BV(CORE_PIN20_BIT)
#define CORE_PIN21_BITMASK	_BV(CORE_PIN21_BIT)
#define CORE_PIN22_BITMASK	_BV(CORE_PIN22_BIT)
#define CORE_PIN23_BITMASK	_BV(CORE_PIN23_BIT)
#define CORE_PIN24_BITMASK	_BV(CORE_PIN24_BIT)
#define CORE_PIN25_BITMASK	_BV(CORE_PIN25_BIT)
#define CORE_PIN26_BITMASK	_BV(CORE_PIN26_BIT)
#define CORE_PIN27_BITMASK	_BV(CORE_PIN27_BIT)
#define CORE_PIN28_BITMASK	_BV(CORE_PIN28_BIT)
#define CORE_PIN29_BITMASK	_BV(CORE_PIN29_BIT)
#define CORE_PIN30_BITMASK	_BV(CORE_PIN30_BIT)
#define CORE_PIN31_BITMASK	_BV(CORE_PIN31_BIT)
#define CORE_PIN32_BITMASK	_BV(CORE_PIN32_BIT)
#define CORE_PIN33_BITMASK	_BV(CORE_PIN33_BIT)
#define CORE_PIN34_BITMASK	_BV(CORE_PIN34_BIT)
#define CORE_PIN35_BITMASK	_BV(CORE_PIN35_BIT)
#define CORE_PIN36_BITMASK	_BV(CORE_PIN36_BIT)
#define CORE_PIN37_BITMASK	_BV(CORE_PIN37_BIT)
#define CORE_PIN38_BITMASK	_BV(CORE_PIN38_BIT)
#define CORE_PIN39_BITMASK	_BV(CORE_PIN39_BIT)
#define CORE_PIN40_BITMASK	_BV(CORE_PIN40_BIT)
#define CORE_PIN41_BITMASK	_BV(CORE_PIN41_BIT)
#define CORE_PIN42_BITMASK	_BV(CORE_PIN42_BIT)
#define CORE_PIN43_BITMASK	_BV(CORE_PIN43_BIT)
#define CORE_PIN44_BITMASK	_BV(CORE_PIN44_BIT)
#define CORE_PIN45_BITMASK	_BV(CORE_PIN45_BIT)
#define CORE_PIN0_PORTREG	PORTD
#define CORE_PIN1_PORTREG	PORTD
#define CORE_PIN2_PORTREG	PORTD
#define CORE_PIN3_PORTREG	PORTD
#define CORE_PIN4_PORTREG	PORTD
#define CORE_PIN5_PORTREG	PORTD
#define CORE_PIN6_PORTREG	PORTD
#define CORE_PIN7_PORTREG	PORTD
#define CORE_PIN8_PORTREG	PORTE
#define CORE_PIN9_PORTREG	PORTE
#define CORE_PIN10_PORTREG	PORTC
#define CORE_PIN11_PORTREG	PORTC
#define CORE_PIN12_PORTREG	PORTC
#define CORE_PIN13_PORTREG	PORTC
#define CORE_PIN14_PORTREG	PORTC
#define CORE_PIN15_PORTREG	PORTC
#define CORE_PIN16_PORTREG	PORTC
#define CORE_PIN17_PORTREG	PORTC
#define CORE_PIN18_PORTREG	PORTE
#define CORE_PIN19_PORTREG	PORTE
#define CORE_PIN20_PORTREG	PORTB
#define CORE_PIN21_PORTREG	PORTB
#define CORE_PIN22_PORTREG	PORTB
#define CORE_PIN23_PORTREG	PORTB
#define CORE_PIN24_PORTREG	PORTB
#define CORE_PIN25_PORTREG	PORTB
#define CORE_PIN26_PORTREG	PORTB
#define CORE_PIN27_PORTREG	PORTB
#define CORE_PIN28_PORTREG	PORTA
#define CORE_PIN29_PORTREG	PORTA
#define CORE_PIN30_PORTREG	PORTA
#define CORE_PIN31_PORTREG	PORTA
#define CORE_PIN32_PORTREG	PORTA
#define CORE_PIN33_PORTREG	PORTA
#define CORE_PIN34_PORTREG	PORTA
#define CORE_PIN35_PORTREG	PORTA
#define CORE_PIN36_PORTREG	PORTE
#define CORE_PIN37_PORTREG	PORTE
#define CORE_PIN38_PORTREG	PORTF
#define CORE_PIN39_PORTREG	PORTF
#define CORE_PIN40_PORTREG	PORTF
#define CORE_PIN41_PORTREG	PORTF
#define CORE_PIN42_PORTREG	PORTF
#define CORE_PIN43_PORTREG	PORTF
#define CORE_PIN44_PORTREG	PORTF
#define CORE_PIN45_PORTREG	PORTF
#define CORE_PIN0_DDRREG	DDRD
#define CORE_PIN1_DDRREG	DDRD
#define CORE_PIN2_DDRREG	DDRD
#define CORE_PIN3_DDRREG	DDRD
#define CORE_PIN4_DDRREG	DDRD
#define CORE_PIN5_DDRREG	DDRD
#define CORE_PIN6_DDRREG	DDRD
#define CORE_PIN7_DDRREG	DDRD
#define CORE_PIN8_DDRREG	DDRE
#define CORE_PIN9_DDRREG	DDRE
#define CORE_PIN10_DDRREG	DDRC
#define CORE_PIN11_DDRREG	DDRC
#define CORE_PIN12_DDRREG	DDRC
#define CORE_PIN13_DDRREG	DDRC
#define CORE_PIN14_DDRREG	DDRC
#define CORE_PIN15_DDRREG	DDRC
#define CORE_PIN16_DDRREG	DDRC
#define CORE_PIN17_DDRREG	DDRC
#define CORE_PIN18_DDRREG	DDRE
#define CORE_PIN19_DDRREG	DDRE
#define CORE_PIN20_DDRREG	DDRB
#define CORE_PIN21_DDRREG	DDRB
#define CORE_PIN22_DDRREG	DDRB
#define CORE_PIN23_DDRREG	DDRB
#define CORE_PIN24_DDRREG	DDRB
#define CORE_PIN25_DDRREG	DDRB
#define CORE_PIN26_DDRREG	DDRB
#define CORE_PIN27_DDRREG	DDRB
#define CORE_PIN28_DDRREG	DDRA
#define CORE_PIN29_DDRREG	DDRA
#define CORE_PIN30_DDRREG	DDRA
#define CORE_PIN31_DDRREG	DDRA
#define CORE_PIN32_DDRREG	DDRA
#define CORE_PIN33_DDRREG	DDRA
#define CORE_PIN34_DDRREG	DDRA
#define CORE_PIN35_DDRREG	DDRA
#define CORE_PIN36_DDRREG	DDRE
#define CORE_PIN37_DDRREG	DDRE
#define CORE_PIN38_DDRREG	DDRF
#define CORE_PIN39_DDRREG	DDRF
#define CORE_PIN40_DDRREG	DDRF
#define CORE_PIN41_DDRREG	DDRF
#define CORE_PIN42_DDRREG	DDRF
#define CORE_PIN43_DDRREG	DDRF
#define CORE_PIN44_DDRREG	DDRF
#define CORE_PIN45_DDRREG	DDRF
#define CORE_PIN0_PINREG	PIND
#define CORE_PIN1_PINREG	PIND
#define CORE_PIN2_PINREG	PIND
#define CORE_PIN3_PINREG	PIND
#define CORE_PIN4_PINREG	PIND
#define CORE_PIN5_PINREG	PIND
#define CORE_PIN6_PINREG	PIND
#define CORE_PIN7_PINREG	PIND
#define CORE_PIN8_PINREG	PINE
#define CORE_PIN9_PINREG	PINE
#define CORE_PIN10_PINREG	PINC
#define CORE_PIN11_PINREG	PINC
#define CORE_PIN12_PINREG	PINC
#define CORE_PIN13_PINREG	PINC
#define CORE_PIN14_PINREG	PINC
#define CORE_PIN15_PINREG	PINC
#define CORE_PIN16_PINREG	PINC
#define CORE_PIN17_PINREG	PINC
#define CORE_PIN18_PINREG	PINE
#define CORE_PIN19_PINREG	PINE
#define CORE_PIN20_PINREG	PINB
#define CORE_PIN21_PINREG	PINB
#define CORE_PIN22_PINREG	PINB
#define CORE_PIN23_PINREG	PINB
#define CORE_PIN24_PINREG	PINB
#define CORE_PIN25_PINREG	PINB
#define CORE_PIN26_PINREG	PINB
#define CORE_PIN27_PINREG	PINB
#define CORE_PIN28_PINREG	PINA
#define CORE_PIN29_PINREG	PINA
#define CORE_PIN30_PINREG	PINA
#define CORE_PIN31_PINREG	PINA
#define CORE_PIN32_PINREG	PINA
#define CORE_PIN33_PINREG	PINA
#define CORE_PIN34_PINREG	PINA
#define CORE_PIN35_PINREG	PINA
#define CORE_PIN36_PINREG	PINE
#define CORE_PIN37_PINREG	PINE
#define CORE_PIN38_PINREG	PINF
#define CORE_PIN39_PINREG	PINF
#define CORE_PIN40_PINREG	PINF
#define CORE_PIN41_PINREG	PINF
#define CORE_PIN42_PINREG	PINF
#define CORE_PIN43_PINREG	PINF
#define CORE_PIN44_PINREG	PINF
#define CORE_PIN45_PINREG	PINF
#define CORE_ADC0_PIN		PIN_F0
#define CORE_ADC1_PIN		PIN_F1
#define CORE_ADC2_PIN		PIN_F2
#define CORE_ADC3_PIN		PIN_F3
#define CORE_ADC4_PIN		PIN_F4
#define CORE_ADC5_PIN		PIN_F5
#define CORE_ADC6_PIN		PIN_F6
#define CORE_ADC7_PIN		PIN_F7
#define CORE_RXD1_PIN           PIN_D2
#define CORE_TXD1_PIN           PIN_D3
#define CORE_XCK1_PIN           PIN_D5
#define CORE_SDA0_PIN           PIN_D1
#define CORE_SCL0_PIN           PIN_D0
#define CORE_INT0_PIN           PIN_D0
#define CORE_INT1_PIN           PIN_D1
#define CORE_INT2_PIN           PIN_D2
#define CORE_INT3_PIN           PIN_D3
#define CORE_INT4_PIN           PIN_E4
#define CORE_INT5_PIN           PIN_E5
#define CORE_INT6_PIN           PIN_E6
#define CORE_INT7_PIN           PIN_E7
#define CORE_SS0_PIN            PIN_B0
#define CORE_MOSI0_PIN          PIN_B2
#define CORE_MISO0_PIN          PIN_B3
#define CORE_SCLK0_PIN          PIN_B1
#define CORE_T0_PIN             PIN_D7
#define CORE_T1_PIN             PIN_D6
#define CORE_ICP1_PIN           PIN_D4
#define CORE_ICP3_PIN           PIN_C7
#define CORE_OC0A_PIN           PIN_B7
#define CORE_OC0B_PIN           PIN_D0
#define CORE_OC1A_PIN           PIN_B5
#define CORE_OC1B_PIN           PIN_B6
#define CORE_OC1C_PIN           PIN_B7
#define CORE_OC2A_PIN           PIN_B4
#define CORE_OC2B_PIN           PIN_D1
#define CORE_OC3A_PIN           PIN_C6
#define CORE_OC3B_PIN           PIN_C5
#define CORE_OC3C_PIN           PIN_C4
#define CORE_PCINT0_PIN		PIN_B0
#define CORE_PCINT1_PIN		PIN_B1
#define CORE_PCINT2_PIN		PIN_B2
#define CORE_PCINT3_PIN		PIN_B3
#define CORE_PCINT4_PIN		PIN_B4
#define CORE_PCINT5_PIN		PIN_B5
#define CORE_PCINT6_PIN		PIN_B6
#define CORE_PCINT7_PIN		PIN_B7
#define CORE_LED0_PIN		PIN_D6
#define CORE_PWM0_PIN		CORE_OC0B_PIN	// D0, 0
#define CORE_PWM1_PIN		CORE_OC2B_PIN	// D1, 1
#define CORE_PWM2_PIN		CORE_OC3C_PIN	// C4, 14
#define CORE_PWM3_PIN		CORE_OC3B_PIN	// C5, 15
#define CORE_PWM4_PIN		CORE_OC3A_PIN	// C6, 16
#define CORE_PWM5_PIN		CORE_OC2A_PIN	// B4, 24
#define CORE_PWM6_PIN		CORE_OC1A_PIN	// B5, 25
#define CORE_PWM7_PIN		CORE_OC1B_PIN	// B6, 26
#define CORE_PWM8_PIN		CORE_OC1C_PIN	// B7, 27
#define CORE_ANALOG0_PIN	PIN_F0
#define CORE_ANALOG1_PIN	PIN_F1
#define CORE_ANALOG2_PIN	PIN_F2
#define CORE_ANALOG3_PIN	PIN_F3
#define CORE_ANALOG4_PIN	PIN_F4
#define CORE_ANALOG5_PIN	PIN_F5
#define CORE_ANALOG6_PIN	PIN_F6
#define CORE_ANALOG7_PIN	PIN_F7

#endif


#define CORE_BIT(pin) CORE_PIN_CONCATENATE(pin, BIT)
#define CORE_BITMASK(pin) CORE_PIN_CONCATENATE(pin, BITMASK)
#define CORE_PORTREG(pin) CORE_PIN_CONCATENATE(pin, PORTREG)
#define CORE_DDRREG(pin) CORE_PIN_CONCATENATE(pin, DDRREG)
#define CORE_PINREG(pin) CORE_PIN_CONCATENATE(pin, PINREG)
#define CORE_PIN_CONCATENATE(pin, reg) (CORE_PIN ## pin ## _ ## reg)

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

// DateTimeFields follows C library "struct tm" convention, but uses much less memory
typedef struct  {
	uint8_t sec;   // 0-59
	uint8_t min;   // 0-59
	uint8_t hour;  // 0-23
	uint8_t wday;  // 0-6, 0=sunday
	uint8_t mday;  // 1-31
	uint8_t mon;   // 0-11
	uint8_t year;  // 70-206, 70=1970, 206=2106
} DateTimeFields;
void breakTime(uint32_t time, DateTimeFields &tm);  // break 32 bit time into DateTimeFields
uint32_t makeTime(const DateTimeFields &tm); // convert DateTimeFields to 32 bit time

#endif // __cplusplus
#endif
