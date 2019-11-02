#[doc = "Reader of register MAM6"]
pub type R = crate::R<u32, super::MAM6>;
#[doc = "Writer for register MAM6"]
pub type W = crate::W<u32, super::MAM6>;
#[doc = "Register MAM6 `reset()`'s with value 0"]
impl crate::ResetValue for super::MAM6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MIDvB`"]
pub type MIDVB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MIDvB`"]
pub struct MIDVB_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDVB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
#[doc = "Reader of field `MIDvA`"]
pub type MIDVA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MIDvA`"]
pub struct MIDVA_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDVA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 18)) | (((value as u32) & 0x07ff) << 18);
        self.w
    }
}
#[doc = "Reader of field `MIDE`"]
pub type MIDE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIDE`"]
pub struct MIDE_W<'a> {
    w: &'a mut W,
}
impl<'a> MIDE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Complementary bits for identifier in extended frame mode"]
    #[inline(always)]
    pub fn midv_b(&self) -> MIDVB_R {
        MIDVB_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:28 - Identifier for standard frame mode"]
    #[inline(always)]
    pub fn midv_a(&self) -> MIDVA_R {
        MIDVA_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bit 29 - Identifier Version"]
    #[inline(always)]
    pub fn mide(&self) -> MIDE_R {
        MIDE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:17 - Complementary bits for identifier in extended frame mode"]
    #[inline(always)]
    pub fn midv_b(&mut self) -> MIDVB_W {
        MIDVB_W { w: self }
    }
    #[doc = "Bits 18:28 - Identifier for standard frame mode"]
    #[inline(always)]
    pub fn midv_a(&mut self) -> MIDVA_W {
        MIDVA_W { w: self }
    }
    #[doc = "Bit 29 - Identifier Version"]
    #[inline(always)]
    pub fn mide(&mut self) -> MIDE_W {
        MIDE_W { w: self }
    }
}
