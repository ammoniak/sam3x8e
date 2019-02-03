#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EMR {
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
#[doc = "Possible values of the field `CMPMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPMODER {
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    LOW,
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    HIGH,
    #[doc = "Generates an event when the converted data is in the comparison window."]
    IN,
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    OUT,
}
impl CMPMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMPMODER::LOW => 0,
            CMPMODER::HIGH => 1,
            CMPMODER::IN => 2,
            CMPMODER::OUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMPMODER {
        match value {
            0 => CMPMODER::LOW,
            1 => CMPMODER::HIGH,
            2 => CMPMODER::IN,
            3 => CMPMODER::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CMPMODER::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CMPMODER::HIGH
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline]
    pub fn is_in_(&self) -> bool {
        *self == CMPMODER::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline]
    pub fn is_out(&self) -> bool {
        *self == CMPMODER::OUT
    }
}
#[doc = r" Value of the field"]
pub struct CMPSELR {
    bits: u8,
}
impl CMPSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMPALLR {
    bits: bool,
}
impl CMPALLR {
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
pub struct CMPFILTERR {
    bits: u8,
}
impl CMPFILTERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TAGR {
    bits: bool,
}
impl TAGR {
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
#[doc = "Values that can be written to the field `CMPMODE`"]
pub enum CMPMODEW {
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    LOW,
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    HIGH,
    #[doc = "Generates an event when the converted data is in the comparison window."]
    IN,
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    OUT,
}
impl CMPMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMPMODEW::LOW => 0,
            CMPMODEW::HIGH => 1,
            CMPMODEW::IN => 2,
            CMPMODEW::OUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMPMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMPMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CMPMODEW::LOW)
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CMPMODEW::HIGH)
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline]
    pub fn in_(self) -> &'a mut W {
        self.variant(CMPMODEW::IN)
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline]
    pub fn out(self) -> &'a mut W {
        self.variant(CMPMODEW::OUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMPALLW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPALLW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMPFILTERW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPFILTERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TAGW<'a> {
    w: &'a mut W,
}
impl<'a> _TAGW<'a> {
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
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline]
    pub fn cmpmode(&self) -> CMPMODER {
        CMPMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Comparison Selected Channel"]
    #[inline]
    pub fn cmpsel(&self) -> CMPSELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMPSELR { bits }
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline]
    pub fn cmpall(&self) -> CMPALLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CMPALLR { bits }
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline]
    pub fn cmpfilter(&self) -> CMPFILTERR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMPFILTERR { bits }
    }
    #[doc = "Bit 24 - TAG of the ADC_LDCR register"]
    #[inline]
    pub fn tag(&self) -> TAGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAGR { bits }
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
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline]
    pub fn cmpmode(&mut self) -> _CMPMODEW {
        _CMPMODEW { w: self }
    }
    #[doc = "Bits 4:7 - Comparison Selected Channel"]
    #[inline]
    pub fn cmpsel(&mut self) -> _CMPSELW {
        _CMPSELW { w: self }
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline]
    pub fn cmpall(&mut self) -> _CMPALLW {
        _CMPALLW { w: self }
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline]
    pub fn cmpfilter(&mut self) -> _CMPFILTERW {
        _CMPFILTERW { w: self }
    }
    #[doc = "Bit 24 - TAG of the ADC_LDCR register"]
    #[inline]
    pub fn tag(&mut self) -> _TAGW {
        _TAGW { w: self }
    }
}
