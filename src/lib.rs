#![doc = "Peripheral access API for SAM3X8E microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[cfg(feature = "rt")]
extern "C" {
    fn ID_PMC();
    fn ID_EFC0();
    fn ID_EFC1();
    fn ID_UART();
    fn ID_PIOA();
    fn ID_PIOB();
    fn ID_PIOC();
    fn ID_PIOD();
    fn ID_USART0();
    fn ID_USART1();
    fn ID_USART2();
    fn ID_USART3();
    fn ID_HSMCI();
    fn ID_TWI0();
    fn ID_TWI1();
    fn ID_SPI0();
    fn ID_SSC();
    fn ID_TC0();
    fn ID_TC1();
    fn ID_TC2();
    fn ID_TC3();
    fn ID_TC4();
    fn ID_TC5();
    fn ID_TC6();
    fn ID_TC7();
    fn ID_TC8();
    fn ID_PWM();
    fn ID_ADC();
    fn ID_DACC();
    fn ID_DMAC();
    fn ID_UOTGHS();
    fn ID_TRNG();
    fn ID_EMAC();
    fn ID_CAN0();
    fn ID_CAN1();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 45] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: ID_PMC },
    Vector { _handler: ID_EFC0 },
    Vector { _handler: ID_EFC1 },
    Vector { _handler: ID_UART },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: ID_PIOA },
    Vector { _handler: ID_PIOB },
    Vector { _handler: ID_PIOC },
    Vector { _handler: ID_PIOD },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: ID_USART0,
    },
    Vector {
        _handler: ID_USART1,
    },
    Vector {
        _handler: ID_USART2,
    },
    Vector {
        _handler: ID_USART3,
    },
    Vector { _handler: ID_HSMCI },
    Vector { _handler: ID_TWI0 },
    Vector { _handler: ID_TWI1 },
    Vector { _handler: ID_SPI0 },
    Vector { _reserved: 0 },
    Vector { _handler: ID_SSC },
    Vector { _handler: ID_TC0 },
    Vector { _handler: ID_TC1 },
    Vector { _handler: ID_TC2 },
    Vector { _handler: ID_TC3 },
    Vector { _handler: ID_TC4 },
    Vector { _handler: ID_TC5 },
    Vector { _handler: ID_TC6 },
    Vector { _handler: ID_TC7 },
    Vector { _handler: ID_TC8 },
    Vector { _handler: ID_PWM },
    Vector { _handler: ID_ADC },
    Vector { _handler: ID_DACC },
    Vector { _handler: ID_DMAC },
    Vector {
        _handler: ID_UOTGHS,
    },
    Vector { _handler: ID_TRNG },
    Vector { _handler: ID_EMAC },
    Vector { _handler: ID_CAN0 },
    Vector { _handler: ID_CAN1 },
];
#[doc = r" Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "5 - ID_PMC"]
    ID_PMC,
    #[doc = "6 - ID_EFC0"]
    ID_EFC0,
    #[doc = "7 - ID_EFC1"]
    ID_EFC1,
    #[doc = "8 - ID_UART"]
    ID_UART,
    #[doc = "11 - ID_PIOA"]
    ID_PIOA,
    #[doc = "12 - ID_PIOB"]
    ID_PIOB,
    #[doc = "13 - ID_PIOC"]
    ID_PIOC,
    #[doc = "14 - ID_PIOD"]
    ID_PIOD,
    #[doc = "17 - ID_USART0"]
    ID_USART0,
    #[doc = "18 - ID_USART1"]
    ID_USART1,
    #[doc = "19 - ID_USART2"]
    ID_USART2,
    #[doc = "20 - ID_USART3"]
    ID_USART3,
    #[doc = "21 - ID_HSMCI"]
    ID_HSMCI,
    #[doc = "22 - ID_TWI0"]
    ID_TWI0,
    #[doc = "23 - ID_TWI1"]
    ID_TWI1,
    #[doc = "24 - ID_SPI0"]
    ID_SPI0,
    #[doc = "26 - ID_SSC"]
    ID_SSC,
    #[doc = "27 - ID_TC0"]
    ID_TC0,
    #[doc = "28 - ID_TC1"]
    ID_TC1,
    #[doc = "29 - ID_TC2"]
    ID_TC2,
    #[doc = "30 - ID_TC3"]
    ID_TC3,
    #[doc = "31 - ID_TC4"]
    ID_TC4,
    #[doc = "32 - ID_TC5"]
    ID_TC5,
    #[doc = "33 - ID_TC6"]
    ID_TC6,
    #[doc = "34 - ID_TC7"]
    ID_TC7,
    #[doc = "35 - ID_TC8"]
    ID_TC8,
    #[doc = "36 - ID_PWM"]
    ID_PWM,
    #[doc = "37 - ID_ADC"]
    ID_ADC,
    #[doc = "38 - ID_DACC"]
    ID_DACC,
    #[doc = "39 - ID_DMAC"]
    ID_DMAC,
    #[doc = "40 - ID_UOTGHS"]
    ID_UOTGHS,
    #[doc = "41 - ID_TRNG"]
    ID_TRNG,
    #[doc = "42 - ID_EMAC"]
    ID_EMAC,
    #[doc = "43 - ID_CAN0"]
    ID_CAN0,
    #[doc = "44 - ID_CAN1"]
    ID_CAN1,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::ID_PMC => 5,
            Interrupt::ID_EFC0 => 6,
            Interrupt::ID_EFC1 => 7,
            Interrupt::ID_UART => 8,
            Interrupt::ID_PIOA => 11,
            Interrupt::ID_PIOB => 12,
            Interrupt::ID_PIOC => 13,
            Interrupt::ID_PIOD => 14,
            Interrupt::ID_USART0 => 17,
            Interrupt::ID_USART1 => 18,
            Interrupt::ID_USART2 => 19,
            Interrupt::ID_USART3 => 20,
            Interrupt::ID_HSMCI => 21,
            Interrupt::ID_TWI0 => 22,
            Interrupt::ID_TWI1 => 23,
            Interrupt::ID_SPI0 => 24,
            Interrupt::ID_SSC => 26,
            Interrupt::ID_TC0 => 27,
            Interrupt::ID_TC1 => 28,
            Interrupt::ID_TC2 => 29,
            Interrupt::ID_TC3 => 30,
            Interrupt::ID_TC4 => 31,
            Interrupt::ID_TC5 => 32,
            Interrupt::ID_TC6 => 33,
            Interrupt::ID_TC7 => 34,
            Interrupt::ID_TC8 => 35,
            Interrupt::ID_PWM => 36,
            Interrupt::ID_ADC => 37,
            Interrupt::ID_DACC => 38,
            Interrupt::ID_DMAC => 39,
            Interrupt::ID_UOTGHS => 40,
            Interrupt::ID_TRNG => 41,
            Interrupt::ID_EMAC => 42,
            Interrupt::ID_CAN0 => 43,
            Interrupt::ID_CAN1 => 44,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "High Speed MultiMedia Card Interface"]
