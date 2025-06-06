/* USB API for Teensy USB Development Board
 * http://www.pjrc.com/teensy/teensyduino.html
 * Copyright (c) 2008 PJRC.COM, LLC
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

#include <avr/io.h>
#include <stdint.h>
#include "usb_common.h"
#include "usb_private.h"
#include "usb_api.h"
#include "core_pins.h"

// This file is used only for Teensy 2.0 and Teensy++ 2.0

// Public Methods //////////////////////////////////////////////////////////////

void usb_serial_class::begin(long speed)
{
	// make sure USB is initialized
	peek_buf = -1;
	usb_init();
	uint16_t begin_wait = (uint16_t)millis();
	while (1) {
		// wait for the host to finish enumeration
		if (usb_configuration) {
			delay(200);  // a little time for host to load a driver
			return;
		}
		// or for suspend mode (powered without USB)
		if (usb_suspended) {
			uint16_t begin_suspend = (uint16_t)millis();
			while (usb_suspended) {
				// must remain suspended for a while, because
				// normal USB enumeration causes brief suspend
				// states, typically under 0.1 second
				if ((uint16_t)millis() - begin_suspend > 250) {
					return;
				}
			}
		}
		// ... or a timout (powered by a USB power adaptor that
		// wiggles the data lines to keep a USB device charging)
		if ((uint16_t)millis() - begin_wait > 2500) return;
	}
}

void usb_serial_class::end()
{
	usb_shutdown();
	delay(25);
}

// number of bytes available in the receive buffer
int usb_serial_class::available()
{
        uint8_t n=0, i, intr_state;

        intr_state = SREG;
        cli();
        if (usb_configuration) {
                UENUM = CDC_RX_ENDPOINT;
                n = UEBCLX;
		if (!n) {
			i = UEINTX;
			if (i & (1<<RXOUTI) && !(i & (1<<RWAL))) UEINTX = 0x6B;
		}
        }
        SREG = intr_state;
	if (peek_buf >= 0 && n < 255) n++;
        return n;
}

int usb_serial_class::peek()
{
	if (peek_buf < 0) peek_buf = read();
	return peek_buf;
}

// get the next character, or -1 if nothing received
int usb_serial_class::read(void)
{
        uint8_t c, intr_state;

	if (peek_buf >= 0) {
		c = peek_buf;
		peek_buf = -1;
		return c;
	}
        // interrupts are disabled so these functions can be
        // used from the main program or interrupt context,
        // even both in the same program!
        intr_state = SREG;
        cli();
        if (!usb_configuration) {
                SREG = intr_state;
                return -1;
        }
        UENUM = CDC_RX_ENDPOINT;
	retry:
	c = UEINTX;
        if (!(c & (1<<RWAL))) {
                // no data in buffer
		if (c & (1<<RXOUTI)) {
			UEINTX = 0x6B;
			goto retry;
		}
                SREG = intr_state;
                return -1;
        }
        // take one byte out of the buffer
        c = UEDATX;
        // if this drained the buffer, release it
        if (!(UEINTX & (1<<RWAL))) UEINTX = 0x6B;
        SREG = intr_state;
        return c;
}

// discard any buffered input
void usb_serial_class::flush()
{
        uint8_t intr_state;

        if (usb_configuration) {
                intr_state = SREG;
                cli();
                UENUM = CDC_RX_ENDPOINT;
                while ((UEINTX & (1<<RWAL))) {
                        UEINTX = 0x6B;
                }
                SREG = intr_state;
        }
	peek_buf = -1;
}
#if 0
// transmit a character.
void usb_serial_class::write(uint8_t c)
{
        uint8_t timeout, intr_state;

        // if we're not online (enumerated and configured), error
        if (!usb_configuration) return;
        // interrupts are disabled so these functions can be
        // used from the main program or interrupt context,
        // even both in the same program!
        intr_state = SREG;
        cli();
        UENUM = CDC_TX_ENDPOINT;
        // if we gave up due to timeout before, don't wait again
        if (transmit_previous_timeout) {
                if (!(UEINTX & (1<<RWAL))) {
                        SREG = intr_state;
                        return;
                }
                transmit_previous_timeout = 0;
        }
        // wait for the FIFO to be ready to accept data
        timeout = UDFNUML + TRANSMIT_TIMEOUT;
        while (1) {
                // are we ready to transmit?
                if (UEINTX & (1<<RWAL)) break;
                SREG = intr_state;
                // have we waited too long?  This happens if the user
                // is not running an application that is listening
                if (UDFNUML == timeout) {
                        transmit_previous_timeout = 1;
                        return;
                }
                // has the USB gone offline?
                if (!usb_configuration) return;
                // get ready to try checking again
                intr_state = SREG;
                cli();
                UENUM = CDC_TX_ENDPOINT;
        }
        // actually write the byte into the FIFO
        UEDATX = c;
        // if this completed a packet, transmit it now!
        if (!(UEINTX & (1<<RWAL))) UEINTX = 0x3A;
        transmit_flush_timer = TRANSMIT_FLUSH_TIMEOUT;
        SREG = intr_state;
}
#endif


// transmit a block of data
size_t usb_serial_class::write(const uint8_t *buffer, uint16_t size)
{
	uint8_t timeout, intr_state, write_size;
	size_t count=0;

	// if we're not online (enumerated and configured), error
	if (!usb_configuration) {
		setWriteError();
		goto end;
	}
	// interrupts are disabled so these functions can be
	// used from the main program or interrupt context,
	// even both in the same program!
	intr_state = SREG;
	cli();
	UENUM = CDC_TX_ENDPOINT;
	// if we gave up due to timeout before, don't wait again
	if (transmit_previous_timeout) {
		if (!(UEINTX & (1<<RWAL))) {
			SREG = intr_state;
			setWriteError();
			goto end;
		}
		transmit_previous_timeout = 0;
	}
	// each iteration of this loop transmits a packet
	while (size) {
		// wait for the FIFO to be ready to accept data
		timeout = UDFNUML + TRANSMIT_TIMEOUT;
		while (1) {
			// are we ready to transmit?
			if (UEINTX & (1<<RWAL)) break;
			SREG = intr_state;
			// have we waited too long?  This happens if the user
			// is not running an application that is listening
			if (UDFNUML == timeout) {
				transmit_previous_timeout = 1;
				setWriteError();
				goto end;
			}
			// has the USB gone offline?
			if (!usb_configuration) {
				setWriteError();
				goto end;
			}
			// get ready to try checking again
			intr_state = SREG;
			cli();
			UENUM = CDC_TX_ENDPOINT;
		}

		// compute how many bytes will fit into the next packet
		write_size = CDC_TX_SIZE - UEBCLX;
		if (write_size > size) write_size = size;
		size -= write_size;
		count += write_size;

#define ASM_COPY1(src, dest, tmp) "ld " tmp ", " src "\n\t" "st " dest ", " tmp "\n\t"
#define ASM_COPY2(src, dest, tmp) ASM_COPY1(src, dest, tmp) ASM_COPY1(src, dest, tmp)
#define ASM_COPY4(src, dest, tmp) ASM_COPY2(src, dest, tmp) ASM_COPY2(src, dest, tmp)
#define ASM_COPY8(src, dest, tmp) ASM_COPY4(src, dest, tmp) ASM_COPY4(src, dest, tmp)

#if 1
		// write the packet
		do {
			uint8_t tmp;
			asm volatile(
			"L%=begin:"					"\n\t"
				"ldi	r30, %4"			"\n\t"
				"sub	r30, %3"			"\n\t"
				"cpi	r30, %4"			"\n\t"
				"brsh	L%=err"				"\n\t"
				"lsl	r30"				"\n\t"
				"clr	r31"				"\n\t"
				"subi	r30, lo8(-(pm(L%=table)))"	"\n\t"
				"sbci	r31, hi8(-(pm(L%=table)))"	"\n\t"
				"ijmp"					"\n\t"
			"L%=err:"					"\n\t"
				"rjmp	L%=end"				"\n\t"
			"L%=table:"					"\n\t"
				#if (CDC_TX_SIZE == 64)
				ASM_COPY8("Y+", "X", "%1")
				ASM_COPY8("Y+", "X", "%1")
				ASM_COPY8("Y+", "X", "%1")
				ASM_COPY8("Y+", "X", "%1")
				#endif
				#if (CDC_TX_SIZE >= 32)
				ASM_COPY8("Y+", "X", "%1")
				ASM_COPY8("Y+", "X", "%1")
				#endif
				#if (CDC_TX_SIZE >= 16)
				ASM_COPY8("Y+", "X", "%1")
				#endif
				ASM_COPY8("Y+", "X", "%1")
			"L%=end:"					"\n\t"
				: "+y" (buffer), "=r" (tmp)
				: "x" (&UEDATX), "r" (write_size), "M" (CDC_TX_SIZE)
				: "r30", "r31"
			);
		} while (0);
#endif
		// if this completed a packet, transmit it now!
		if (!(UEINTX & (1<<RWAL))) UEINTX = 0x3A;
		transmit_flush_timer = TRANSMIT_FLUSH_TIMEOUT;
	}
	SREG = intr_state;
end:
	return count;
}

// transmit a string
/*
void usb_serial_class::write(const char *str)
{
	uint16_t size=0;
	const char *p=str;

	while (*p++) size++;
	if (size) write((const uint8_t *)str, size);
}
*/

