#[doc = "Reader of register DSCR2"]
pub type R = crate::R<u32, super::DSCR2>;
#[doc = "Writer for register DSCR2"]
pub type W = crate::W<u32, super::DSCR2>;
#[doc = "Register DSCR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DSCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DSCR`"]
pub type DSCR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DSCR`"]
pub struct DSCR_W<'a> {
    w: &'a mut W,
}
impl<'a> DSCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Buffer Transfer Descriptor Address"]
    #[inline(always)]
    pub fn dscr(&self) -> DSCR_R {
        DSCR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Buffer Transfer Descriptor Address"]
    #[inline(always)]
    pub fn dscr(&mut self) -> DSCR_W {
        DSCR_W { w: self }
    }
}
