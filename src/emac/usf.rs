#[doc = "Reader of register USF"]
pub type R = crate::R<u32, super::USF>;
#[doc = "Writer for register USF"]
pub type W = crate::W<u32, super::USF>;
#[doc = "Register USF `reset()`'s with value 0"]
impl crate::ResetValue for super::USF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USF`"]
pub type USF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USF`"]
pub struct USF_W<'a> {
    w: &'a mut W,
}
impl<'a> USF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Undersize frames"]
    #[inline(always)]
    pub fn usf(&self) -> USF_R {
        USF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Undersize frames"]
    #[inline(always)]
    pub fn usf(&mut self) -> USF_W {
        USF_W { w: self }
    }
}
