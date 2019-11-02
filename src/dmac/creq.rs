#[doc = "Reader of register CREQ"]
pub type R = crate::R<u32, super::CREQ>;
#[doc = "Writer for register CREQ"]
pub type W = crate::W<u32, super::CREQ>;
#[doc = "Register CREQ `reset()`'s with value 0"]
impl crate::ResetValue for super::CREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCREQ0`"]
pub type SCREQ0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCREQ0`"]
pub struct SCREQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> SCREQ0_W<'a> {
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
#[doc = "Reader of field `DCREQ0`"]
pub type DCREQ0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCREQ0`"]
pub struct DCREQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> DCREQ0_W<'a> {
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
#[doc = "Reader of field `SCREQ1`"]
pub type SCREQ1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCREQ1`"]
pub struct SCREQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> SCREQ1_W<'a> {
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
#[doc = "Reader of field `DCREQ1`"]
pub type DCREQ1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCREQ1`"]
pub struct DCREQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> DCREQ1_W<'a> {
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
#[doc = "Reader of field `SCREQ2`"]
pub type SCREQ2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCREQ2`"]
pub struct SCREQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> SCREQ2_W<'a> {
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
#[doc = "Reader of field `DCREQ2`"]
pub type DCREQ2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCREQ2`"]
pub struct DCREQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> DCREQ2_W<'a> {
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
#[doc = "Reader of field `SCREQ3`"]
pub type SCREQ3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCREQ3`"]
pub struct SCREQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> SCREQ3_W<'a> {
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
#[doc = "Reader of field `DCREQ3`"]
pub type DCREQ3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCREQ3`"]
pub struct DCREQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> DCREQ3_W<'a> {
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
#[doc = "Reader of field `SCREQ4`"]
pub type SCREQ4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCREQ4`"]
pub struct SCREQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> SCREQ4_W<'a> {
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
#[doc = "Reader of field `DCREQ4`"]
pub type DCREQ4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCREQ4`"]
pub struct DCREQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> DCREQ4_W<'a> {
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
#[doc = "Reader of field `SCREQ5`"]
pub type SCREQ5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCREQ5`"]
pub struct SCREQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> SCREQ5_W<'a> {
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
#[doc = "Reader of field `DCREQ5`"]
pub type DCREQ5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCREQ5`"]
pub struct DCREQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> DCREQ5_W<'a> {
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
    #[doc = "Bit 0 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq0(&self) -> SCREQ0_R {
        SCREQ0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq0(&self) -> DCREQ0_R {
        DCREQ0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq1(&self) -> SCREQ1_R {
        SCREQ1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq1(&self) -> DCREQ1_R {
        DCREQ1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq2(&self) -> SCREQ2_R {
        SCREQ2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq2(&self) -> DCREQ2_R {
        DCREQ2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq3(&self) -> SCREQ3_R {
        SCREQ3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq3(&self) -> DCREQ3_R {
        DCREQ3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq4(&self) -> SCREQ4_R {
        SCREQ4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq4(&self) -> DCREQ4_R {
        DCREQ4_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq5(&self) -> SCREQ5_R {
        SCREQ5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq5(&self) -> DCREQ5_R {
        DCREQ5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq0(&mut self) -> SCREQ0_W {
        SCREQ0_W { w: self }
    }
    #[doc = "Bit 1 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq0(&mut self) -> DCREQ0_W {
        DCREQ0_W { w: self }
    }
    #[doc = "Bit 2 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq1(&mut self) -> SCREQ1_W {
        SCREQ1_W { w: self }
    }
    #[doc = "Bit 3 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq1(&mut self) -> DCREQ1_W {
        DCREQ1_W { w: self }
    }
    #[doc = "Bit 4 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq2(&mut self) -> SCREQ2_W {
        SCREQ2_W { w: self }
    }
    #[doc = "Bit 5 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq2(&mut self) -> DCREQ2_W {
        DCREQ2_W { w: self }
    }
    #[doc = "Bit 6 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq3(&mut self) -> SCREQ3_W {
        SCREQ3_W { w: self }
    }
    #[doc = "Bit 7 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq3(&mut self) -> DCREQ3_W {
        DCREQ3_W { w: self }
    }
    #[doc = "Bit 8 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq4(&mut self) -> SCREQ4_W {
        SCREQ4_W { w: self }
    }
    #[doc = "Bit 9 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq4(&mut self) -> DCREQ4_W {
        DCREQ4_W { w: self }
    }
    #[doc = "Bit 10 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq5(&mut self) -> SCREQ5_W {
        SCREQ5_W { w: self }
    }
    #[doc = "Bit 11 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq5(&mut self) -> DCREQ5_W {
        DCREQ5_W { w: self }
    }
}
