#[doc = "Reader of register LAST"]
pub type R = crate::R<u32, super::LAST>;
#[doc = "Writer for register LAST"]
pub type W = crate::W<u32, super::LAST>;
#[doc = "Register LAST `reset()`'s with value 0"]
impl crate::ResetValue for super::LAST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLAST0`"]
pub type SLAST0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAST0`"]
pub struct SLAST0_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAST0_W<'a> {
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
#[doc = "Reader of field `DLAST0`"]
pub type DLAST0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLAST0`"]
pub struct DLAST0_W<'a> {
    w: &'a mut W,
}
impl<'a> DLAST0_W<'a> {
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
#[doc = "Reader of field `SLAST1`"]
pub type SLAST1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAST1`"]
pub struct SLAST1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAST1_W<'a> {
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
#[doc = "Reader of field `DLAST1`"]
pub type DLAST1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLAST1`"]
pub struct DLAST1_W<'a> {
    w: &'a mut W,
}
impl<'a> DLAST1_W<'a> {
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
#[doc = "Reader of field `SLAST2`"]
pub type SLAST2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAST2`"]
pub struct SLAST2_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAST2_W<'a> {
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
#[doc = "Reader of field `DLAST2`"]
pub type DLAST2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLAST2`"]
pub struct DLAST2_W<'a> {
    w: &'a mut W,
}
impl<'a> DLAST2_W<'a> {
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
#[doc = "Reader of field `SLAST3`"]
pub type SLAST3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAST3`"]
pub struct SLAST3_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAST3_W<'a> {
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
#[doc = "Reader of field `DLAST3`"]
pub type DLAST3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLAST3`"]
pub struct DLAST3_W<'a> {
    w: &'a mut W,
}
impl<'a> DLAST3_W<'a> {
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
#[doc = "Reader of field `SLAST4`"]
pub type SLAST4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAST4`"]
pub struct SLAST4_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAST4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `DLAST4`"]
pub type DLAST4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLAST4`"]
pub struct DLAST4_W<'a> {
    w: &'a mut W,
}
impl<'a> DLAST4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SLAST5`"]
pub type SLAST5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAST5`"]
pub struct SLAST5_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAST5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `DLAST5`"]
pub type DLAST5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLAST5`"]
pub struct DLAST5_W<'a> {
    w: &'a mut W,
}
impl<'a> DLAST5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Source Last"]
    #[inline(always)]
    pub fn slast0(&self) -> SLAST0_R {
        SLAST0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Destination Last"]
    #[inline(always)]
    pub fn dlast0(&self) -> DLAST0_R {
        DLAST0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Source Last"]
    #[inline(always)]
    pub fn slast1(&self) -> SLAST1_R {
        SLAST1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Destination Last"]
    #[inline(always)]
    pub fn dlast1(&self) -> DLAST1_R {
        DLAST1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Source Last"]
    #[inline(always)]
    pub fn slast2(&self) -> SLAST2_R {
        SLAST2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Destination Last"]
    #[inline(always)]
    pub fn dlast2(&self) -> DLAST2_R {
        DLAST2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Source Last"]
    #[inline(always)]
    pub fn slast3(&self) -> SLAST3_R {
        SLAST3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Destination Last"]
    #[inline(always)]
    pub fn dlast3(&self) -> DLAST3_R {
        DLAST3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Source Last"]
    #[inline(always)]
    pub fn slast4(&self) -> SLAST4_R {
        SLAST4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Destination Last"]
    #[inline(always)]
    pub fn dlast4(&self) -> DLAST4_R {
        DLAST4_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Source Last"]
    #[inline(always)]
    pub fn slast5(&self) -> SLAST5_R {
        SLAST5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Destination Last"]
    #[inline(always)]
    pub fn dlast5(&self) -> DLAST5_R {
        DLAST5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source Last"]
    #[inline(always)]
    pub fn slast0(&mut self) -> SLAST0_W {
        SLAST0_W { w: self }
    }
    #[doc = "Bit 1 - Destination Last"]
    #[inline(always)]
    pub fn dlast0(&mut self) -> DLAST0_W {
        DLAST0_W { w: self }
    }
    #[doc = "Bit 2 - Source Last"]
    #[inline(always)]
    pub fn slast1(&mut self) -> SLAST1_W {
        SLAST1_W { w: self }
    }
    #[doc = "Bit 3 - Destination Last"]
    #[inline(always)]
    pub fn dlast1(&mut self) -> DLAST1_W {
        DLAST1_W { w: self }
    }
    #[doc = "Bit 4 - Source Last"]
    #[inline(always)]
    pub fn slast2(&mut self) -> SLAST2_W {
        SLAST2_W { w: self }
    }
    #[doc = "Bit 5 - Destination Last"]
    #[inline(always)]
    pub fn dlast2(&mut self) -> DLAST2_W {
        DLAST2_W { w: self }
    }
    #[doc = "Bit 6 - Source Last"]
    #[inline(always)]
    pub fn slast3(&mut self) -> SLAST3_W {
        SLAST3_W { w: self }
    }
    #[doc = "Bit 7 - Destination Last"]
    #[inline(always)]
    pub fn dlast3(&mut self) -> DLAST3_W {
        DLAST3_W { w: self }
    }
    #[doc = "Bit 8 - Source Last"]
    #[inline(always)]
    pub fn slast4(&mut self) -> SLAST4_W {
        SLAST4_W { w: self }
    }
    #[doc = "Bit 9 - Destination Last"]
    #[inline(always)]
    pub fn dlast4(&mut self) -> DLAST4_W {
        DLAST4_W { w: self }
    }
    #[doc = "Bit 10 - Source Last"]
    #[inline(always)]
    pub fn slast5(&mut self) -> SLAST5_W {
        SLAST5_W { w: self }
    }
    #[doc = "Bit 11 - Destination Last"]
    #[inline(always)]
    pub fn dlast5(&mut self) -> DLAST5_W {
        DLAST5_W { w: self }
    }
}
