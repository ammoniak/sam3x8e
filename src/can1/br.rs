#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BR {
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
pub struct PHASE2R {
    bits: u8,
}
impl PHASE2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PHASE1R {
    bits: u8,
}
impl PHASE1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PROPAGR {
    bits: u8,
}
impl PROPAGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SJWR {
    bits: u8,
}
impl SJWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BRPR {
    bits: u8,
}
impl BRPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `SMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPR {
    #[doc = "The incoming bit stream is sampled once at sample point."]
    ONCE,
    #[doc = "The incoming bit stream is sampled three times with a period of a MCK clock period, centered on sample point."]
    THREE,
}
impl SMPR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SMPR::ONCE => false,
            SMPR::THREE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMPR {
        match value {
            false => SMPR::ONCE,
            true => SMPR::THREE,
        }
    }
    #[doc = "Checks if the value of the field is `ONCE`"]
    #[inline]
    pub fn is_once(&self) -> bool {
        *self == SMPR::ONCE
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline]
    pub fn is_three(&self) -> bool {
        *self == SMPR::THREE
    }
}
#[doc = r" Proxy"]
pub struct _PHASE2W<'a> {
    w: &'a mut W,
}
impl<'a> _PHASE2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PHASE1W<'a> {
    w: &'a mut W,
}
impl<'a> _PHASE1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PROPAGW<'a> {
    w: &'a mut W,
}
impl<'a> _PROPAGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SJWW<'a> {
    w: &'a mut W,
}
impl<'a> _SJWW<'a> {
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
pub struct _BRPW<'a> {
    w: &'a mut W,
}
impl<'a> _BRPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMP`"]
pub enum SMPW {
    #[doc = "The incoming bit stream is sampled once at sample point."]
    ONCE,
    #[doc = "The incoming bit stream is sampled three times with a period of a MCK clock period, centered on sample point."]
    THREE,
}
impl SMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMPW::ONCE => false,
            SMPW::THREE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMPW<'a> {
    w: &'a mut W,
}
impl<'a> _SMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The incoming bit stream is sampled once at sample point."]
    #[inline]
    pub fn once(self) -> &'a mut W {
        self.variant(SMPW::ONCE)
    }
    #[doc = "The incoming bit stream is sampled three times with a period of a MCK clock period, centered on sample point."]
    #[inline]
    pub fn three(self) -> &'a mut W {
        self.variant(SMPW::THREE)
    }
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
    #[doc = "Bits 0:2 - Phase 2 segment"]
    #[inline]
    pub fn phase2(&self) -> PHASE2R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PHASE2R { bits }
    }
    #[doc = "Bits 4:6 - Phase 1 segment"]
    #[inline]
    pub fn phase1(&self) -> PHASE1R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PHASE1R { bits }
    }
    #[doc = "Bits 8:10 - Programming time segment"]
    #[inline]
    pub fn propag(&self) -> PROPAGR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PROPAGR { bits }
    }
    #[doc = "Bits 12:13 - Re-synchronization jump width"]
    #[inline]
    pub fn sjw(&self) -> SJWR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SJWR { bits }
    }
    #[doc = "Bits 16:22 - Baudrate Prescaler."]
    #[inline]
    pub fn brp(&self) -> BRPR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BRPR { bits }
    }
    #[doc = "Bit 24 - Sampling Mode"]
    #[inline]
    pub fn smp(&self) -> SMPR {
        SMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
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
    #[doc = "Bits 0:2 - Phase 2 segment"]
    #[inline]
    pub fn phase2(&mut self) -> _PHASE2W {
        _PHASE2W { w: self }
    }
    #[doc = "Bits 4:6 - Phase 1 segment"]
    #[inline]
    pub fn phase1(&mut self) -> _PHASE1W {
        _PHASE1W { w: self }
    }
    #[doc = "Bits 8:10 - Programming time segment"]
    #[inline]
    pub fn propag(&mut self) -> _PROPAGW {
        _PROPAGW { w: self }
    }
    #[doc = "Bits 12:13 - Re-synchronization jump width"]
    #[inline]
    pub fn sjw(&mut self) -> _SJWW {
        _SJWW { w: self }
    }
    #[doc = "Bits 16:22 - Baudrate Prescaler."]
    #[inline]
    pub fn brp(&mut self) -> _BRPW {
        _BRPW { w: self }
    }
    #[doc = "Bit 24 - Sampling Mode"]
    #[inline]
    pub fn smp(&mut self) -> _SMPW {
        _SMPW { w: self }
    }
}
