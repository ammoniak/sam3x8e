#[doc = "Reader of register FPV"]
pub type R = crate::R<u32, super::FPV>;
#[doc = "Writer for register FPV"]
pub type W = crate::W<u32, super::FPV>;
#[doc = "Register FPV `reset()`'s with value 0"]
impl crate::ResetValue for super::FPV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FPVH0`"]
pub type FPVH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPVH0`"]
pub struct FPVH0_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVH0_W<'a> {
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
#[doc = "Reader of field `FPVH1`"]
pub type FPVH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPVH1`"]
pub struct FPVH1_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVH1_W<'a> {
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
#[doc = "Reader of field `FPVH2`"]
pub type FPVH2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPVH2`"]
pub struct FPVH2_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVH2_W<'a> {
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
#[doc = "Reader of field `FPVH3`"]
pub type FPVH3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPVH3`"]
pub struct FPVH3_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVH3_W<'a> {
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
#[doc = "Reader of field `FPVH4`"]
pub type FPVH4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPVH4`"]
pub struct FPVH4_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVH4_W<'a> {
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
#[doc = "Reader of field `FPVH5`"]
pub type FPVH5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPVH5`"]
pub struct FPVH5_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVH5_W<'a> {
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
#[doc = "Reader of field `FPVH6`"]
pub type FPVH6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPVH6`"]
pub struct FPVH6_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVH6_W<'a> {
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
#[doc = "Reader of field `FPVH7`"]
pub type FPVH7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPVH7`"]
pub struct FPVH7_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVH7_W<'a> {
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
#[doc = "Reader of field `FPVL0`"]
pub type FPVL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPVL0`"]
pub struct FPVL0_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVL0_W<'a> {
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
#[doc = "Reader of field `FPVL1`"]
pub type FPVL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPVL1`"]
pub struct FPVL1_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVL1_W<'a> {
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
#[doc = "Reader of field `FPVL2`"]
pub type FPVL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPVL2`"]
pub struct FPVL2_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVL2_W<'a> {
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
#[doc = "Reader of field `FPVL3`"]
pub type FPVL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPVL3`"]
pub struct FPVL3_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVL3_W<'a> {
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
#[doc = "Reader of field `FPVL4`"]
pub type FPVL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPVL4`"]
pub struct FPVL4_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVL4_W<'a> {
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
#[doc = "Reader of field `FPVL5`"]
pub type FPVL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPVL5`"]
pub struct FPVL5_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVL5_W<'a> {
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
#[doc = "Reader of field `FPVL6`"]
pub type FPVL6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPVL6`"]
pub struct FPVL6_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVL6_W<'a> {
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
#[doc = "Reader of field `FPVL7`"]
pub type FPVL7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPVL7`"]
pub struct FPVL7_W<'a> {
    w: &'a mut W,
}
impl<'a> FPVL7_W<'a> {
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
    #[doc = "Bit 0 - Fault Protection Value for PWMH output on channel 0"]
    #[inline(always)]
    pub fn fpvh0(&self) -> FPVH0_R {
        FPVH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fault Protection Value for PWMH output on channel 1"]
    #[inline(always)]
    pub fn fpvh1(&self) -> FPVH1_R {
        FPVH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fault Protection Value for PWMH output on channel 2"]
    #[inline(always)]
    pub fn fpvh2(&self) -> FPVH2_R {
        FPVH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fault Protection Value for PWMH output on channel 3"]
    #[inline(always)]
    pub fn fpvh3(&self) -> FPVH3_R {
        FPVH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fault Protection Value for PWMH output on channel 4"]
    #[inline(always)]
    pub fn fpvh4(&self) -> FPVH4_R {
        FPVH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fault Protection Value for PWMH output on channel 5"]
    #[inline(always)]
    pub fn fpvh5(&self) -> FPVH5_R {
        FPVH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fault Protection Value for PWMH output on channel 6"]
    #[inline(always)]
    pub fn fpvh6(&self) -> FPVH6_R {
        FPVH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fault Protection Value for PWMH output on channel 7"]
    #[inline(always)]
    pub fn fpvh7(&self) -> FPVH7_R {
        FPVH7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fault Protection Value for PWML output on channel 0"]
    #[inline(always)]
    pub fn fpvl0(&self) -> FPVL0_R {
        FPVL0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fault Protection Value for PWML output on channel 1"]
    #[inline(always)]
    pub fn fpvl1(&self) -> FPVL1_R {
        FPVL1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fault Protection Value for PWML output on channel 2"]
    #[inline(always)]
    pub fn fpvl2(&self) -> FPVL2_R {
        FPVL2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fault Protection Value for PWML output on channel 3"]
    #[inline(always)]
    pub fn fpvl3(&self) -> FPVL3_R {
        FPVL3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Fault Protection Value for PWML output on channel 4"]
    #[inline(always)]
    pub fn fpvl4(&self) -> FPVL4_R {
        FPVL4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Fault Protection Value for PWML output on channel 5"]
    #[inline(always)]
    pub fn fpvl5(&self) -> FPVL5_R {
        FPVL5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Fault Protection Value for PWML output on channel 6"]
    #[inline(always)]
    pub fn fpvl6(&self) -> FPVL6_R {
        FPVL6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Fault Protection Value for PWML output on channel 7"]
    #[inline(always)]
    pub fn fpvl7(&self) -> FPVL7_R {
        FPVL7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Protection Value for PWMH output on channel 0"]
    #[inline(always)]
    pub fn fpvh0(&mut self) -> FPVH0_W {
        FPVH0_W { w: self }
    }
    #[doc = "Bit 1 - Fault Protection Value for PWMH output on channel 1"]
    #[inline(always)]
    pub fn fpvh1(&mut self) -> FPVH1_W {
        FPVH1_W { w: self }
    }
    #[doc = "Bit 2 - Fault Protection Value for PWMH output on channel 2"]
    #[inline(always)]
    pub fn fpvh2(&mut self) -> FPVH2_W {
        FPVH2_W { w: self }
    }
    #[doc = "Bit 3 - Fault Protection Value for PWMH output on channel 3"]
    #[inline(always)]
    pub fn fpvh3(&mut self) -> FPVH3_W {
        FPVH3_W { w: self }
    }
    #[doc = "Bit 4 - Fault Protection Value for PWMH output on channel 4"]
    #[inline(always)]
    pub fn fpvh4(&mut self) -> FPVH4_W {
        FPVH4_W { w: self }
    }
    #[doc = "Bit 5 - Fault Protection Value for PWMH output on channel 5"]
    #[inline(always)]
    pub fn fpvh5(&mut self) -> FPVH5_W {
        FPVH5_W { w: self }
    }
    #[doc = "Bit 6 - Fault Protection Value for PWMH output on channel 6"]
    #[inline(always)]
    pub fn fpvh6(&mut self) -> FPVH6_W {
        FPVH6_W { w: self }
    }
    #[doc = "Bit 7 - Fault Protection Value for PWMH output on channel 7"]
    #[inline(always)]
    pub fn fpvh7(&mut self) -> FPVH7_W {
        FPVH7_W { w: self }
    }
    #[doc = "Bit 16 - Fault Protection Value for PWML output on channel 0"]
    #[inline(always)]
    pub fn fpvl0(&mut self) -> FPVL0_W {
        FPVL0_W { w: self }
    }
    #[doc = "Bit 17 - Fault Protection Value for PWML output on channel 1"]
    #[inline(always)]
    pub fn fpvl1(&mut self) -> FPVL1_W {
        FPVL1_W { w: self }
    }
    #[doc = "Bit 18 - Fault Protection Value for PWML output on channel 2"]
    #[inline(always)]
    pub fn fpvl2(&mut self) -> FPVL2_W {
        FPVL2_W { w: self }
    }
    #[doc = "Bit 19 - Fault Protection Value for PWML output on channel 3"]
    #[inline(always)]
    pub fn fpvl3(&mut self) -> FPVL3_W {
        FPVL3_W { w: self }
    }
    #[doc = "Bit 20 - Fault Protection Value for PWML output on channel 4"]
    #[inline(always)]
    pub fn fpvl4(&mut self) -> FPVL4_W {
        FPVL4_W { w: self }
    }
    #[doc = "Bit 21 - Fault Protection Value for PWML output on channel 5"]
    #[inline(always)]
    pub fn fpvl5(&mut self) -> FPVL5_W {
        FPVL5_W { w: self }
    }
    #[doc = "Bit 22 - Fault Protection Value for PWML output on channel 6"]
    #[inline(always)]
    pub fn fpvl6(&mut self) -> FPVL6_W {
        FPVL6_W { w: self }
    }
    #[doc = "Bit 23 - Fault Protection Value for PWML output on channel 7"]
    #[inline(always)]
    pub fn fpvl7(&mut self) -> FPVL7_W {
        FPVL7_W { w: self }
    }
}
