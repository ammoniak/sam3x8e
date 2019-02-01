#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MR {
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
pub struct CANENR {
    bits: bool,
}
impl CANENR {
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
pub struct LPMR {
    bits: bool,
}
impl LPMR {
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
pub struct ABMR {
    bits: bool,
}
impl ABMR {
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
pub struct OVLR {
    bits: bool,
}
impl OVLR {
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
pub struct TEOFR {
    bits: bool,
}
impl TEOFR {
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
pub struct TTMR {
    bits: bool,
}
impl TTMR {
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
pub struct TIMFRZR {
    bits: bool,
}
impl TIMFRZR {
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
pub struct DRPTR {
    bits: bool,
}
impl DRPTR {
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
#[doc = "Possible values of the field `RXSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSYNCR {
    #[doc = "Rx Signal with Double Synchro Stages (2 Positive Edges)"]
    DOUBLE_PP,
    #[doc = "Rx Signal with Double Synchro Stages (One Positive Edge and One Negative Edge)"]
    DOUBLE_PN,
    #[doc = "Rx Signal with Single Synchro Stage (Positive Edge)"]
    SINGLE_P,
    #[doc = "Rx Signal with No Synchro Stage"]
    NONE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RXSYNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXSYNCR::DOUBLE_PP => 0,
            RXSYNCR::DOUBLE_PN => 1,
            RXSYNCR::SINGLE_P => 2,
            RXSYNCR::NONE => 3,
            RXSYNCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXSYNCR {
        match value {
            0 => RXSYNCR::DOUBLE_PP,
            1 => RXSYNCR::DOUBLE_PN,
            2 => RXSYNCR::SINGLE_P,
            3 => RXSYNCR::NONE,
            i => RXSYNCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DOUBLE_PP`"]
    #[inline]
    pub fn is_double_pp(&self) -> bool {
        *self == RXSYNCR::DOUBLE_PP
    }
    #[doc = "Checks if the value of the field is `DOUBLE_PN`"]
    #[inline]
    pub fn is_double_pn(&self) -> bool {
        *self == RXSYNCR::DOUBLE_PN
    }
    #[doc = "Checks if the value of the field is `SINGLE_P`"]
    #[inline]
    pub fn is_single_p(&self) -> bool {
        *self == RXSYNCR::SINGLE_P
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == RXSYNCR::NONE
    }
}
#[doc = r" Proxy"]
pub struct _CANENW<'a> {
    w: &'a mut W,
}
impl<'a> _CANENW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPMW<'a> {
    w: &'a mut W,
}
impl<'a> _LPMW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ABMW<'a> {
    w: &'a mut W,
}
impl<'a> _ABMW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OVLW<'a> {
    w: &'a mut W,
}
impl<'a> _OVLW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TEOFW<'a> {
    w: &'a mut W,
}
impl<'a> _TEOFW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TTMW<'a> {
    w: &'a mut W,
}
impl<'a> _TTMW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TIMFRZW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMFRZW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DRPTW<'a> {
    w: &'a mut W,
}
impl<'a> _DRPTW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXSYNC`"]
pub enum RXSYNCW {
    #[doc = "Rx Signal with Double Synchro Stages (2 Positive Edges)"]
    DOUBLE_PP,
    #[doc = "Rx Signal with Double Synchro Stages (One Positive Edge and One Negative Edge)"]
    DOUBLE_PN,
    #[doc = "Rx Signal with Single Synchro Stage (Positive Edge)"]
    SINGLE_P,
    #[doc = "Rx Signal with No Synchro Stage"]
    NONE,
}
impl RXSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXSYNCW::DOUBLE_PP => 0,
            RXSYNCW::DOUBLE_PN => 1,
            RXSYNCW::SINGLE_P => 2,
            RXSYNCW::NONE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _RXSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXSYNCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Rx Signal with Double Synchro Stages (2 Positive Edges)"]
    #[inline]
    pub fn double_pp(self) -> &'a mut W {
        self.variant(RXSYNCW::DOUBLE_PP)
    }
    #[doc = "Rx Signal with Double Synchro Stages (One Positive Edge and One Negative Edge)"]
    #[inline]
    pub fn double_pn(self) -> &'a mut W {
        self.variant(RXSYNCW::DOUBLE_PN)
    }
    #[doc = "Rx Signal with Single Synchro Stage (Positive Edge)"]
    #[inline]
    pub fn single_p(self) -> &'a mut W {
        self.variant(RXSYNCW::SINGLE_P)
    }
    #[doc = "Rx Signal with No Synchro Stage"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(RXSYNCW::NONE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
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
    #[doc = "Bit 0 - CAN Controller Enable"]
    #[inline]
    pub fn canen(&self) -> CANENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CANENR { bits }
    }
    #[doc = "Bit 1 - Disable/Enable Low Power Mode"]
    #[inline]
    pub fn lpm(&self) -> LPMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPMR { bits }
    }
    #[doc = "Bit 2 - Disable/Enable Autobaud/Listen mode"]
    #[inline]
    pub fn abm(&self) -> ABMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ABMR { bits }
    }
    #[doc = "Bit 3 - Disable/Enable Overload Frame"]
    #[inline]
    pub fn ovl(&self) -> OVLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OVLR { bits }
    }
    #[doc = "Bit 4 - Timestamp messages at each end of Frame"]
    #[inline]
    pub fn teof(&self) -> TEOFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TEOFR { bits }
    }
    #[doc = "Bit 5 - Disable/Enable Time Triggered Mode"]
    #[inline]
    pub fn ttm(&self) -> TTMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TTMR { bits }
    }
    #[doc = "Bit 6 - Enable Timer Freeze"]
    #[inline]
    pub fn timfrz(&self) -> TIMFRZR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TIMFRZR { bits }
    }
    #[doc = "Bit 7 - Disable Repeat"]
    #[inline]
    pub fn drpt(&self) -> DRPTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DRPTR { bits }
    }
    #[doc = "Bits 24:26 - Reception Synchronization Stage (not readable)"]
    #[inline]
    pub fn rxsync(&self) -> RXSYNCR {
        RXSYNCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - CAN Controller Enable"]
    #[inline]
    pub fn canen(&mut self) -> _CANENW {
        _CANENW { w: self }
    }
    #[doc = "Bit 1 - Disable/Enable Low Power Mode"]
    #[inline]
    pub fn lpm(&mut self) -> _LPMW {
        _LPMW { w: self }
    }
    #[doc = "Bit 2 - Disable/Enable Autobaud/Listen mode"]
    #[inline]
    pub fn abm(&mut self) -> _ABMW {
        _ABMW { w: self }
    }
    #[doc = "Bit 3 - Disable/Enable Overload Frame"]
    #[inline]
    pub fn ovl(&mut self) -> _OVLW {
        _OVLW { w: self }
    }
    #[doc = "Bit 4 - Timestamp messages at each end of Frame"]
    #[inline]
    pub fn teof(&mut self) -> _TEOFW {
        _TEOFW { w: self }
    }
    #[doc = "Bit 5 - Disable/Enable Time Triggered Mode"]
    #[inline]
    pub fn ttm(&mut self) -> _TTMW {
        _TTMW { w: self }
    }
    #[doc = "Bit 6 - Enable Timer Freeze"]
    #[inline]
    pub fn timfrz(&mut self) -> _TIMFRZW {
        _TIMFRZW { w: self }
    }
    #[doc = "Bit 7 - Disable Repeat"]
    #[inline]
    pub fn drpt(&mut self) -> _DRPTW {
        _DRPTW { w: self }
    }
    #[doc = "Bits 24:26 - Reception Synchronization Stage (not readable)"]
    #[inline]
    pub fn rxsync(&mut self) -> _RXSYNCW {
        _RXSYNCW { w: self }
    }
}
