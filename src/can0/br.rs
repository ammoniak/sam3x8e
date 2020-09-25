#[doc = "Reader of register BR"]
pub type R = crate::R<u32, super::BR>;
#[doc = "Writer for register BR"]
pub type W = crate::W<u32, super::BR>;
#[doc = "Register BR `reset()`'s with value 0"]
impl crate::ResetValue for super::BR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PHASE2`"]
pub type PHASE2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PHASE2`"]
pub struct PHASE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `PHASE1`"]
pub type PHASE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PHASE1`"]
pub struct PHASE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `PROPAG`"]
pub type PROPAG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PROPAG`"]
pub struct PROPAG_W<'a> {
    w: &'a mut W,
}
impl<'a> PROPAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `SJW`"]
pub type SJW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SJW`"]
pub struct SJW_W<'a> {
    w: &'a mut W,
}
impl<'a> SJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `BRP`"]
pub type BRP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BRP`"]
pub struct BRP_W<'a> {
    w: &'a mut W,
}
impl<'a> BRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Sampling Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMP_A {
    #[doc = "0: The incoming bit stream is sampled once at sample point."]
    ONCE = 0,
    #[doc = "1: The incoming bit stream is sampled three times with a period of a MCK clock period, centered on sample point."]
    THREE = 1,
}
impl From<SMP_A> for bool {
    #[inline(always)]
    fn from(variant: SMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SMP`"]
pub type SMP_R = crate::R<bool, SMP_A>;
impl SMP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP_A {
        match self.bits {
            false => SMP_A::ONCE,
            true => SMP_A::THREE,
        }
    }
    #[doc = "Checks if the value of the field is `ONCE`"]
    #[inline(always)]
    pub fn is_once(&self) -> bool {
        *self == SMP_A::ONCE
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == SMP_A::THREE
    }
}
#[doc = "Write proxy for field `SMP`"]
pub struct SMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The incoming bit stream is sampled once at sample point."]
    #[inline(always)]
    pub fn once(self) -> &'a mut W {
        self.variant(SMP_A::ONCE)
    }
    #[doc = "The incoming bit stream is sampled three times with a period of a MCK clock period, centered on sample point."]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(SMP_A::THREE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Phase 2 segment"]
    #[inline(always)]
    pub fn phase2(&self) -> PHASE2_R {
        PHASE2_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Phase 1 segment"]
    #[inline(always)]
    pub fn phase1(&self) -> PHASE1_R {
        PHASE1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Programming time segment"]
    #[inline(always)]
    pub fn propag(&self) -> PROPAG_R {
        PROPAG_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:13 - Re-synchronization jump width"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:22 - Baudrate Prescaler."]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Sampling Mode"]
    #[inline(always)]
    pub fn smp(&self) -> SMP_R {
        SMP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Phase 2 segment"]
    #[inline(always)]
    pub fn phase2(&mut self) -> PHASE2_W {
        PHASE2_W { w: self }
    }
    #[doc = "Bits 4:6 - Phase 1 segment"]
    #[inline(always)]
    pub fn phase1(&mut self) -> PHASE1_W {
        PHASE1_W { w: self }
    }
    #[doc = "Bits 8:10 - Programming time segment"]
    #[inline(always)]
    pub fn propag(&mut self) -> PROPAG_W {
        PROPAG_W { w: self }
    }
    #[doc = "Bits 12:13 - Re-synchronization jump width"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W {
        SJW_W { w: self }
    }
    #[doc = "Bits 16:22 - Baudrate Prescaler."]
    #[inline(always)]
    pub fn brp(&mut self) -> BRP_W {
        BRP_W { w: self }
    }
    #[doc = "Bit 24 - Sampling Mode"]
    #[inline(always)]
    pub fn smp(&mut self) -> SMP_W {
        SMP_W { w: self }
    }
}
