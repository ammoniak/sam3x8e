#[doc = "Writer for register PMC_SCDR"]
pub type W = crate::W<u32, super::PMC_SCDR>;
#[doc = "Write proxy for field `UOTGCLK`"]
pub struct UOTGCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> UOTGCLK_W<'a> {
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
#[doc = "Write proxy for field `PCK0`"]
pub struct PCK0_W<'a> {
    w: &'a mut W,
}
impl<'a> PCK0_W<'a> {
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
#[doc = "Write proxy for field `PCK1`"]
pub struct PCK1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCK1_W<'a> {
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
#[doc = "Write proxy for field `PCK2`"]
pub struct PCK2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCK2_W<'a> {
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
impl W {
    #[doc = "Bit 5 - Disable USB OTG Clock (48 MHz, USB_48M) for UTMI"]
    #[inline(always)]
    pub fn uotgclk(&mut self) -> UOTGCLK_W {
        UOTGCLK_W { w: self }
    }
    #[doc = "Bit 8 - Programmable Clock 0 Output Disable"]
    #[inline(always)]
    pub fn pck0(&mut self) -> PCK0_W {
        PCK0_W { w: self }
    }
    #[doc = "Bit 9 - Programmable Clock 1 Output Disable"]
    #[inline(always)]
    pub fn pck1(&mut self) -> PCK1_W {
        PCK1_W { w: self }
    }
    #[doc = "Bit 10 - Programmable Clock 2 Output Disable"]
    #[inline(always)]
    pub fn pck2(&mut self) -> PCK2_W {
        PCK2_W { w: self }
    }
}
