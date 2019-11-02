#[doc = "Writer for register SFR"]
pub type W = crate::W<u32, super::SFR>;
#[doc = "Write proxy for field `IDTIS`"]
pub struct IDTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> IDTIS_W<'a> {
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
#[doc = "Write proxy for field `VBUSTIS`"]
pub struct VBUSTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSTIS_W<'a> {
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
#[doc = "Write proxy for field `SRPIS`"]
pub struct SRPIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SRPIS_W<'a> {
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
#[doc = "Write proxy for field `VBERRIS`"]
pub struct VBERRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> VBERRIS_W<'a> {
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
#[doc = "Write proxy for field `BCERRIS`"]
pub struct BCERRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BCERRIS_W<'a> {
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
#[doc = "Write proxy for field `ROLEEXIS`"]
pub struct ROLEEXIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ROLEEXIS_W<'a> {
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
#[doc = "Write proxy for field `HNPERRIS`"]
pub struct HNPERRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HNPERRIS_W<'a> {
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
#[doc = "Write proxy for field `STOIS`"]
pub struct STOIS_W<'a> {
    w: &'a mut W,
}
impl<'a> STOIS_W<'a> {
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
#[doc = "Write proxy for field `VBUSRQS`"]
pub struct VBUSRQS_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSRQS_W<'a> {
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
    #[doc = "Bit 0 - ID Transition Interrupt Set"]
    #[inline(always)]
    pub fn idtis(&mut self) -> IDTIS_W {
        IDTIS_W { w: self }
    }
    #[doc = "Bit 1 - VBus Transition Interrupt Set"]
    #[inline(always)]
    pub fn vbustis(&mut self) -> VBUSTIS_W {
        VBUSTIS_W { w: self }
    }
    #[doc = "Bit 2 - SRP Interrupt Set"]
    #[inline(always)]
    pub fn srpis(&mut self) -> SRPIS_W {
        SRPIS_W { w: self }
    }
    #[doc = "Bit 3 - VBus Error Interrupt Set"]
    #[inline(always)]
    pub fn vberris(&mut self) -> VBERRIS_W {
        VBERRIS_W { w: self }
    }
    #[doc = "Bit 4 - B-Connection Error Interrupt Set"]
    #[inline(always)]
    pub fn bcerris(&mut self) -> BCERRIS_W {
        BCERRIS_W { w: self }
    }
    #[doc = "Bit 5 - Role Exchange Interrupt Set"]
    #[inline(always)]
    pub fn roleexis(&mut self) -> ROLEEXIS_W {
        ROLEEXIS_W { w: self }
    }
    #[doc = "Bit 6 - HNP Error Interrupt Set"]
    #[inline(always)]
    pub fn hnperris(&mut self) -> HNPERRIS_W {
        HNPERRIS_W { w: self }
    }
    #[doc = "Bit 7 - Suspend Time-Out Interrupt Set"]
    #[inline(always)]
    pub fn stois(&mut self) -> STOIS_W {
        STOIS_W { w: self }
    }
    #[doc = "Bit 9 - VBus Request Set"]
    #[inline(always)]
    pub fn vbusrqs(&mut self) -> VBUSRQS_W {
        VBUSRQS_W { w: self }
    }
}
