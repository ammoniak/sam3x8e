#[doc = "Writer for register ACR"]
pub type W = crate::W<u32, super::ACR>;
#[doc = "Write proxy for field `MB0`"]
pub struct MB0_W<'a> {
    w: &'a mut W,
}
impl<'a> MB0_W<'a> {
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
#[doc = "Write proxy for field `MB1`"]
pub struct MB1_W<'a> {
    w: &'a mut W,
}
impl<'a> MB1_W<'a> {
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
#[doc = "Write proxy for field `MB2`"]
pub struct MB2_W<'a> {
    w: &'a mut W,
}
impl<'a> MB2_W<'a> {
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
#[doc = "Write proxy for field `MB3`"]
pub struct MB3_W<'a> {
    w: &'a mut W,
}
impl<'a> MB3_W<'a> {
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
#[doc = "Write proxy for field `MB4`"]
pub struct MB4_W<'a> {
    w: &'a mut W,
}
impl<'a> MB4_W<'a> {
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
#[doc = "Write proxy for field `MB5`"]
pub struct MB5_W<'a> {
    w: &'a mut W,
}
impl<'a> MB5_W<'a> {
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
#[doc = "Write proxy for field `MB6`"]
pub struct MB6_W<'a> {
    w: &'a mut W,
}
impl<'a> MB6_W<'a> {
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
#[doc = "Write proxy for field `MB7`"]
pub struct MB7_W<'a> {
    w: &'a mut W,
}
impl<'a> MB7_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Abort Request for Mailbox 0"]
    #[inline(always)]
    pub fn mb0(&mut self) -> MB0_W {
        MB0_W { w: self }
    }
    #[doc = "Bit 1 - Abort Request for Mailbox 1"]
    #[inline(always)]
    pub fn mb1(&mut self) -> MB1_W {
        MB1_W { w: self }
    }
    #[doc = "Bit 2 - Abort Request for Mailbox 2"]
    #[inline(always)]
    pub fn mb2(&mut self) -> MB2_W {
        MB2_W { w: self }
    }
    #[doc = "Bit 3 - Abort Request for Mailbox 3"]
    #[inline(always)]
    pub fn mb3(&mut self) -> MB3_W {
        MB3_W { w: self }
    }
    #[doc = "Bit 4 - Abort Request for Mailbox 4"]
    #[inline(always)]
    pub fn mb4(&mut self) -> MB4_W {
        MB4_W { w: self }
    }
    #[doc = "Bit 5 - Abort Request for Mailbox 5"]
    #[inline(always)]
    pub fn mb5(&mut self) -> MB5_W {
        MB5_W { w: self }
    }
    #[doc = "Bit 6 - Abort Request for Mailbox 6"]
    #[inline(always)]
    pub fn mb6(&mut self) -> MB6_W {
        MB6_W { w: self }
    }
    #[doc = "Bit 7 - Abort Request for Mailbox 7"]
    #[inline(always)]
    pub fn mb7(&mut self) -> MB7_W {
        MB7_W { w: self }
    }
}
