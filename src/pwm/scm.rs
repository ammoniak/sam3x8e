#[doc = "Reader of register SCM"]
pub type R = crate::R<u32, super::SCM>;
#[doc = "Writer for register SCM"]
pub type W = crate::W<u32, super::SCM>;
#[doc = "Register SCM `reset()`'s with value 0"]
impl crate::ResetValue for super::SCM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYNC0`"]
pub type SYNC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNC0`"]
pub struct SYNC0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC0_W<'a> {
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
#[doc = "Reader of field `SYNC1`"]
pub type SYNC1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNC1`"]
pub struct SYNC1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC1_W<'a> {
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
#[doc = "Reader of field `SYNC2`"]
pub type SYNC2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNC2`"]
pub struct SYNC2_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC2_W<'a> {
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
#[doc = "Reader of field `SYNC3`"]
pub type SYNC3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNC3`"]
pub struct SYNC3_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC3_W<'a> {
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
#[doc = "Reader of field `SYNC4`"]
pub type SYNC4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNC4`"]
pub struct SYNC4_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC4_W<'a> {
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
#[doc = "Reader of field `SYNC5`"]
pub type SYNC5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNC5`"]
pub struct SYNC5_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC5_W<'a> {
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
#[doc = "Reader of field `SYNC6`"]
pub type SYNC6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNC6`"]
pub struct SYNC6_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC6_W<'a> {
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
#[doc = "Reader of field `SYNC7`"]
pub type SYNC7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNC7`"]
pub struct SYNC7_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC7_W<'a> {
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
#[doc = "Synchronous Channels Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDM_A {
    #[doc = "0: Manual write of double buffer registers and manual update of synchronous channels"]
    MODE0,
    #[doc = "1: Manual write of double buffer registers and automatic update of synchronous channels"]
    MODE1,
    #[doc = "2: Automatic write of duty-cycle update registers by the PDC and automatic update of synchronous channels"]
    MODE2,
}
impl From<UPDM_A> for u8 {
    #[inline(always)]
    fn from(variant: UPDM_A) -> Self {
        match variant {
            UPDM_A::MODE0 => 0,
            UPDM_A::MODE1 => 1,
            UPDM_A::MODE2 => 2,
        }
    }
}
#[doc = "Reader of field `UPDM`"]
pub type UPDM_R = crate::R<u8, UPDM_A>;
impl UPDM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UPDM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UPDM_A::MODE0),
            1 => Val(UPDM_A::MODE1),
            2 => Val(UPDM_A::MODE2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == UPDM_A::MODE0
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == UPDM_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == UPDM_A::MODE2
    }
}
#[doc = "Write proxy for field `UPDM`"]
pub struct UPDM_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPDM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Manual write of double buffer registers and manual update of synchronous channels"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(UPDM_A::MODE0)
    }
    #[doc = "Manual write of double buffer registers and automatic update of synchronous channels"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(UPDM_A::MODE1)
    }
    #[doc = "Automatic write of duty-cycle update registers by the PDC and automatic update of synchronous channels"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(UPDM_A::MODE2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `PTRM`"]
pub type PTRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTRM`"]
pub struct PTRM_W<'a> {
    w: &'a mut W,
}
impl<'a> PTRM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `PTRCS`"]
pub type PTRCS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PTRCS`"]
pub struct PTRCS_W<'a> {
    w: &'a mut W,
}
impl<'a> PTRCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Synchronous Channel 0"]
    #[inline(always)]
    pub fn sync0(&self) -> SYNC0_R {
        SYNC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Synchronous Channel 1"]
    #[inline(always)]
    pub fn sync1(&self) -> SYNC1_R {
        SYNC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Synchronous Channel 2"]
    #[inline(always)]
    pub fn sync2(&self) -> SYNC2_R {
        SYNC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Synchronous Channel 3"]
    #[inline(always)]
    pub fn sync3(&self) -> SYNC3_R {
        SYNC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Synchronous Channel 4"]
    #[inline(always)]
    pub fn sync4(&self) -> SYNC4_R {
        SYNC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Synchronous Channel 5"]
    #[inline(always)]
    pub fn sync5(&self) -> SYNC5_R {
        SYNC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Synchronous Channel 6"]
    #[inline(always)]
    pub fn sync6(&self) -> SYNC6_R {
        SYNC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Synchronous Channel 7"]
    #[inline(always)]
    pub fn sync7(&self) -> SYNC7_R {
        SYNC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Synchronous Channels Update Mode"]
    #[inline(always)]
    pub fn updm(&self) -> UPDM_R {
        UPDM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 20 - PDC Transfer Request Mode"]
    #[inline(always)]
    pub fn ptrm(&self) -> PTRM_R {
        PTRM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:23 - PDC Transfer Request Comparison Selection"]
    #[inline(always)]
    pub fn ptrcs(&self) -> PTRCS_R {
        PTRCS_R::new(((self.bits >> 21) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronous Channel 0"]
    #[inline(always)]
    pub fn sync0(&mut self) -> SYNC0_W {
        SYNC0_W { w: self }
    }
    #[doc = "Bit 1 - Synchronous Channel 1"]
    #[inline(always)]
    pub fn sync1(&mut self) -> SYNC1_W {
        SYNC1_W { w: self }
    }
    #[doc = "Bit 2 - Synchronous Channel 2"]
    #[inline(always)]
    pub fn sync2(&mut self) -> SYNC2_W {
        SYNC2_W { w: self }
    }
    #[doc = "Bit 3 - Synchronous Channel 3"]
    #[inline(always)]
    pub fn sync3(&mut self) -> SYNC3_W {
        SYNC3_W { w: self }
    }
    #[doc = "Bit 4 - Synchronous Channel 4"]
    #[inline(always)]
    pub fn sync4(&mut self) -> SYNC4_W {
        SYNC4_W { w: self }
    }
    #[doc = "Bit 5 - Synchronous Channel 5"]
    #[inline(always)]
    pub fn sync5(&mut self) -> SYNC5_W {
        SYNC5_W { w: self }
    }
    #[doc = "Bit 6 - Synchronous Channel 6"]
    #[inline(always)]
    pub fn sync6(&mut self) -> SYNC6_W {
        SYNC6_W { w: self }
    }
    #[doc = "Bit 7 - Synchronous Channel 7"]
    #[inline(always)]
    pub fn sync7(&mut self) -> SYNC7_W {
        SYNC7_W { w: self }
    }
    #[doc = "Bits 16:17 - Synchronous Channels Update Mode"]
    #[inline(always)]
    pub fn updm(&mut self) -> UPDM_W {
        UPDM_W { w: self }
    }
    #[doc = "Bit 20 - PDC Transfer Request Mode"]
    #[inline(always)]
    pub fn ptrm(&mut self) -> PTRM_W {
        PTRM_W { w: self }
    }
    #[doc = "Bits 21:23 - PDC Transfer Request Comparison Selection"]
    #[inline(always)]
    pub fn ptrcs(&mut self) -> PTRCS_W {
        PTRCS_W { w: self }
    }
}
