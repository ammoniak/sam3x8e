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
#[doc = "Values that can be written to the field `FCMD`"]
pub enum FCMDW {
    #[doc = "Get Flash Descriptor"]
    GETD,
    #[doc = "Write page"]
    WP,
    #[doc = "Write page and lock"]
    WPL,
    #[doc = "Erase page and write page"]
    EWP,
    #[doc = "Erase page and write page then lock"]
    EWPL,
    #[doc = "Erase all"]
    EA,
    #[doc = "Set Lock Bit"]
    SLB,
    #[doc = "Clear Lock Bit"]
    CLB,
    #[doc = "Get Lock Bit"]
    GLB,
    #[doc = "Set GPNVM Bit"]
    SGPB,
    #[doc = "Clear GPNVM Bit"]
    CGPB,
    #[doc = "Get GPNVM Bit"]
    GGPB,
    #[doc = "Start Read Unique Identifier"]
    STUI,
    #[doc = "Stop Read Unique Identifier"]
    SPUI,
    #[doc = "Get CALIB Bit"]
    GCALB,
}
impl FCMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FCMDW::GETD => 0,
            FCMDW::WP => 1,
            FCMDW::WPL => 2,
            FCMDW::EWP => 3,
            FCMDW::EWPL => 4,
            FCMDW::EA => 5,
            FCMDW::SLB => 8,
            FCMDW::CLB => 9,
            FCMDW::GLB => 10,
            FCMDW::SGPB => 11,
            FCMDW::CGPB => 12,
            FCMDW::GGPB => 13,
            FCMDW::STUI => 14,
            FCMDW::SPUI => 15,
            FCMDW::GCALB => 16,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _FCMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCMDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Get Flash Descriptor"]
    #[inline]
    pub fn getd(self) -> &'a mut W {
        self.variant(FCMDW::GETD)
    }
    #[doc = "Write page"]
    #[inline]
    pub fn wp(self) -> &'a mut W {
        self.variant(FCMDW::WP)
    }
    #[doc = "Write page and lock"]
    #[inline]
    pub fn wpl(self) -> &'a mut W {
        self.variant(FCMDW::WPL)
    }
    #[doc = "Erase page and write page"]
    #[inline]
    pub fn ewp(self) -> &'a mut W {
        self.variant(FCMDW::EWP)
    }
    #[doc = "Erase page and write page then lock"]
    #[inline]
    pub fn ewpl(self) -> &'a mut W {
        self.variant(FCMDW::EWPL)
    }
    #[doc = "Erase all"]
    #[inline]
    pub fn ea(self) -> &'a mut W {
        self.variant(FCMDW::EA)
    }
    #[doc = "Set Lock Bit"]
    #[inline]
    pub fn slb(self) -> &'a mut W {
        self.variant(FCMDW::SLB)
    }
    #[doc = "Clear Lock Bit"]
    #[inline]
    pub fn clb(self) -> &'a mut W {
        self.variant(FCMDW::CLB)
    }
    #[doc = "Get Lock Bit"]
    #[inline]
    pub fn glb(self) -> &'a mut W {
        self.variant(FCMDW::GLB)
    }
    #[doc = "Set GPNVM Bit"]
    #[inline]
    pub fn sgpb(self) -> &'a mut W {
        self.variant(FCMDW::SGPB)
    }
    #[doc = "Clear GPNVM Bit"]
    #[inline]
    pub fn cgpb(self) -> &'a mut W {
        self.variant(FCMDW::CGPB)
    }
    #[doc = "Get GPNVM Bit"]
    #[inline]
    pub fn ggpb(self) -> &'a mut W {
        self.variant(FCMDW::GGPB)
    }
    #[doc = "Start Read Unique Identifier"]
    #[inline]
    pub fn stui(self) -> &'a mut W {
        self.variant(FCMDW::STUI)
    }
    #[doc = "Stop Read Unique Identifier"]
    #[inline]
    pub fn spui(self) -> &'a mut W {
        self.variant(FCMDW::SPUI)
    }
    #[doc = "Get CALIB Bit"]
    #[inline]
    pub fn gcalb(self) -> &'a mut W {
        self.variant(FCMDW::GCALB)
    }
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
#[doc = r" Proxy"]
pub struct _FARGW<'a> {
    w: &'a mut W,
}
impl<'a> _FARGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FKEY`"]
pub enum FKEYW {
    #[doc = "The 0x5A value enables the command defined by the bits of the register. If the field is written with a different value, the write is not performed and no action is started."]
    PASSWD,
}
impl FKEYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FKEYW::PASSWD => 90,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FKEYW<'a> {
    w: &'a mut W,
}
impl<'a> _FKEYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FKEYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The 0x5A value enables the command defined by the bits of the register. If the field is written with a different value, the write is not performed and no action is started."]
    #[inline]
    pub fn passwd(self) -> &'a mut W {
        self.variant(FKEYW::PASSWD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Flash Command"]
    #[inline]
    pub fn fcmd(&mut self) -> _FCMDW {
        _FCMDW { w: self }
    }
    #[doc = "Bits 8:23 - Flash Command Argument"]
    #[inline]
    pub fn farg(&mut self) -> _FARGW {
        _FARGW { w: self }
    }
    #[doc = "Bits 24:31 - Flash Writing Protection Key"]
    #[inline]
    pub fn fkey(&mut self) -> _FKEYW {
        _FKEYW { w: self }
    }
}
