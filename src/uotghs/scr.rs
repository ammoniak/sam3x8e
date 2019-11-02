#[doc = "Writer for register SCR"]
pub type W = crate::W<u32, super::SCR>;
#[doc = "Write proxy for field `IDTIC`"]
pub struct IDTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> IDTIC_W<'a> {
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
#[doc = "Write proxy for field `VBUSTIC`"]
pub struct VBUSTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSTIC_W<'a> {
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
#[doc = "Write proxy for field `SRPIC`"]
pub struct SRPIC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRPIC_W<'a> {
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
#[doc = "Write proxy for field `VBERRIC`"]
pub struct VBERRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> VBERRIC_W<'a> {
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
#[doc = "Write proxy for field `BCERRIC`"]
pub struct BCERRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> BCERRIC_W<'a> {
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
#[doc = "Write proxy for field `ROLEEXIC`"]
pub struct ROLEEXIC_W<'a> {
    w: &'a mut W,
}
impl<'a> ROLEEXIC_W<'a> {
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
#[doc = "Write proxy for field `HNPERRIC`"]
pub struct HNPERRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> HNPERRIC_W<'a> {
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
#[doc = "Write proxy for field `STOIC`"]
pub struct STOIC_W<'a> {
    w: &'a mut W,
}
impl<'a> STOIC_W<'a> {
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
#[doc = "Write proxy for field `VBUSRQC`"]
pub struct VBUSRQC_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSRQC_W<'a> {
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
impl W {
    #[doc = "Bit 0 - ID Transition Interrupt Clear"]
    #[inline(always)]
    pub fn idtic(&mut self) -> IDTIC_W {
        IDTIC_W { w: self }
    }
    #[doc = "Bit 1 - VBus Transition Interrupt Clear"]
    #[inline(always)]
    pub fn vbustic(&mut self) -> VBUSTIC_W {
        VBUSTIC_W { w: self }
    }
    #[doc = "Bit 2 - SRP Interrupt Clear"]
    #[inline(always)]
    pub fn srpic(&mut self) -> SRPIC_W {
        SRPIC_W { w: self }
    }
    #[doc = "Bit 3 - VBus Error Interrupt Clear"]
    #[inline(always)]
    pub fn vberric(&mut self) -> VBERRIC_W {
        VBERRIC_W { w: self }
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt Clear"]
    #[inline(always)]
    pub fn bcerric(&mut self) -> BCERRIC_W {
        BCERRIC_W { w: self }
    }
    #[doc = "Bit 5 - Role Exchange Interrupt Clear"]
    #[inline(always)]
    pub fn roleexic(&mut self) -> ROLEEXIC_W {
        ROLEEXIC_W { w: self }
    }
    #[doc = "Bit 6 - HNP Error Interrupt Clear"]
    #[inline(always)]
    pub fn hnperric(&mut self) -> HNPERRIC_W {
        HNPERRIC_W { w: self }
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt Clear"]
    #[inline(always)]
    pub fn stoic(&mut self) -> STOIC_W {
        STOIC_W { w: self }
    }
    #[doc = "Bit 9 - VBus Request Clear"]
    #[inline(always)]
    pub fn vbusrqc(&mut self) -> VBUSRQC_W {
        VBUSRQC_W { w: self }
    }
}