// These are Teensy-specific extensions to the Serial object

// immediately transmit any buffered output.
// This doesn't actually transmit the data - that is impossible!
// USB devices only transmit when the host allows, so the best
// we can do is release the FIFO buffer for when the host wants it
void usb_serial_class::send_now(void)
{
        uint8_t intr_state;

        intr_state = SREG;
        cli();
        if (usb_configuration && transmit_flush_timer) {
                UENUM = CDC_TX_ENDPOINT;
                UEINTX = 0x3A;
                transmit_flush_timer = 0;
        }
        SREG = intr_state;
}

uint32_t usb_serial_class::baud(void)
{
	return *(uint32_t *)cdc_line_coding;
}

uint8_t usb_serial_class::stopbits(void)
{
	return cdc_line_coding[4];
}

uint8_t usb_serial_class::paritytype(void)
{
	return cdc_line_coding[5];
}

uint8_t usb_serial_class::numbits(void)
{
	return cdc_line_coding[6];
}

uint8_t usb_serial_class::dtr(void)
{
	return (cdc_line_rtsdtr & USB_SERIAL_DTR) ? 1 : 0;
}

uint8_t usb_serial_class::rts(void)
{
	return (cdc_line_rtsdtr & USB_SERIAL_RTS) ? 1 : 0;
}

