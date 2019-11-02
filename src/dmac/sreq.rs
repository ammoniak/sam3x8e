#[doc = "Reader of register SREQ"]
pub type R = crate::R<u32, super::SREQ>;
#[doc = "Writer for register SREQ"]
pub type W = crate::W<u32, super::SREQ>;
#[doc = "Register SREQ `reset()`'s with value 0"]
impl crate::ResetValue for super::SREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSREQ0`"]
pub type SSREQ0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSREQ0`"]
pub struct SSREQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> SSREQ0_W<'a> {
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
#[doc = "Reader of field `DSREQ0`"]
pub type DSREQ0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSREQ0`"]
pub struct DSREQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> DSREQ0_W<'a> {
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
#[doc = "Reader of field `SSREQ1`"]
pub type SSREQ1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSREQ1`"]
pub struct SSREQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> SSREQ1_W<'a> {
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
#[doc = "Reader of field `DSREQ1`"]
pub type DSREQ1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSREQ1`"]
pub struct DSREQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> DSREQ1_W<'a> {
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
#[doc = "Reader of field `SSREQ2`"]
pub type SSREQ2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSREQ2`"]
pub struct SSREQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> SSREQ2_W<'a> {
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
#[doc = "Reader of field `DSREQ2`"]
pub type DSREQ2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSREQ2`"]
pub struct DSREQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> DSREQ2_W<'a> {
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
#[doc = "Reader of field `SSREQ3`"]
pub type SSREQ3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSREQ3`"]
pub struct SSREQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> SSREQ3_W<'a> {
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
#[doc = "Reader of field `DSREQ3`"]
pub type DSREQ3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSREQ3`"]
pub struct DSREQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> DSREQ3_W<'a> {
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
#[doc = "Reader of field `SSREQ4`"]
pub type SSREQ4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSREQ4`"]
pub struct SSREQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> SSREQ4_W<'a> {
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
#[doc = "Reader of field `DSREQ4`"]
pub type DSREQ4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSREQ4`"]
pub struct DSREQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> DSREQ4_W<'a> {
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
#[doc = "Reader of field `SSREQ5`"]
pub type SSREQ5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSREQ5`"]
pub struct SSREQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> SSREQ5_W<'a> {
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
#[doc = "Reader of field `DSREQ5`"]
pub type DSREQ5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSREQ5`"]
pub struct DSREQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> DSREQ5_W<'a> {
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
    #[doc = "Bit 0 - Source Request"]
    #[inline(always)]
    pub fn ssreq0(&self) -> SSREQ0_R {
        SSREQ0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Destination Request"]
    #[inline(always)]
    pub fn dsreq0(&self) -> DSREQ0_R {
        DSREQ0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Source Request"]
    #[inline(always)]
    pub fn ssreq1(&self) -> SSREQ1_R {
        SSREQ1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Destination Request"]
    #[inline(always)]
    pub fn dsreq1(&self) -> DSREQ1_R {
        DSREQ1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Source Request"]
    #[inline(always)]
    pub fn ssreq2(&self) -> SSREQ2_R {
        SSREQ2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Destination Request"]
    #[inline(always)]
    pub fn dsreq2(&self) -> DSREQ2_R {
        DSREQ2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Source Request"]
    #[inline(always)]
    pub fn ssreq3(&self) -> SSREQ3_R {
        SSREQ3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Destination Request"]
    #[inline(always)]
    pub fn dsreq3(&self) -> DSREQ3_R {
        DSREQ3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Source Request"]
    #[inline(always)]
    pub fn ssreq4(&self) -> SSREQ4_R {
        SSREQ4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Destination Request"]
    #[inline(always)]
    pub fn dsreq4(&self) -> DSREQ4_R {
        DSREQ4_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Source Request"]
    #[inline(always)]
    pub fn ssreq5(&self) -> SSREQ5_R {
        SSREQ5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Destination Request"]
    #[inline(always)]
    pub fn dsreq5(&self) -> DSREQ5_R {
        DSREQ5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source Request"]
    #[inline(always)]
    pub fn ssreq0(&mut self) -> SSREQ0_W {
        SSREQ0_W { w: self }
    }
    #[doc = "Bit 1 - Destination Request"]
    #[inline(always)]
    pub fn dsreq0(&mut self) -> DSREQ0_W {
        DSREQ0_W { w: self }
    }
    #[doc = "Bit 2 - Source Request"]
    #[inline(always)]
    pub fn ssreq1(&mut self) -> SSREQ1_W {
        SSREQ1_W { w: self }
    }
    #[doc = "Bit 3 - Destination Request"]
    #[inline(always)]
    pub fn dsreq1(&mut self) -> DSREQ1_W {
        DSREQ1_W { w: self }
    }
    #[doc = "Bit 4 - Source Request"]
    #[inline(always)]
    pub fn ssreq2(&mut self) -> SSREQ2_W {
        SSREQ2_W { w: self }
    }
    #[doc = "Bit 5 - Destination Request"]
    #[inline(always)]
    pub fn dsreq2(&mut self) -> DSREQ2_W {
        DSREQ2_W { w: self }
    }
    #[doc = "Bit 6 - Source Request"]
    #[inline(always)]
    pub fn ssreq3(&mut self) -> SSREQ3_W {
        SSREQ3_W { w: self }
    }
    #[doc = "Bit 7 - Destination Request"]
    #[inline(always)]
    pub fn dsreq3(&mut self) -> DSREQ3_W {
        DSREQ3_W { w: self }
    }
    #[doc = "Bit 8 - Source Request"]
    #[inline(always)]
    pub fn ssreq4(&mut self) -> SSREQ4_W {
        SSREQ4_W { w: self }
    }
    #[doc = "Bit 9 - Destination Request"]
    #[inline(always)]
    pub fn dsreq4(&mut self) -> DSREQ4_W {
        DSREQ4_W { w: self }
    }
    #[doc = "Bit 10 - Source Request"]
    #[inline(always)]
    pub fn ssreq5(&mut self) -> SSREQ5_W {
        SSREQ5_W { w: self }
    }
    #[doc = "Bit 11 - Destination Request"]
    #[inline(always)]
    pub fn dsreq5(&mut self) -> DSREQ5_W {
        DSREQ5_W { w: self }
    }
}