pub struct HSMCI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HSMCI {}
impl HSMCI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const hsmci::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for HSMCI {
    type Target = hsmci::RegisterBlock;
    fn deref(&self) -> &hsmci::RegisterBlock {
        unsafe { &*HSMCI::ptr() }
    }
}
#[doc = "High Speed MultiMedia Card Interface"]
pub mod hsmci;
#[doc = "Synchronous Serial Controller"]
pub struct SSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSC {}
impl SSC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ssc::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for SSC {
    type Target = ssc::RegisterBlock;
    fn deref(&self) -> &ssc::RegisterBlock {
        unsafe { &*SSC::ptr() }
    }
}
#[doc = "Synchronous Serial Controller"]
pub mod ssc;
#[doc = "Serial Peripheral Interface 0"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi0::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &spi0::RegisterBlock {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "Serial Peripheral Interface 0"]
pub mod spi0;
#[doc = "Timer Counter 0"]
pub struct TC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC0 {}
impl TC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tc0::RegisterBlock {
        1074266112 as *const _
    }
}
impl Deref for TC0 {
    type Target = tc0::RegisterBlock;
    fn deref(&self) -> &tc0::RegisterBlock {
        unsafe { &*TC0::ptr() }
    }
}
#[doc = "Timer Counter 0"]
pub mod tc0;
#[doc = "Timer Counter 1"]
pub struct TC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC1 {}
impl TC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tc1::RegisterBlock {
        1074282496 as *const _
    }
}
impl Deref for TC1 {
    type Target = tc1::RegisterBlock;
    fn deref(&self) -> &tc1::RegisterBlock {
        unsafe { &*TC1::ptr() }
    }
}
#[doc = "Timer Counter 1"]
pub mod tc1;
#[doc = "Timer Counter 2"]
pub struct TC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC2 {}
impl TC2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tc2::RegisterBlock {
        1074298880 as *const _
    }
}
impl Deref for TC2 {
    type Target = tc2::RegisterBlock;
    fn deref(&self) -> &tc2::RegisterBlock {
        unsafe { &*TC2::ptr() }
    }
}
#[doc = "Timer Counter 2"]
pub mod tc2;
#[doc = "Two-wire Interface 0"]
pub struct TWI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWI0 {}
impl TWI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twi0::RegisterBlock {
        1074315264 as *const _
    }
}
impl Deref for TWI0 {
    type Target = twi0::RegisterBlock;
    fn deref(&self) -> &twi0::RegisterBlock {
        unsafe { &*TWI0::ptr() }
    }
}
#[doc = "Two-wire Interface 0"]
pub mod twi0;
#[doc = "Two-wire Interface 1"]
pub struct TWI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWI1 {}
impl TWI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twi1::RegisterBlock {
        1074331648 as *const _
    }
}
impl Deref for TWI1 {
    type Target = twi1::RegisterBlock;
    fn deref(&self) -> &twi1::RegisterBlock {
        unsafe { &*TWI1::ptr() }
    }
}
#[doc = "Two-wire Interface 1"]
pub mod twi1;
#[doc = "Pulse Width Modulation Controller"]
pub struct PWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM {}
impl PWM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwm::RegisterBlock {
        1074348032 as *const _
    }
}
impl Deref for PWM {
    type Target = pwm::RegisterBlock;
    fn deref(&self) -> &pwm::RegisterBlock {
        unsafe { &*PWM::ptr() }
    }
}
#[doc = "Pulse Width Modulation Controller"]
pub mod pwm;
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 0"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart0::RegisterBlock {
        1074364416 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &usart0::RegisterBlock {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 0"]
pub mod usart0;
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 1"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1074380800 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 1"]
pub mod usart1;
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 2"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart2::RegisterBlock {
        1074397184 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart2::RegisterBlock;
    fn deref(&self) -> &usart2::RegisterBlock {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 2"]
pub mod usart2;
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 3"]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart3::RegisterBlock {
        1074413568 as *const _
    }
}
impl Deref for USART3 {
    type Target = usart3::RegisterBlock;
    fn deref(&self) -> &usart3::RegisterBlock {
        unsafe { &*USART3::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter 3"]
pub mod usart3;
#[doc = "USB On-The-Go Interface"]
pub struct UOTGHS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UOTGHS {}
impl UOTGHS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uotghs::RegisterBlock {
        1074446336 as *const _
    }
}
impl Deref for UOTGHS {
    type Target = uotghs::RegisterBlock;
    fn deref(&self) -> &uotghs::RegisterBlock {
        unsafe { &*UOTGHS::ptr() }
    }
}
#[doc = "USB On-The-Go Interface"]
pub mod uotghs;
#[doc = "Ethernet MAC 10/100"]
pub struct EMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMAC {}
impl EMAC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const emac::RegisterBlock {
        1074462720 as *const _
    }
}
impl Deref for EMAC {
    type Target = emac::RegisterBlock;
    fn deref(&self) -> &emac::RegisterBlock {
        unsafe { &*EMAC::ptr() }
    }
}
#[doc = "Ethernet MAC 10/100"]
pub mod emac;
#[doc = "Controller Area Network 0"]
pub struct CAN0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN0 {}
impl CAN0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can0::RegisterBlock {
        1074479104 as *const _
    }
}
impl Deref for CAN0 {
    type Target = can0::RegisterBlock;
    fn deref(&self) -> &can0::RegisterBlock {
        unsafe { &*CAN0::ptr() }
    }
}
#[doc = "Controller Area Network 0"]
pub mod can0;
#[doc = "Controller Area Network 1"]
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN1 {}
impl CAN1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can1::RegisterBlock {
        1074495488 as *const _
    }
}
impl Deref for CAN1 {
    type Target = can1::RegisterBlock;
    fn deref(&self) -> &can1::RegisterBlock {
        unsafe { &*CAN1::ptr() }
    }
}
#[doc = "Controller Area Network 1"]
pub mod can1;
#[doc = "True Random Number Generator"]
pub struct TRNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRNG {}
impl TRNG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const trng::RegisterBlock {
        1074511872 as *const _
    }
}
impl Deref for TRNG {
    type Target = trng::RegisterBlock;
    fn deref(&self) -> &trng::RegisterBlock {
        unsafe { &*TRNG::ptr() }
    }
}
#[doc = "True Random Number Generator"]
pub mod trng;
#[doc = "Analog-to-digital Converter"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc::RegisterBlock {
        1074528256 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &adc::RegisterBlock {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Analog-to-digital Converter"]
pub mod adc;
#[doc = "DMA Controller"]
pub struct DMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMAC {}
impl DMAC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dmac::RegisterBlock {
        1074544640 as *const _
    }
}
impl Deref for DMAC {
    type Target = dmac::RegisterBlock;
    fn deref(&self) -> &dmac::RegisterBlock {
        unsafe { &*DMAC::ptr() }
    }
}
#[doc = "DMA Controller"]
pub mod dmac;
#[doc = "Digital-to-Analog Converter Controller"]
pub struct DACC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DACC {}
impl DACC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dacc::RegisterBlock {
        1074561024 as *const _
    }
}
impl Deref for DACC {
    type Target = dacc::RegisterBlock;
    fn deref(&self) -> &dacc::RegisterBlock {
        unsafe { &*DACC::ptr() }
    }
}
#[doc = "Digital-to-Analog Converter Controller"]
pub mod dacc;
#[doc = "Static Memory Controller"]
pub struct SMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMC {}
impl SMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const smc::RegisterBlock {
        1074659328 as *const _
    }
}
impl Deref for SMC {
    type Target = smc::RegisterBlock;
    fn deref(&self) -> &smc::RegisterBlock {
        unsafe { &*SMC::ptr() }
    }
}
#[doc = "Static Memory Controller"]
pub mod smc;
#[doc = "AHB Bus Matrix"]
pub struct MATRIX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MATRIX {}
impl MATRIX {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const matrix::RegisterBlock {
        1074660352 as *const _
    }
}
impl Deref for MATRIX {
    type Target = matrix::RegisterBlock;
    fn deref(&self) -> &matrix::RegisterBlock {
        unsafe { &*MATRIX::ptr() }
    }
}
#[doc = "AHB Bus Matrix"]
pub mod matrix;
#[doc = "Power Management Controller"]
pub struct PMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMC {}
impl PMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pmc::RegisterBlock {
        1074660864 as *const _
    }
}
impl Deref for PMC {
    type Target = pmc::RegisterBlock;
    fn deref(&self) -> &pmc::RegisterBlock {
        unsafe { &*PMC::ptr() }
    }
}
#[doc = "Power Management Controller"]
pub mod pmc;
#[doc = "Universal Asynchronous Receiver Transmitter"]
pub struct UART {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART {}
impl UART {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart::RegisterBlock {
        1074661376 as *const _
    }
}
impl Deref for UART {
    type Target = uart::RegisterBlock;
    fn deref(&self) -> &uart::RegisterBlock {
        unsafe { &*UART::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter"]
pub mod uart;
#[doc = "Chip Identifier"]
pub struct CHIPID {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CHIPID {}
impl CHIPID {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const chipid::RegisterBlock {
        1074661696 as *const _
    }
}
impl Deref for CHIPID {
    type Target = chipid::RegisterBlock;
    fn deref(&self) -> &chipid::RegisterBlock {
        unsafe { &*CHIPID::ptr() }
    }
}
#[doc = "Chip Identifier"]
pub mod chipid;
#[doc = "Embedded Flash Controller 0"]
pub struct EFC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EFC0 {}
impl EFC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const efc0::RegisterBlock {
        1074661888 as *const _
    }
}
impl Deref for EFC0 {
    type Target = efc0::RegisterBlock;
    fn deref(&self) -> &efc0::RegisterBlock {
        unsafe { &*EFC0::ptr() }
    }
}
#[doc = "Embedded Flash Controller 0"]
pub mod efc0;
#[doc = "Embedded Flash Controller 1"]
pub struct EFC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EFC1 {}
impl EFC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const efc1::RegisterBlock {
        1074662400 as *const _
    }
}
impl Deref for EFC1 {
    type Target = efc1::RegisterBlock;
    fn deref(&self) -> &efc1::RegisterBlock {
        unsafe { &*EFC1::ptr() }
    }
}
#[doc = "Embedded Flash Controller 1"]
pub mod efc1;
#[doc = "Parallel Input/Output Controller A"]
pub struct PIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOA {}
impl PIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pioa::RegisterBlock {
        1074662912 as *const _
    }
}
impl Deref for PIOA {
    type Target = pioa::RegisterBlock;
    fn deref(&self) -> &pioa::RegisterBlock {
        unsafe { &*PIOA::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller A"]
pub mod pioa;
#[doc = "Parallel Input/Output Controller B"]
pub struct PIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOB {}
impl PIOB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const piob::RegisterBlock {
        1074663424 as *const _
    }
}
impl Deref for PIOB {
    type Target = piob::RegisterBlock;
    fn deref(&self) -> &piob::RegisterBlock {
        unsafe { &*PIOB::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller B"]
pub mod piob;
#[doc = "Parallel Input/Output Controller C"]
pub struct PIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOC {}
impl PIOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pioc::RegisterBlock {
        1074663936 as *const _
    }
}
impl Deref for PIOC {
    type Target = pioc::RegisterBlock;
    fn deref(&self) -> &pioc::RegisterBlock {
        unsafe { &*PIOC::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller C"]
pub mod pioc;
#[doc = "Parallel Input/Output Controller D"]
pub struct PIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOD {}
impl PIOD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const piod::RegisterBlock {
        1074664448 as *const _
    }
}
impl Deref for PIOD {
    type Target = piod::RegisterBlock;
    fn deref(&self) -> &piod::RegisterBlock {
        unsafe { &*PIOD::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller D"]
pub mod piod;
#[doc = "Reset Controller"]
pub struct RSTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSTC {}
impl RSTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rstc::RegisterBlock {
        1074665984 as *const _
    }
}
impl Deref for RSTC {
    type Target = rstc::RegisterBlock;
    fn deref(&self) -> &rstc::RegisterBlock {
        unsafe { &*RSTC::ptr() }
    }
}
#[doc = "Reset Controller"]
pub mod rstc;
#[doc = "Supply Controller"]
pub struct SUPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SUPC {}
impl SUPC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const supc::RegisterBlock {
        1074666000 as *const _
    }
}
impl Deref for SUPC {
    type Target = supc::RegisterBlock;
    fn deref(&self) -> &supc::RegisterBlock {
        unsafe { &*SUPC::ptr() }
    }
}
#[doc = "Supply Controller"]
pub mod supc;
#[doc = "Real-time Timer"]
pub struct RTT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTT {}
impl RTT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtt::RegisterBlock {
        1074666032 as *const _
    }
}
impl Deref for RTT {
    type Target = rtt::RegisterBlock;
    fn deref(&self) -> &rtt::RegisterBlock {
        unsafe { &*RTT::ptr() }
    }
}
#[doc = "Real-time Timer"]
pub mod rtt;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdt::RegisterBlock {
        1074666064 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    fn deref(&self) -> &wdt::RegisterBlock {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[doc = "Real-time Clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1074666080 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-time Clock"]
pub mod rtc;
#[doc = "General Purpose Backup Register"]
pub struct GPBR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPBR {}
impl GPBR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpbr::RegisterBlock {
        1074666128 as *const _
    }
}
impl Deref for GPBR {
    type Target = gpbr::RegisterBlock;
    fn deref(&self) -> &gpbr::RegisterBlock {
        unsafe { &*GPBR::ptr() }
    }
}
#[doc = "General Purpose Backup Register"]
pub mod gpbr;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "HSMCI"]
    pub HSMCI: HSMCI,
    #[doc = "SSC"]
    pub SSC: SSC,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "TC0"]
    pub TC0: TC0,
    #[doc = "TC1"]
    pub TC1: TC1,
    #[doc = "TC2"]
    pub TC2: TC2,
    #[doc = "TWI0"]
    pub TWI0: TWI0,
    #[doc = "TWI1"]
    pub TWI1: TWI1,
    #[doc = "PWM"]
    pub PWM: PWM,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "UOTGHS"]
    pub UOTGHS: UOTGHS,
    #[doc = "EMAC"]
    pub EMAC: EMAC,
    #[doc = "CAN0"]
    pub CAN0: CAN0,
    #[doc = "CAN1"]
    pub CAN1: CAN1,
    #[doc = "TRNG"]
    pub TRNG: TRNG,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "DMAC"]
    pub DMAC: DMAC,
    #[doc = "DACC"]
    pub DACC: DACC,
    #[doc = "SMC"]
    pub SMC: SMC,
    #[doc = "MATRIX"]
    pub MATRIX: MATRIX,
    #[doc = "PMC"]
    pub PMC: PMC,
    #[doc = "UART"]
    pub UART: UART,
    #[doc = "CHIPID"]
    pub CHIPID: CHIPID,
    #[doc = "EFC0"]
    pub EFC0: EFC0,
    #[doc = "EFC1"]
    pub EFC1: EFC1,
    #[doc = "PIOA"]
    pub PIOA: PIOA,
    #[doc = "PIOB"]
    pub PIOB: PIOB,
    #[doc = "PIOC"]
    pub PIOC: PIOC,
    #[doc = "PIOD"]
    pub PIOD: PIOD,
    #[doc = "RSTC"]
    pub RSTC: RSTC,
    #[doc = "SUPC"]
    pub SUPC: SUPC,
    #[doc = "RTT"]
    pub RTT: RTT,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "GPBR"]
    pub GPBR: GPBR,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            HSMCI: HSMCI {
                _marker: PhantomData,
            },
            SSC: SSC {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            TC0: TC0 {
                _marker: PhantomData,
            },
            TC1: TC1 {
                _marker: PhantomData,
            },
            TC2: TC2 {
                _marker: PhantomData,
            },
            TWI0: TWI0 {
                _marker: PhantomData,
            },
            TWI1: TWI1 {
                _marker: PhantomData,
            },
            PWM: PWM {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            UOTGHS: UOTGHS {
                _marker: PhantomData,
            },
            EMAC: EMAC {
                _marker: PhantomData,
            },
            CAN0: CAN0 {
                _marker: PhantomData,
            },
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            TRNG: TRNG {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            DMAC: DMAC {
                _marker: PhantomData,
            },
            DACC: DACC {
                _marker: PhantomData,
            },
            SMC: SMC {
                _marker: PhantomData,
            },
            MATRIX: MATRIX {
                _marker: PhantomData,
            },
            PMC: PMC {
                _marker: PhantomData,
            },
            UART: UART {
                _marker: PhantomData,
            },
            CHIPID: CHIPID {
                _marker: PhantomData,
            },
            EFC0: EFC0 {
                _marker: PhantomData,
            },
            EFC1: EFC1 {
                _marker: PhantomData,
            },
            PIOA: PIOA {
                _marker: PhantomData,
            },
            PIOB: PIOB {
                _marker: PhantomData,
            },
            PIOC: PIOC {
                _marker: PhantomData,
            },
            PIOD: PIOD {
                _marker: PhantomData,
            },
            RSTC: RSTC {
                _marker: PhantomData,
            },
            SUPC: SUPC {
                _marker: PhantomData,
            },
            RTT: RTT {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            GPBR: GPBR {
                _marker: PhantomData,
            },
        }
    }
}