usb_serial_class::operator bool()
{
	if (usb_configuration &&
	  (cdc_line_rtsdtr & (USB_SERIAL_DTR | USB_SERIAL_RTS))) {
		return true;
	}
	return false;
}

void usb_keyboard_class::set_modifier(uint16_t c)
{
	keyboard_report_data[0] = (uint8_t)c;
}
void usb_keyboard_class::set_key1(uint8_t c)
{
	keyboard_report_data[2] = c;
}
void usb_keyboard_class::set_key2(uint8_t c)
{
	keyboard_report_data[3] = c;
}
void usb_keyboard_class::set_key3(uint8_t c)
{
	keyboard_report_data[4] = c;
}
void usb_keyboard_class::set_key4(uint8_t c)
{
	keyboard_report_data[5] = c;
}
void usb_keyboard_class::set_key5(uint8_t c)
{
	keyboard_report_data[6] = c;
}
void usb_keyboard_class::set_key6(uint8_t c)
{
	keyboard_report_data[7] = c;
}

void usb_keyboard_class::send_now(void)
{
        uint8_t intr_state, timeout;

        if (!usb_configuration) return;
        intr_state = SREG;
        cli();
        UENUM = KEYBOARD_ENDPOINT;
        timeout = UDFNUML + 50;
        while (1) {
                // are we ready to transmit?
                if (UEINTX & (1<<RWAL)) break;
                SREG = intr_state;
                // has the USB gone offline?
                if (!usb_configuration) return;
                // have we waited too long?
                if (UDFNUML == timeout) return;
                // get ready to try checking again
                intr_state = SREG;
                cli();
                UENUM = KEYBOARD_ENDPOINT;
        }
        UEDATX = keyboard_report_data[0];
        UEDATX = keyboard_report_data[1];
        UEDATX = keyboard_report_data[2];
        UEDATX = keyboard_report_data[3];
        UEDATX = keyboard_report_data[4];
        UEDATX = keyboard_report_data[5];
        UEDATX = keyboard_report_data[6];
        UEDATX = keyboard_report_data[7];
        UEINTX = 0x3A;
        keyboard_idle_count = 0;
        SREG = intr_state;
}

