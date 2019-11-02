#[doc = "Writer for register MCR1"]
pub type W = crate::W<u32, super::MCR1>;
#[doc = "Write proxy for field `MDLC`"]
pub struct MDLC_W<'a> {
    w: &'a mut W,
}
impl<'a> MDLC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `MRTR`"]
pub struct MRTR_W<'a> {
    w: &'a mut W,
}
impl<'a> MRTR_W<'a> {
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
#[doc = "Write proxy for field `MACR`"]
pub struct MACR_W<'a> {
    w: &'a mut W,
}
impl<'a> MACR_W<'a> {
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
#[doc = "Write proxy for field `MTCR`"]
pub struct MTCR_W<'a> {
    w: &'a mut W,
}
impl<'a> MTCR_W<'a> {
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
impl W {
    #[doc = "Bits 16:19 - Mailbox Data Length Code"]
    #[inline(always)]
    pub fn mdlc(&mut self) -> MDLC_W {
        MDLC_W { w: self }
    }
    #[doc = "Bit 20 - Mailbox Remote Transmission Request"]
    #[inline(always)]
    pub fn mrtr(&mut self) -> MRTR_W {
        MRTR_W { w: self }
    }
    #[doc = "Bit 22 - Abort Request for Mailbox x"]
    #[inline(always)]
    pub fn macr(&mut self) -> MACR_W {
        MACR_W { w: self }
    }
    #[doc = "Bit 23 - Mailbox Transfer Command"]
    #[inline(always)]
    pub fn mtcr(&mut self) -> MTCR_W {
        MTCR_W { w: self }
    }
}
