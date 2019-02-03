#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WPCR {
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
}
#[doc = r" Proxy"]
pub struct _WP_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _WP_ENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WP_KEY`"]
pub enum WP_KEYW {
    #[doc = "Writing any other value in this field aborts the write operation of the WP_EN bit. Always reads as 0."]
    PASSWD,
}
impl WP_KEYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            WP_KEYW::PASSWD => 5459267,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WP_KEYW<'a> {
    w: &'a mut W,
}
impl<'a> _WP_KEYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WP_KEYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Writing any other value in this field aborts the write operation of the WP_EN bit. Always reads as 0."]
    #[inline]
    pub fn passwd(self) -> &'a mut W {
        self.variant(WP_KEYW::PASSWD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline]
    pub fn wp_en(&mut self) -> _WP_ENW {
        _WP_ENW { w: self }
    }
    #[doc = "Bits 8:31 - Write Protection KEY Password"]
    #[inline]
    pub fn wp_key(&mut self) -> _WP_KEYW {
        _WP_KEYW { w: self }
    }
}
