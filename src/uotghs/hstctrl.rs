#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HSTCTRL {
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
pub struct SOFER {
    bits: bool,
}
impl SOFER {
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
pub struct RESETR {
    bits: bool,
}
impl RESETR {
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
pub struct RESUMER {
    bits: bool,
}
impl RESUMER {
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
#[doc = "Possible values of the field `SPDCONF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPDCONFR {
    #[doc = "The host starts in full-speed mode and performs a high-speed reset to switch to the high-speed mode if the downstream peripheral is high-speed capable."]
    NORMAL,
    #[doc = "For a better consumption, if high-speed is not needed."]
    LOW_POWER,
    #[doc = "Forced high speed."]
    HIGH_SPEED,
    #[doc = "The host remains to full-speed mode whatever the peripheral speed capability."]
    FORCED_FS,
}
impl SPDCONFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPDCONFR::NORMAL => 0,
            SPDCONFR::LOW_POWER => 1,
            SPDCONFR::HIGH_SPEED => 2,
            SPDCONFR::FORCED_FS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPDCONFR {
        match value {
            0 => SPDCONFR::NORMAL,
            1 => SPDCONFR::LOW_POWER,
            2 => SPDCONFR::HIGH_SPEED,
            3 => SPDCONFR::FORCED_FS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == SPDCONFR::NORMAL
    }
    #[doc = "Checks if the value of the field is `LOW_POWER`"]
    #[inline]
    pub fn is_low_power(&self) -> bool {
        *self == SPDCONFR::LOW_POWER
    }
    #[doc = "Checks if the value of the field is `HIGH_SPEED`"]
    #[inline]
    pub fn is_high_speed(&self) -> bool {
        *self == SPDCONFR::HIGH_SPEED
    }
    #[doc = "Checks if the value of the field is `FORCED_FS`"]
    #[inline]
    pub fn is_forced_fs(&self) -> bool {
        *self == SPDCONFR::FORCED_FS
    }
}
#[doc = r" Proxy"]
pub struct _SOFEW<'a> {
    w: &'a mut W,
}
impl<'a> _SOFEW<'a> {
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
pub struct _RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _RESETW<'a> {
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
pub struct _RESUMEW<'a> {
    w: &'a mut W,
}
impl<'a> _RESUMEW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPDCONF`"]
pub enum SPDCONFW {
    #[doc = "The host starts in full-speed mode and performs a high-speed reset to switch to the high-speed mode if the downstream peripheral is high-speed capable."]
    NORMAL,
    #[doc = "For a better consumption, if high-speed is not needed."]
    LOW_POWER,
    #[doc = "Forced high speed."]
    HIGH_SPEED,
    #[doc = "The host remains to full-speed mode whatever the peripheral speed capability."]
    FORCED_FS,
}
impl SPDCONFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SPDCONFW::NORMAL => 0,
            SPDCONFW::LOW_POWER => 1,
            SPDCONFW::HIGH_SPEED => 2,
            SPDCONFW::FORCED_FS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPDCONFW<'a> {
    w: &'a mut W,
}
impl<'a> _SPDCONFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPDCONFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The host starts in full-speed mode and performs a high-speed reset to switch to the high-speed mode if the downstream peripheral is high-speed capable."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(SPDCONFW::NORMAL)
    }
    #[doc = "For a better consumption, if high-speed is not needed."]
    #[inline]
    pub fn low_power(self) -> &'a mut W {
        self.variant(SPDCONFW::LOW_POWER)
    }
    #[doc = "Forced high speed."]
    #[inline]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(SPDCONFW::HIGH_SPEED)
    }
    #[doc = "The host remains to full-speed mode whatever the peripheral speed capability."]
    #[inline]
    pub fn forced_fs(self) -> &'a mut W {
        self.variant(SPDCONFW::FORCED_FS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
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
    #[doc = "Bit 8 - Start of Frame Generation Enable"]
    #[inline]
    pub fn sofe(&self) -> SOFER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SOFER { bits }
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline]
    pub fn reset(&self) -> RESETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESETR { bits }
    }
    #[doc = "Bit 10 - Send USB Resume"]
    #[inline]
    pub fn resume(&self) -> RESUMER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RESUMER { bits }
    }
    #[doc = "Bits 12:13 - Mode Configuration"]
    #[inline]
    pub fn spdconf(&self) -> SPDCONFR {
        SPDCONFR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 8 - Start of Frame Generation Enable"]
    #[inline]
    pub fn sofe(&mut self) -> _SOFEW {
        _SOFEW { w: self }
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline]
    pub fn reset(&mut self) -> _RESETW {
        _RESETW { w: self }
    }
    #[doc = "Bit 10 - Send USB Resume"]
    #[inline]
    pub fn resume(&mut self) -> _RESUMEW {
        _RESUMEW { w: self }
    }
    #[doc = "Bits 12:13 - Mode Configuration"]
    #[inline]
    pub fn spdconf(&mut self) -> _SPDCONFW {
        _SPDCONFW { w: self }
    }
}
