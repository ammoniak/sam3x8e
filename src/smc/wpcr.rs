#[doc = "Writer for register WPCR"]
pub type W = crate::W<u32, super::WPCR>;
#[doc = "Register WPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::WPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `WP_EN`"]
pub struct WP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WP_EN_W<'a> {
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
#[doc = "Write Protection KEY Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP_KEY_AW {
    #[doc = "5459267: Writing any other value in this field aborts the write operation of the WP_EN bit. Always reads as 0."]
    PASSWD,
}
impl From<WP_KEY_AW> for u32 {
    #[inline(always)]
    fn from(variant: WP_KEY_AW) -> Self {
        match variant {
            WP_KEY_AW::PASSWD => 5459267,
        }
    }
}
#[doc = "Write proxy for field `WP_KEY`"]
pub struct WP_KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> WP_KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WP_KEY_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Writing any other value in this field aborts the write operation of the WP_EN bit. Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(WP_KEY_AW::PASSWD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    pub fn wp_en(&mut self) -> WP_EN_W {
        WP_EN_W { w: self }
    }
    #[doc = "Bits 8:31 - Write Protection KEY Password"]
    #[inline(always)]
    pub fn wp_key(&mut self) -> WP_KEY_W {
        WP_KEY_W { w: self }
    }
}
