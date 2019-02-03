#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MR_SPI_MODE {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let bits = self.register.get();
        let mut w = W { bits: bits };
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `USART_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART_MODER {
    #[doc = "SPI master"]
    SPI_MASTER,
    #[doc = "SPI Slave"]
    SPI_SLAVE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl USART_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            USART_MODER::SPI_MASTER => 14,
            USART_MODER::SPI_SLAVE => 15,
            USART_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> USART_MODER {
        match value {
            14 => USART_MODER::SPI_MASTER,
            15 => USART_MODER::SPI_SLAVE,
            i => USART_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline]
    pub fn is_spi_master(&self) -> bool {
        *self == USART_MODER::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `SPI_SLAVE`"]
    #[inline]
    pub fn is_spi_slave(&self) -> bool {
        *self == USART_MODER::SPI_SLAVE
    }
}
#[doc = "Possible values of the field `USCLKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USCLKSR {
    #[doc = "master Clock MCK is selected"]
    MCK,
    #[doc = "Internal Clock Divided MCK/DIV (DIV=8) is selected"]
    DIV,
    #[doc = "Serial Clock SLK is selected"]
    SCK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl USCLKSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            USCLKSR::MCK => 0,
            USCLKSR::DIV => 1,
            USCLKSR::SCK => 3,
            USCLKSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> USCLKSR {
        match value {
            0 => USCLKSR::MCK,
            1 => USCLKSR::DIV,
            3 => USCLKSR::SCK,
            i => USCLKSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline]
    pub fn is_mck(&self) -> bool {
        *self == USCLKSR::MCK
    }
    #[doc = "Checks if the value of the field is `DIV`"]
    #[inline]
    pub fn is_div(&self) -> bool {
        *self == USCLKSR::DIV
    }
    #[doc = "Checks if the value of the field is `SCK`"]
    #[inline]
    pub fn is_sck(&self) -> bool {
        *self == USCLKSR::SCK
    }
}
#[doc = "Possible values of the field `CHRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHRLR {
    #[doc = "Character length is 8 bits"]
    _8_BIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CHRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CHRLR::_8_BIT => 3,
            CHRLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CHRLR {
        match value {
            3 => CHRLR::_8_BIT,
            i => CHRLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline]
    pub fn is_8_bit(&self) -> bool {
        *self == CHRLR::_8_BIT
    }
}
#[doc = r" Value of the field"]
pub struct CPHAR {
    bits: bool,
}
impl CPHAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct CPOLR {
    bits: bool,
}
impl CPOLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct WRDBTR {
    bits: bool,
}
impl WRDBTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Values that can be written to the field `USART_MODE`"]
pub enum USART_MODEW {
    #[doc = "SPI master"]
    SPI_MASTER,
    #[doc = "SPI Slave"]
    SPI_SLAVE,
}
impl USART_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            USART_MODEW::SPI_MASTER => 14,
            USART_MODEW::SPI_SLAVE => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USART_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _USART_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SPI master"]
    #[inline]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(USART_MODEW::SPI_MASTER)
    }
    #[doc = "SPI Slave"]
    #[inline]
    pub fn spi_slave(self) -> &'a mut W {
        self.variant(USART_MODEW::SPI_SLAVE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `USCLKS`"]
pub enum USCLKSW {
    #[doc = "master Clock MCK is selected"]
    MCK,
    #[doc = "Internal Clock Divided MCK/DIV (DIV=8) is selected"]
    DIV,
    #[doc = "Serial Clock SLK is selected"]
    SCK,
}
impl USCLKSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            USCLKSW::MCK => 0,
            USCLKSW::DIV => 1,
            USCLKSW::SCK => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USCLKSW<'a> {
    w: &'a mut W,
}
impl<'a> _USCLKSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USCLKSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "master Clock MCK is selected"]
    #[inline]
    pub fn mck(self) -> &'a mut W {
        self.variant(USCLKSW::MCK)
    }
    #[doc = "Internal Clock Divided MCK/DIV (DIV=8) is selected"]
    #[inline]
    pub fn div(self) -> &'a mut W {
        self.variant(USCLKSW::DIV)
    }
    #[doc = "Serial Clock SLK is selected"]
    #[inline]
    pub fn sck(self) -> &'a mut W {
        self.variant(USCLKSW::SCK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CHRL`"]
pub enum CHRLW {
    #[doc = "Character length is 8 bits"]
    _8_BIT,
}
impl CHRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHRLW::_8_BIT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHRLW<'a> {
    w: &'a mut W,
}
impl<'a> _CHRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHRLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Character length is 8 bits"]
    #[inline]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(CHRLW::_8_BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _CPHAW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOLW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WRDBTW<'a> {
    w: &'a mut W,
}
impl<'a> _WRDBTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline]
    pub fn usart_mode(&self) -> USART_MODER {
        USART_MODER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline]
    pub fn usclks(&self) -> USCLKSR {
        USCLKSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Character Length"]
    #[inline]
    pub fn chrl(&self) -> CHRLR {
        CHRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - SPI Clock Phase"]
    #[inline]
    pub fn cpha(&self) -> CPHAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CPHAR { bits }
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline]
    pub fn cpol(&self) -> CPOLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CPOLR { bits }
    }
    #[doc = "Bit 20 - Wait Read Data Before Transfer"]
    #[inline]
    pub fn wrdbt(&self) -> WRDBTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRDBTR { bits }
    }
}
impl W {
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline]
    pub fn usart_mode(&mut self) -> _USART_MODEW {
        _USART_MODEW { w: self }
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline]
    pub fn usclks(&mut self) -> _USCLKSW {
        _USCLKSW { w: self }
    }
    #[doc = "Bits 6:7 - Character Length"]
    #[inline]
    pub fn chrl(&mut self) -> _CHRLW {
        _CHRLW { w: self }
    }
    #[doc = "Bit 8 - SPI Clock Phase"]
    #[inline]
    pub fn cpha(&mut self) -> _CPHAW {
        _CPHAW { w: self }
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
    #[doc = "Bit 20 - Wait Read Data Before Transfer"]
    #[inline]
    pub fn wrdbt(&mut self) -> _WRDBTW {
        _WRDBTW { w: self }
    }
}
