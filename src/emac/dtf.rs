#[doc = "Reader of register DTF"]
pub type R = crate::R<u32, super::DTF>;
#[doc = "Writer for register DTF"]
pub type W = crate::W<u32, super::DTF>;
#[doc = "Register DTF `reset()`'s with value 0"]
impl crate::ResetValue for super::DTF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DTF`"]
pub type DTF_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DTF`"]
pub struct DTF_W<'a> {
    w: &'a mut W,
}
impl<'a> DTF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Deferred Transmission Frames"]
    #[inline(always)]
    pub fn dtf(&self) -> DTF_R {
        DTF_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Deferred Transmission Frames"]
    #[inline(always)]
    pub fn dtf(&mut self) -> DTF_W {
        DTF_W { w: self }
    }
}
