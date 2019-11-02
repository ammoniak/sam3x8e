#[doc = "Reader of register CCFG_SYSIO"]
pub type R = crate::R<u32, super::CCFG_SYSIO>;
#[doc = "Writer for register CCFG_SYSIO"]
pub type W = crate::W<u32, super::CCFG_SYSIO>;
#[doc = "Register CCFG_SYSIO `reset()`'s with value 0"]
impl crate::ResetValue for super::CCFG_SYSIO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSIO12`"]
pub type SYSIO12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSIO12`"]
pub struct SYSIO12_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSIO12_W<'a> {
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
impl R {
    #[doc = "Bit 12 - PC0 or ERASE Assignment"]
    #[inline(always)]
    pub fn sysio12(&self) -> SYSIO12_R {
        SYSIO12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - PC0 or ERASE Assignment"]
    #[inline(always)]
    pub fn sysio12(&mut self) -> SYSIO12_W {
        SYSIO12_W { w: self }
    }
}
