#[doc = "Reader of register MR"]
pub type R = crate::R<u32, super::MR>;
#[doc = "Writer for register MR"]
pub type W = crate::W<u32, super::MR>;
#[doc = "Register MR `reset()`'s with value 0"]
impl crate::ResetValue for super::MR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CANEN`"]
pub type CANEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CANEN`"]
pub struct CANEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CANEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `LPM`"]
pub type LPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPM`"]
pub struct LPM_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ABM`"]
pub type ABM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ABM`"]
pub struct ABM_W<'a> {
    w: &'a mut W,
}
impl<'a> ABM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `OVL`"]
pub type OVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVL`"]
pub struct OVL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TEOF`"]
pub type TEOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TEOF`"]
pub struct TEOF_W<'a> {
    w: &'a mut W,
}
impl<'a> TEOF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TTM`"]
pub type TTM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TTM`"]
pub struct TTM_W<'a> {
    w: &'a mut W,
}
impl<'a> TTM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TIMFRZ`"]
pub type TIMFRZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMFRZ`"]
pub struct TIMFRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMFRZ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DRPT`"]
pub type DRPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DRPT`"]
pub struct DRPT_W<'a> {
    w: &'a mut W,
}
impl<'a> DRPT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reception Synchronization Stage (not readable)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSYNC_A {
    #[doc = "0: Rx Signal with Double Synchro Stages (2 Positive Edges)"]
    DOUBLE_PP,
    #[doc = "1: Rx Signal with Double Synchro Stages (One Positive Edge and One Negative Edge)"]
    DOUBLE_PN,
    #[doc = "2: Rx Signal with Single Synchro Stage (Positive Edge)"]
    SINGLE_P,
    #[doc = "3: Rx Signal with No Synchro Stage"]
    NONE,
}
impl From<RXSYNC_A> for u8 {
    #[inline(always)]
    fn from(variant: RXSYNC_A) -> Self {
        match variant {
            RXSYNC_A::DOUBLE_PP => 0,
            RXSYNC_A::DOUBLE_PN => 1,
            RXSYNC_A::SINGLE_P => 2,
            RXSYNC_A::NONE => 3,
        }
    }
}
#[doc = "Reader of field `RXSYNC`"]
pub type RXSYNC_R = crate::R<u8, RXSYNC_A>;
impl RXSYNC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RXSYNC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXSYNC_A::DOUBLE_PP),
            1 => Val(RXSYNC_A::DOUBLE_PN),
            2 => Val(RXSYNC_A::SINGLE_P),
            3 => Val(RXSYNC_A::NONE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DOUBLE_PP`"]
    #[inline(always)]
    pub fn is_double_pp(&self) -> bool {
        *self == RXSYNC_A::DOUBLE_PP
    }
    #[doc = "Checks if the value of the field is `DOUBLE_PN`"]
    #[inline(always)]
    pub fn is_double_pn(&self) -> bool {
        *self == RXSYNC_A::DOUBLE_PN
    }
    #[doc = "Checks if the value of the field is `SINGLE_P`"]
    #[inline(always)]
    pub fn is_single_p(&self) -> bool {
        *self == RXSYNC_A::SINGLE_P
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == RXSYNC_A::NONE
    }
}
#[doc = "Write proxy for field `RXSYNC`"]
pub struct RXSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXSYNC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Rx Signal with Double Synchro Stages (2 Positive Edges)"]
    #[inline(always)]
    pub fn double_pp(self) -> &'a mut W {
        self.variant(RXSYNC_A::DOUBLE_PP)
    }
    #[doc = "Rx Signal with Double Synchro Stages (One Positive Edge and One Negative Edge)"]
    #[inline(always)]
    pub fn double_pn(self) -> &'a mut W {
        self.variant(RXSYNC_A::DOUBLE_PN)
    }
    #[doc = "Rx Signal with Single Synchro Stage (Positive Edge)"]
    #[inline(always)]
    pub fn single_p(self) -> &'a mut W {
        self.variant(RXSYNC_A::SINGLE_P)
    }
    #[doc = "Rx Signal with No Synchro Stage"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(RXSYNC_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CAN Controller Enable"]
    #[inline(always)]
    pub fn canen(&self) -> CANEN_R {
        CANEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Disable/Enable Low Power Mode"]
    #[inline(always)]
    pub fn lpm(&self) -> LPM_R {
        LPM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Disable/Enable Autobaud/Listen mode"]
    #[inline(always)]
    pub fn abm(&self) -> ABM_R {
        ABM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Disable/Enable Overload Frame"]
    #[inline(always)]
    pub fn ovl(&self) -> OVL_R {
        OVL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timestamp messages at each end of Frame"]
    #[inline(always)]
    pub fn teof(&self) -> TEOF_R {
        TEOF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Disable/Enable Time Triggered Mode"]
    #[inline(always)]
    pub fn ttm(&self) -> TTM_R {
        TTM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable Timer Freeze"]
    #[inline(always)]
    pub fn timfrz(&self) -> TIMFRZ_R {
        TIMFRZ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Disable Repeat"]
    #[inline(always)]
    pub fn drpt(&self) -> DRPT_R {
        DRPT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Reception Synchronization Stage (not readable)"]
    #[inline(always)]
    pub fn rxsync(&self) -> RXSYNC_R {
        RXSYNC_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CAN Controller Enable"]
    #[inline(always)]
    pub fn canen(&mut self) -> CANEN_W {
        CANEN_W { w: self }
    }
    #[doc = "Bit 1 - Disable/Enable Low Power Mode"]
    #[inline(always)]
    pub fn lpm(&mut self) -> LPM_W {
        LPM_W { w: self }
    }
    #[doc = "Bit 2 - Disable/Enable Autobaud/Listen mode"]
    #[inline(always)]
    pub fn abm(&mut self) -> ABM_W {
        ABM_W { w: self }
    }
    #[doc = "Bit 3 - Disable/Enable Overload Frame"]
    #[inline(always)]
    pub fn ovl(&mut self) -> OVL_W {
        OVL_W { w: self }
    }
    #[doc = "Bit 4 - Timestamp messages at each end of Frame"]
    #[inline(always)]
    pub fn teof(&mut self) -> TEOF_W {
        TEOF_W { w: self }
    }
    #[doc = "Bit 5 - Disable/Enable Time Triggered Mode"]
    #[inline(always)]
    pub fn ttm(&mut self) -> TTM_W {
        TTM_W { w: self }
    }
    #[doc = "Bit 6 - Enable Timer Freeze"]
    #[inline(always)]
    pub fn timfrz(&mut self) -> TIMFRZ_W {
        TIMFRZ_W { w: self }
    }
    #[doc = "Bit 7 - Disable Repeat"]
    #[inline(always)]
    pub fn drpt(&mut self) -> DRPT_W {
        DRPT_W { w: self }
    }
    #[doc = "Bits 24:26 - Reception Synchronization Stage (not readable)"]
    #[inline(always)]
    pub fn rxsync(&mut self) -> RXSYNC_W {
        RXSYNC_W { w: self }
    }
}