void usb_keyboard_class::releaseAll(void)
{
	uint8_t i, anybits;

	anybits = keyboard_report_data[0];
	for (i=2; i < 8; i++) {
		anybits |= keyboard_report_data[i];
		keyboard_report_data[i] = 0;
	}
	if (!anybits) return;
	keyboard_report_data[0] = 0;
	send_now();
}






void usb_mouse_class::move(int8_t x, int8_t y, int8_t wheel, int8_t horiz)
{
        uint8_t intr_state, timeout;

        if (!usb_configuration) return;
        if (x == -128) x = -127;
        if (y == -128) y = -127;
        if (wheel == -128) wheel = -127;
        if (horiz == -128) horiz = -127;
        intr_state = SREG;
        cli();
        UENUM = MOUSE_ENDPOINT;
        timeout = UDFNUML + 50;
        while (1) {
                // are we ready to transmit?
                if (UEINTX & (1<<RWAL)) break;
                SREG = intr_state;
                // has the USB gone offline?
                if (!usb_configuration) return;
                // have we waited too long?
                if (UDFNUML == timeout) return;
                // get ready to try checking again
                intr_state = SREG;
                cli();
                UENUM = MOUSE_ENDPOINT;
        }
        UEDATX = mouse_buttons;
        UEDATX = x;
        UEDATX = y;
        UEDATX = wheel;
        UEDATX = horiz;
        UEINTX = 0x3A;
        SREG = intr_state;
}

void usb_mouse_class::click(uint8_t b)
{
        mouse_buttons = b;
        move(0, 0);
        mouse_buttons = 0;
        move(0, 0);
}

void usb_mouse_class::scroll(int8_t wheel, int8_t horiz)
{
        move(0, 0, wheel, horiz);
}

void usb_mouse_class::set_buttons(uint8_t left, uint8_t middle, uint8_t right, uint8_t back, uint8_t forward)
{
        uint8_t mask=0;

        if (left) mask |= 1;
        if (middle) mask |= 4;
        if (right) mask |= 2;
        if (back) mask |= 8;
        if (forward) mask |= 16;
        mouse_buttons = mask;
        move(0, 0);
}

void usb_mouse_class::press(uint8_t b)
{
        uint8_t prev = mouse_buttons;
        mouse_buttons |= (b & 7);
        if (mouse_buttons != prev) move(0, 0);
}

void usb_mouse_class::release(uint8_t b)
{
        uint8_t prev = mouse_buttons;
        mouse_buttons &= ~(b & 7);
        if (mouse_buttons != prev) move(0, 0);
}

bool usb_mouse_class::isPressed(uint8_t b)
{
        return ((mouse_buttons & (b & 7)) != 0);
}



void usb_joystick_class::send_now(void)
{
        uint8_t intr_state, timeout;

        if (!usb_configuration) return;
        intr_state = SREG;
        cli();
        UENUM = JOYSTICK_ENDPOINT;
        timeout = UDFNUML + 50;
        while (1) {
                // are we ready to transmit?
                if (UEINTX & (1<<RWAL)) break;
                SREG = intr_state;
                // has the USB gone offline?
                if (!usb_configuration) return;
                // have we waited too long?
                if (UDFNUML == timeout) return;
                // get ready to try checking again
                intr_state = SREG;
                cli();
                UENUM = JOYSTICK_ENDPOINT;
        }
        UEDATX = joystick_report_data[0];
        UEDATX = joystick_report_data[1];
        UEDATX = joystick_report_data[2];
        UEDATX = joystick_report_data[3];
        UEDATX = joystick_report_data[4];
        UEDATX = joystick_report_data[5];
        UEDATX = joystick_report_data[6];
        UEDATX = joystick_report_data[7];
        UEDATX = joystick_report_data[8];
        UEDATX = joystick_report_data[9];
        UEDATX = joystick_report_data[10];
        UEDATX = joystick_report_data[11];
        UEINTX = 0x3A;
        SREG = intr_state;
}







// Preinstantiate Objects //////////////////////////////////////////////////////

usb_serial_class        Serial = usb_serial_class();
usb_keyboard_class      Keyboard = usb_keyboard_class();
usb_mouse_class         Mouse = usb_mouse_class();
usb_joystick_class      Joystick = usb_joystick_class();

