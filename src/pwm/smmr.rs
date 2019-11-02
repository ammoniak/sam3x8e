#[doc = "Reader of register SMMR"]
pub type R = crate::R<u32, super::SMMR>;
#[doc = "Writer for register SMMR"]
pub type W = crate::W<u32, super::SMMR>;
#[doc = "Register SMMR `reset()`'s with value 0"]
impl crate::ResetValue for super::SMMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GCEN0`"]
pub type GCEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCEN0`"]
pub struct GCEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> GCEN0_W<'a> {
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
#[doc = "Reader of field `GCEN1`"]
pub type GCEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCEN1`"]
pub struct GCEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> GCEN1_W<'a> {
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
#[doc = "Reader of field `GCEN2`"]
pub type GCEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCEN2`"]
pub struct GCEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> GCEN2_W<'a> {
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
#[doc = "Reader of field `GCEN3`"]
pub type GCEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCEN3`"]
pub struct GCEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> GCEN3_W<'a> {
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
#[doc = "Reader of field `DOWN0`"]
pub type DOWN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOWN0`"]
pub struct DOWN0_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWN0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `DOWN1`"]
pub type DOWN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOWN1`"]
pub struct DOWN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWN1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `DOWN2`"]
pub type DOWN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOWN2`"]
pub struct DOWN2_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `DOWN3`"]
pub type DOWN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOWN3`"]
pub struct DOWN3_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWN3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Gray Count ENable"]
    #[inline(always)]
    pub fn gcen0(&self) -> GCEN0_R {
        GCEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Gray Count ENable"]
    #[inline(always)]
    pub fn gcen1(&self) -> GCEN1_R {
        GCEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Gray Count ENable"]
    #[inline(always)]
    pub fn gcen2(&self) -> GCEN2_R {
        GCEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Gray Count ENable"]
    #[inline(always)]
    pub fn gcen3(&self) -> GCEN3_R {
        GCEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DOWN Count"]
    #[inline(always)]
    pub fn down0(&self) -> DOWN0_R {
        DOWN0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DOWN Count"]
    #[inline(always)]
    pub fn down1(&self) -> DOWN1_R {
        DOWN1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - DOWN Count"]
    #[inline(always)]
    pub fn down2(&self) -> DOWN2_R {
        DOWN2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - DOWN Count"]
    #[inline(always)]
    pub fn down3(&self) -> DOWN3_R {
        DOWN3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Gray Count ENable"]
    #[inline(always)]
    pub fn gcen0(&mut self) -> GCEN0_W {
        GCEN0_W { w: self }
    }
    #[doc = "Bit 1 - Gray Count ENable"]
    #[inline(always)]
    pub fn gcen1(&mut self) -> GCEN1_W {
        GCEN1_W { w: self }
    }
    #[doc = "Bit 2 - Gray Count ENable"]
    #[inline(always)]
    pub fn gcen2(&mut self) -> GCEN2_W {
        GCEN2_W { w: self }
    }
    #[doc = "Bit 3 - Gray Count ENable"]
    #[inline(always)]
    pub fn gcen3(&mut self) -> GCEN3_W {
        GCEN3_W { w: self }
    }
    #[doc = "Bit 16 - DOWN Count"]
    #[inline(always)]
    pub fn down0(&mut self) -> DOWN0_W {
        DOWN0_W { w: self }
    }
    #[doc = "Bit 17 - DOWN Count"]
    #[inline(always)]
    pub fn down1(&mut self) -> DOWN1_W {
        DOWN1_W { w: self }
    }
    #[doc = "Bit 18 - DOWN Count"]
    #[inline(always)]
    pub fn down2(&mut self) -> DOWN2_W {
        DOWN2_W { w: self }
    }
    #[doc = "Bit 19 - DOWN Count"]
    #[inline(always)]
    pub fn down3(&mut self) -> DOWN3_W {
        DOWN3_W { w: self }
    }
}
