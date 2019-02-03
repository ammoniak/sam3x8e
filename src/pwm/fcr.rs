#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FCR {
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let bits = self.register.get();
        let mut w = W { bits: bits };
        f(&mut w);
        self.register.set(w.bits);
    }
}
#[doc = r" Proxy"]
pub struct _FCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _FCLRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Fault Clear (fault input bit varies from 0 to 5)"]
    #[inline]
    pub fn fclr(&mut self) -> _FCLRW {
        _FCLRW { w: self }
    }
}
