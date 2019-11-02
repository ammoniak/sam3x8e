#[doc = "Reader of register ADDR"]
pub type R = crate::R<u32, super::ADDR>;
#[doc = "Writer for register ADDR"]
pub type W = crate::W<u32, super::ADDR>;
#[doc = "Register ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR_CYCLE0`"]
pub type ADDR_CYCLE0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDR_CYCLE0`"]
pub struct ADDR_CYCLE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_CYCLE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - NAND Flash Array Address cycle 0"]
    #[inline(always)]
    pub fn addr_cycle0(&self) -> ADDR_CYCLE0_R {
        ADDR_CYCLE0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NAND Flash Array Address cycle 0"]
    #[inline(always)]
    pub fn addr_cycle0(&mut self) -> ADDR_CYCLE0_W {
        ADDR_CYCLE0_W { w: self }
    }
}
