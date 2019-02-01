#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMC_PCR {
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
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct PIDR {
    bits: u8,
}
impl PIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMDR {
    bits: bool,
}
impl CMDR {
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
#[doc = "Possible values of the field `DIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVR {
    #[doc = "Peripheral clock is MCK"]
    PERIPH_DIV_MCK,
    #[doc = "Peripheral clock is MCK/2"]
    PERIPH_DIV2_MCK,
    #[doc = "Peripheral clock is MCK/4"]
    PERIPH_DIV4_MCK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIVR::PERIPH_DIV_MCK => 0,
            DIVR::PERIPH_DIV2_MCK => 1,
            DIVR::PERIPH_DIV4_MCK => 2,
            DIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIVR {
        match value {
            0 => DIVR::PERIPH_DIV_MCK,
            1 => DIVR::PERIPH_DIV2_MCK,
            2 => DIVR::PERIPH_DIV4_MCK,
            i => DIVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_DIV_MCK`"]
    #[inline]
    pub fn is_periph_div_mck(&self) -> bool {
        *self == DIVR::PERIPH_DIV_MCK
    }
    #[doc = "Checks if the value of the field is `PERIPH_DIV2_MCK`"]
    #[inline]
    pub fn is_periph_div2_mck(&self) -> bool {
        *self == DIVR::PERIPH_DIV2_MCK
    }
    #[doc = "Checks if the value of the field is `PERIPH_DIV4_MCK`"]
    #[inline]
    pub fn is_periph_div4_mck(&self) -> bool {
        *self == DIVR::PERIPH_DIV4_MCK
    }
}
#[doc = r" Value of the field"]
pub struct ENR {
    bits: bool,
}
impl ENR {
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
#[doc = r" Proxy"]
pub struct _PIDW<'a> {
    w: &'a mut W,
}
impl<'a> _PIDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIV`"]
pub enum DIVW {
    #[doc = "Peripheral clock is MCK"]
    PERIPH_DIV_MCK,
    #[doc = "Peripheral clock is MCK/2"]
    PERIPH_DIV2_MCK,
    #[doc = "Peripheral clock is MCK/4"]
    PERIPH_DIV4_MCK,
}
impl DIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIVW::PERIPH_DIV_MCK => 0,
            DIVW::PERIPH_DIV2_MCK => 1,
            DIVW::PERIPH_DIV4_MCK => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Peripheral clock is MCK"]
    #[inline]
    pub fn periph_div_mck(self) -> &'a mut W {
        self.variant(DIVW::PERIPH_DIV_MCK)
    }
    #[doc = "Peripheral clock is MCK/2"]
    #[inline]
    pub fn periph_div2_mck(self) -> &'a mut W {
        self.variant(DIVW::PERIPH_DIV2_MCK)
    }
    #[doc = "Peripheral clock is MCK/4"]
    #[inline]
    pub fn periph_div4_mck(self) -> &'a mut W {
        self.variant(DIVW::PERIPH_DIV4_MCK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
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
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:5 - Peripheral ID"]
    #[inline]
    pub fn pid(&self) -> PIDR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PIDR { bits }
    }
    #[doc = "Bit 12 - Command"]
    #[inline]
    pub fn cmd(&self) -> CMDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CMDR { bits }
    }
    #[doc = "Bits 16:17 - Divisor Value"]
    #[inline]
    pub fn div(&self) -> DIVR {
        DIVR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Enable"]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Peripheral ID"]
    #[inline]
    pub fn pid(&mut self) -> _PIDW {
        _PIDW { w: self }
    }
    #[doc = "Bit 12 - Command"]
    #[inline]
    pub fn cmd(&mut self) -> _CMDW {
        _CMDW { w: self }
    }
    #[doc = "Bits 16:17 - Divisor Value"]
    #[inline]
    pub fn div(&mut self) -> _DIVW {
        _DIVW { w: self }
    }
    #[doc = "Bit 28 - Enable"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
}
