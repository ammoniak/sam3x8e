#[doc = "Reader of register FPE2"]
pub type R = crate::R<u32, super::FPE2>;
#[doc = "Writer for register FPE2"]
pub type W = crate::W<u32, super::FPE2>;
#[doc = "Register FPE2 `reset()`'s with value 0"]
impl crate::ResetValue for super::FPE2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FPE4`"]
pub type FPE4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FPE4`"]
pub struct FPE4_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `FPE5`"]
pub type FPE5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FPE5`"]
pub struct FPE5_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `FPE6`"]
pub type FPE6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FPE6`"]
pub struct FPE6_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `FPE7`"]
pub type FPE7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FPE7`"]
pub struct FPE7_W<'a> {
    w: &'a mut W,
}
impl<'a> FPE7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Fault Protection Enable for channel 4 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe4(&self) -> FPE4_R {
        FPE4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fault Protection Enable for channel 5 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe5(&self) -> FPE5_R {
        FPE5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Fault Protection Enable for channel 6 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe6(&self) -> FPE6_R {
        FPE6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Fault Protection Enable for channel 7 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe7(&self) -> FPE7_R {
        FPE7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fault Protection Enable for channel 4 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe4(&mut self) -> FPE4_W {
        FPE4_W { w: self }
    }
    #[doc = "Bits 8:15 - Fault Protection Enable for channel 5 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe5(&mut self) -> FPE5_W {
        FPE5_W { w: self }
    }
    #[doc = "Bits 16:23 - Fault Protection Enable for channel 6 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe6(&mut self) -> FPE6_W {
        FPE6_W { w: self }
    }
    #[doc = "Bits 24:31 - Fault Protection Enable for channel 7 (fault input bit varies from 0 to 5)"]
    #[inline(always)]
    pub fn fpe7(&mut self) -> FPE7_W {
        FPE7_W { w: self }
    }
}
