/*
 *    This file (examples/blink.rs) is part of sam3x8e.
 *
 *    sam3x8e is free software: you can redistribute it and/or modify
 *    it under the terms of the GNU Lesser General Public License as published
 *    by the Free Software Foundation, either version 3 of the License, or
 *    (at your option) any later version.
 *
 *    sam3x8e is distributed in the hope that it will be useful,
 *    but WITHOUT ANY WARRANTY; without even the implied warranty of
 *    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *    GNU Lesser General Public License for more details.
 *
 *    You should have received a copy of the GNU Lesser General Public License
 *    along with sam3x8e.  If not, see <https://www.gnu.org/licenses/>.
 */

#![no_std]
#![no_main]

extern crate panic_halt;

extern crate cortex_m_rt;
extern crate embedded_hal;
extern crate sam3x8e;
use cortex_m_rt::entry;

pub fn delay_ms(rtt: &sam3x8e::RTT, ms: u32) {
    let y = rtt.vr.read().bits() + ms;

    // TODO: deal with overflow
    while rtt.vr.read().bits() < y {}
}

fn system_init(p: &sam3x8e::Peripherals) {
    // TODO: Set FWS (flash wait state) according to clock configuration
    {
        let efc = &p.EFC0;
        let fmr = &efc.fmr;
        fmr.write(|w| unsafe { w.fws().bits(4) });
    }

    {
        let efc = &p.EFC1;
        let fmr = &efc.fmr;
        fmr.write(|w| unsafe { w.fws().bits(4) });
    }

    let pmc = &p.PMC;

    // Initialize main oscillator
    if pmc.ckgr_mor.read().moscsel().bit_is_clear() {
        pmc.ckgr_mor.write(|w| unsafe {
            w
                // set "password"
                .key()
                .passwd()
                // Set main clock to 84 MHz
                .moscxtst()
                .bits(8)
                // Main On-Chip RC Oscillator Enable
                .moscrcen()
                .set_bit()
                // Main Crystal Oscillator Enable
                .moscxten()
                .set_bit()
        });

        let sr = &pmc.pmc_sr;
        while !sr.read().moscxts().bit_is_set() {}
    }

    // Switch to 3-20MHz Xtal oscillator
    {
        pmc.ckgr_mor.write(|w| unsafe {
            w
                // set "password"
                .key()
                .passwd()
                // Set main clock to 84 MHz
                .moscxtst()
                .bits(8)
                // Main On-Chip RC Oscillator Enable
                .moscrcen()
                .set_bit()
                // Main Crystal Oscillator Enable
                .moscxten()
                .set_bit()
                // Main Oscillator Selection
                // the 3 to 20 MHz Crystal or Ceramic Resonator-based oscillator clock is selected as the source clock of MAINCK (MOSCSEL = 1),
                .moscsel()
                .set_bit()
        });

        let sr = &pmc.pmc_sr;
        while !sr.read().moscsels().bit_is_set() {}

        // Master Clock Register
        pmc.pmc_mckr.write(|w| w.css().main_clk());
        while !sr.read().mckrdy().bit_is_set() {}
    }

    // Initialize PLLA
    {
        pmc.ckgr_pllar.write(|w| unsafe {
            w.one()
                .set_bit()
                .mula()
                .bits(0xD)
                .pllacount()
                .bits(0x3F)
                .diva()
                .bits(0x1)
        });

        let sr = &pmc.pmc_sr;
        while !sr.read().locka().bit_is_set() {}
    }

    // Switch to main clock
    {
        pmc.pmc_mckr.write(|w| w.pres().clk_2().css().main_clk());

        let sr = &pmc.pmc_sr;
        while !sr.read().mckrdy().bit_is_set() {}
    }

    // Switch to PLLA
    {
        pmc.pmc_mckr.write(|w| {
            w
                // Select clock divided by 2
                .pres()
                .clk_2()
                // PLLA Clock is selected
                .css()
                .plla_clk()
        });

        let sr = &pmc.pmc_sr;
        while !sr.read().mckrdy().bit_is_set() {}
    }
}

#[entry]
fn main() -> ! {
    let p = sam3x8e::Peripherals::take().unwrap();

    // Bring the system to a known state, similar to the Arudino initial state
    system_init(&p);

    let piod = p.PIOD;

    // PER = PIO Enable Register
    piod.per.write_with_zero(|w| w.p10().set_bit());

    // OER = Output Enable Register
    piod.oer.write_with_zero(|w| w.p10().set_bit());

    // Set the pin (PD10) high to disable the LED
    piod.sodr.write_with_zero(|w| w.p10().set_bit());

    // Assume we've got a 32 MHz clock
    p.RTT.mr.write(|w| unsafe { w.bits(32) });

    let mut on = true;

    loop {
        if on {
            // Pulling the pins down turns on the LED
            piod.codr.write_with_zero(|w| w.p10().set_bit());
        } else {
            piod.sodr.write_with_zero(|w| w.p10().set_bit());
        }

        on = !on;
        delay_ms(&p.RTT, 2500_u32);
    }
}
