#[doc = "Reader of register OS"]
pub type R = crate::R<u32, super::OS>;
#[doc = "Writer for register OS"]
pub type W = crate::W<u32, super::OS>;
#[doc = "Register OS `reset()`'s with value 0"]
impl crate::ResetValue for super::OS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OSH0`"]
pub type OSH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSH0`"]
pub struct OSH0_W<'a> {
    w: &'a mut W,
}
impl<'a> OSH0_W<'a> {
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
#[doc = "Reader of field `OSH1`"]
pub type OSH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSH1`"]
pub struct OSH1_W<'a> {
    w: &'a mut W,
}
impl<'a> OSH1_W<'a> {
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
#[doc = "Reader of field `OSH2`"]
pub type OSH2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSH2`"]
pub struct OSH2_W<'a> {
    w: &'a mut W,
}
impl<'a> OSH2_W<'a> {
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
#[doc = "Reader of field `OSH3`"]
pub type OSH3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSH3`"]
pub struct OSH3_W<'a> {
    w: &'a mut W,
}
impl<'a> OSH3_W<'a> {
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
#[doc = "Reader of field `OSH4`"]
pub type OSH4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSH4`"]
pub struct OSH4_W<'a> {
    w: &'a mut W,
}
impl<'a> OSH4_W<'a> {
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
#[doc = "Reader of field `OSH5`"]
pub type OSH5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSH5`"]
pub struct OSH5_W<'a> {
    w: &'a mut W,
}
impl<'a> OSH5_W<'a> {
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
#[doc = "Reader of field `OSH6`"]
pub type OSH6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSH6`"]
pub struct OSH6_W<'a> {
    w: &'a mut W,
}
impl<'a> OSH6_W<'a> {
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
#[doc = "Reader of field `OSH7`"]
pub type OSH7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSH7`"]
pub struct OSH7_W<'a> {
    w: &'a mut W,
}
impl<'a> OSH7_W<'a> {
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
#[doc = "Reader of field `OSL0`"]
pub type OSL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSL0`"]
pub struct OSL0_W<'a> {
    w: &'a mut W,
}
impl<'a> OSL0_W<'a> {
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
#[doc = "Reader of field `OSL1`"]
pub type OSL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSL1`"]
pub struct OSL1_W<'a> {
    w: &'a mut W,
}
impl<'a> OSL1_W<'a> {
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
#[doc = "Reader of field `OSL2`"]
pub type OSL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSL2`"]
pub struct OSL2_W<'a> {
    w: &'a mut W,
}
impl<'a> OSL2_W<'a> {
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
#[doc = "Reader of field `OSL3`"]
pub type OSL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSL3`"]
pub struct OSL3_W<'a> {
    w: &'a mut W,
}
impl<'a> OSL3_W<'a> {
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
#[doc = "Reader of field `OSL4`"]
pub type OSL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSL4`"]
pub struct OSL4_W<'a> {
    w: &'a mut W,
}
impl<'a> OSL4_W<'a> {
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
#[doc = "Reader of field `OSL5`"]
pub type OSL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSL5`"]
pub struct OSL5_W<'a> {
    w: &'a mut W,
}
impl<'a> OSL5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `OSL6`"]
pub type OSL6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSL6`"]
pub struct OSL6_W<'a> {
    w: &'a mut W,
}
impl<'a> OSL6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `OSL7`"]
pub type OSL7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OSL7`"]
pub struct OSL7_W<'a> {
    w: &'a mut W,
}
impl<'a> OSL7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Output Selection for PWMH output of the channel 0"]
    #[inline(always)]
    pub fn osh0(&self) -> OSH0_R {
        OSH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Output Selection for PWMH output of the channel 1"]
    #[inline(always)]
    pub fn osh1(&self) -> OSH1_R {
        OSH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Output Selection for PWMH output of the channel 2"]
    #[inline(always)]
    pub fn osh2(&self) -> OSH2_R {
        OSH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output Selection for PWMH output of the channel 3"]
    #[inline(always)]
    pub fn osh3(&self) -> OSH3_R {
        OSH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Output Selection for PWMH output of the channel 4"]
    #[inline(always)]
    pub fn osh4(&self) -> OSH4_R {
        OSH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Output Selection for PWMH output of the channel 5"]
    #[inline(always)]
    pub fn osh5(&self) -> OSH5_R {
        OSH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Output Selection for PWMH output of the channel 6"]
    #[inline(always)]
    pub fn osh6(&self) -> OSH6_R {
        OSH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Output Selection for PWMH output of the channel 7"]
    #[inline(always)]
    pub fn osh7(&self) -> OSH7_R {
        OSH7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Output Selection for PWML output of the channel 0"]
    #[inline(always)]
    pub fn osl0(&self) -> OSL0_R {
        OSL0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Output Selection for PWML output of the channel 1"]
    #[inline(always)]
    pub fn osl1(&self) -> OSL1_R {
        OSL1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Output Selection for PWML output of the channel 2"]
    #[inline(always)]
    pub fn osl2(&self) -> OSL2_R {
        OSL2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Output Selection for PWML output of the channel 3"]
    #[inline(always)]
    pub fn osl3(&self) -> OSL3_R {
        OSL3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Output Selection for PWML output of the channel 4"]
    #[inline(always)]
    pub fn osl4(&self) -> OSL4_R {
        OSL4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Output Selection for PWML output of the channel 5"]
    #[inline(always)]
    pub fn osl5(&self) -> OSL5_R {
        OSL5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Output Selection for PWML output of the channel 6"]
    #[inline(always)]
    pub fn osl6(&self) -> OSL6_R {
        OSL6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Output Selection for PWML output of the channel 7"]
    #[inline(always)]
    pub fn osl7(&self) -> OSL7_R {
        OSL7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output Selection for PWMH output of the channel 0"]
    #[inline(always)]
    pub fn osh0(&mut self) -> OSH0_W {
        OSH0_W { w: self }
    }
    #[doc = "Bit 1 - Output Selection for PWMH output of the channel 1"]
    #[inline(always)]
    pub fn osh1(&mut self) -> OSH1_W {
        OSH1_W { w: self }
    }
    #[doc = "Bit 2 - Output Selection for PWMH output of the channel 2"]
    #[inline(always)]
    pub fn osh2(&mut self) -> OSH2_W {
        OSH2_W { w: self }
    }
    #[doc = "Bit 3 - Output Selection for PWMH output of the channel 3"]
    #[inline(always)]
    pub fn osh3(&mut self) -> OSH3_W {
        OSH3_W { w: self }
    }
    #[doc = "Bit 4 - Output Selection for PWMH output of the channel 4"]
    #[inline(always)]
    pub fn osh4(&mut self) -> OSH4_W {
        OSH4_W { w: self }
    }
    #[doc = "Bit 5 - Output Selection for PWMH output of the channel 5"]
    #[inline(always)]
    pub fn osh5(&mut self) -> OSH5_W {
        OSH5_W { w: self }
    }
    #[doc = "Bit 6 - Output Selection for PWMH output of the channel 6"]
    #[inline(always)]
    pub fn osh6(&mut self) -> OSH6_W {
        OSH6_W { w: self }
    }
    #[doc = "Bit 7 - Output Selection for PWMH output of the channel 7"]
    #[inline(always)]
    pub fn osh7(&mut self) -> OSH7_W {
        OSH7_W { w: self }
    }
    #[doc = "Bit 16 - Output Selection for PWML output of the channel 0"]
    #[inline(always)]
    pub fn osl0(&mut self) -> OSL0_W {
        OSL0_W { w: self }
    }
    #[doc = "Bit 17 - Output Selection for PWML output of the channel 1"]
    #[inline(always)]
    pub fn osl1(&mut self) -> OSL1_W {
        OSL1_W { w: self }
    }
    #[doc = "Bit 18 - Output Selection for PWML output of the channel 2"]
    #[inline(always)]
    pub fn osl2(&mut self) -> OSL2_W {
        OSL2_W { w: self }
    }
    #[doc = "Bit 19 - Output Selection for PWML output of the channel 3"]
    #[inline(always)]
    pub fn osl3(&mut self) -> OSL3_W {
        OSL3_W { w: self }
    }
    #[doc = "Bit 20 - Output Selection for PWML output of the channel 4"]
    #[inline(always)]
    pub fn osl4(&mut self) -> OSL4_W {
        OSL4_W { w: self }
    }
    #[doc = "Bit 21 - Output Selection for PWML output of the channel 5"]
    #[inline(always)]
    pub fn osl5(&mut self) -> OSL5_W {
        OSL5_W { w: self }
    }
    #[doc = "Bit 22 - Output Selection for PWML output of the channel 6"]
    #[inline(always)]
    pub fn osl6(&mut self) -> OSL6_W {
        OSL6_W { w: self }
    }
    #[doc = "Bit 23 - Output Selection for PWML output of the channel 7"]
    #[inline(always)]
    pub fn osl7(&mut self) -> OSL7_W {
        OSL7_W { w: self }
    }
}
