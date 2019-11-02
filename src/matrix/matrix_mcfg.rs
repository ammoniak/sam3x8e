#[doc = "Reader of register MATRIX_MCFG[%s]"]
pub type R = crate::R<u32, super::MATRIX_MCFG>;
#[doc = "Writer for register MATRIX_MCFG[%s]"]
pub type W = crate::W<u32, super::MATRIX_MCFG>;
#[doc = "Reader of field `ULBT`"]
pub type ULBT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ULBT`"]
pub struct ULBT_W<'a> {
    w: &'a mut W,
}
impl<'a> ULBT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&self) -> ULBT_R {
        ULBT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&mut self) -> ULBT_W {
        ULBT_W { w: self }
    }
}
