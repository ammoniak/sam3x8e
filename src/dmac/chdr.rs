#[doc = "Writer for register CHDR"]
pub type W = crate::W<u32, super::CHDR>;
#[doc = "Write proxy for field `DIS0`"]
pub struct DIS0_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS0_W<'a> {
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
#[doc = "Write proxy for field `DIS1`"]
pub struct DIS1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS1_W<'a> {
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
#[doc = "Write proxy for field `DIS2`"]
pub struct DIS2_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS2_W<'a> {
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
#[doc = "Write proxy for field `DIS3`"]
pub struct DIS3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS3_W<'a> {
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
#[doc = "Write proxy for field `DIS4`"]
pub struct DIS4_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS4_W<'a> {
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
#[doc = "Write proxy for field `DIS5`"]
pub struct DIS5_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS5_W<'a> {
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
#[doc = "Write proxy for field `RES0`"]
pub struct RES0_W<'a> {
    w: &'a mut W,
}
impl<'a> RES0_W<'a> {
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
#[doc = "Write proxy for field `RES1`"]
pub struct RES1_W<'a> {
    w: &'a mut W,
}
impl<'a> RES1_W<'a> {
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
#[doc = "Write proxy for field `RES2`"]
pub struct RES2_W<'a> {
    w: &'a mut W,
}
impl<'a> RES2_W<'a> {
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
#[doc = "Write proxy for field `RES3`"]
pub struct RES3_W<'a> {
    w: &'a mut W,
}
impl<'a> RES3_W<'a> {
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
#[doc = "Write proxy for field `RES4`"]
pub struct RES4_W<'a> {
    w: &'a mut W,
}
impl<'a> RES4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Write proxy for field `RES5`"]
pub struct RES5_W<'a> {
    w: &'a mut W,
}
impl<'a> RES5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Disable \\[5:0\\]"]
    #[inline(always)]
    pub fn dis0(&mut self) -> DIS0_W {
        DIS0_W { w: self }
    }
    #[doc = "Bit 1 - Disable \\[5:0\\]"]
    #[inline(always)]
    pub fn dis1(&mut self) -> DIS1_W {
        DIS1_W { w: self }
    }
    #[doc = "Bit 2 - Disable \\[5:0\\]"]
    #[inline(always)]
    pub fn dis2(&mut self) -> DIS2_W {
        DIS2_W { w: self }
    }
    #[doc = "Bit 3 - Disable \\[5:0\\]"]
    #[inline(always)]
    pub fn dis3(&mut self) -> DIS3_W {
        DIS3_W { w: self }
    }
    #[doc = "Bit 4 - Disable \\[5:0\\]"]
    #[inline(always)]
    pub fn dis4(&mut self) -> DIS4_W {
        DIS4_W { w: self }
    }
    #[doc = "Bit 5 - Disable \\[5:0\\]"]
    #[inline(always)]
    pub fn dis5(&mut self) -> DIS5_W {
        DIS5_W { w: self }
    }
    #[doc = "Bit 8 - Resume \\[5:0\\]"]
    #[inline(always)]
    pub fn res0(&mut self) -> RES0_W {
        RES0_W { w: self }
    }
    #[doc = "Bit 9 - Resume \\[5:0\\]"]
    #[inline(always)]
    pub fn res1(&mut self) -> RES1_W {
        RES1_W { w: self }
    }
    #[doc = "Bit 10 - Resume \\[5:0\\]"]
    #[inline(always)]
    pub fn res2(&mut self) -> RES2_W {
        RES2_W { w: self }
    }
    #[doc = "Bit 11 - Resume \\[5:0\\]"]
    #[inline(always)]
    pub fn res3(&mut self) -> RES3_W {
        RES3_W { w: self }
    }
    #[doc = "Bit 12 - Resume \\[5:0\\]"]
    #[inline(always)]
    pub fn res4(&mut self) -> RES4_W {
        RES4_W { w: self }
    }
    #[doc = "Bit 13 - Resume \\[5:0\\]"]
    #[inline(always)]
    pub fn res5(&mut self) -> RES5_W {
        RES5_W { w: self }
    }
}
