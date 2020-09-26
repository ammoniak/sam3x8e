#[doc = "Writer for register FCR"]
pub type W = crate::W<u32, super::FCR>;
#[doc = "Flash Command"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FCMD_AW {
    #[doc = "0: Get Flash Descriptor"]
    GETD = 0,
    #[doc = "1: Write page"]
    WP = 1,
    #[doc = "2: Write page and lock"]
    WPL = 2,
    #[doc = "3: Erase page and write page"]
    EWP = 3,
    #[doc = "4: Erase page and write page then lock"]
    EWPL = 4,
    #[doc = "5: Erase all"]
    EA = 5,
    #[doc = "8: Set Lock Bit"]
    SLB = 8,
    #[doc = "9: Clear Lock Bit"]
    CLB = 9,
    #[doc = "10: Get Lock Bit"]
    GLB = 10,
    #[doc = "11: Set GPNVM Bit"]
    SGPB = 11,
    #[doc = "12: Clear GPNVM Bit"]
    CGPB = 12,
    #[doc = "13: Get GPNVM Bit"]
    GGPB = 13,
    #[doc = "14: Start Read Unique Identifier"]
    STUI = 14,
    #[doc = "15: Stop Read Unique Identifier"]
    SPUI = 15,
    #[doc = "16: Get CALIB Bit"]
    GCALB = 16,
}
impl From<FCMD_AW> for u8 {
    #[inline(always)]
    fn from(variant: FCMD_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `FCMD`"]
pub struct FCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> FCMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCMD_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Get Flash Descriptor"]
    #[inline(always)]
    pub fn getd(self) -> &'a mut W {
        self.variant(FCMD_AW::GETD)
    }
    #[doc = "Write page"]
    #[inline(always)]
    pub fn wp(self) -> &'a mut W {
        self.variant(FCMD_AW::WP)
    }
    #[doc = "Write page and lock"]
    #[inline(always)]
    pub fn wpl(self) -> &'a mut W {
        self.variant(FCMD_AW::WPL)
    }
    #[doc = "Erase page and write page"]
    #[inline(always)]
    pub fn ewp(self) -> &'a mut W {
        self.variant(FCMD_AW::EWP)
    }
    #[doc = "Erase page and write page then lock"]
    #[inline(always)]
    pub fn ewpl(self) -> &'a mut W {
        self.variant(FCMD_AW::EWPL)
    }
    #[doc = "Erase all"]
    #[inline(always)]
    pub fn ea(self) -> &'a mut W {
        self.variant(FCMD_AW::EA)
    }
    #[doc = "Set Lock Bit"]
    #[inline(always)]
    pub fn slb(self) -> &'a mut W {
        self.variant(FCMD_AW::SLB)
    }
    #[doc = "Clear Lock Bit"]
    #[inline(always)]
    pub fn clb(self) -> &'a mut W {
        self.variant(FCMD_AW::CLB)
    }
    #[doc = "Get Lock Bit"]
    #[inline(always)]
    pub fn glb(self) -> &'a mut W {
        self.variant(FCMD_AW::GLB)
    }
    #[doc = "Set GPNVM Bit"]
    #[inline(always)]
    pub fn sgpb(self) -> &'a mut W {
        self.variant(FCMD_AW::SGPB)
    }
    #[doc = "Clear GPNVM Bit"]
    #[inline(always)]
    pub fn cgpb(self) -> &'a mut W {
        self.variant(FCMD_AW::CGPB)
    }
    #[doc = "Get GPNVM Bit"]
    #[inline(always)]
    pub fn ggpb(self) -> &'a mut W {
        self.variant(FCMD_AW::GGPB)
    }
    #[doc = "Start Read Unique Identifier"]
    #[inline(always)]
    pub fn stui(self) -> &'a mut W {
        self.variant(FCMD_AW::STUI)
    }
    #[doc = "Stop Read Unique Identifier"]
    #[inline(always)]
    pub fn spui(self) -> &'a mut W {
        self.variant(FCMD_AW::SPUI)
    }
    #[doc = "Get CALIB Bit"]
    #[inline(always)]
    pub fn gcalb(self) -> &'a mut W {
        self.variant(FCMD_AW::GCALB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Write proxy for field `FARG`"]
pub struct FARG_W<'a> {
    w: &'a mut W,
}
impl<'a> FARG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | (((value as u32) & 0xffff) << 8);
        self.w
    }
}
#[doc = "Flash Writing Protection Key"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FKEY_AW {
    #[doc = "90: The 0x5A value enables the command defined by the bits of the register. If the field is written with a different value, the write is not performed and no action is started."]
    PASSWD = 90,
}
impl From<FKEY_AW> for u8 {
    #[inline(always)]
    fn from(variant: FKEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `FKEY`"]
pub struct FKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> FKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FKEY_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The 0x5A value enables the command defined by the bits of the register. If the field is written with a different value, the write is not performed and no action is started."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(FKEY_AW::PASSWD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Flash Command"]
    #[inline(always)]
    pub fn fcmd(&mut self) -> FCMD_W {
        FCMD_W { w: self }
    }
    #[doc = "Bits 8:23 - Flash Command Argument"]
    #[inline(always)]
    pub fn farg(&mut self) -> FARG_W {
        FARG_W { w: self }
    }
    #[doc = "Bits 24:31 - Flash Writing Protection Key"]
    #[inline(always)]
    pub fn fkey(&mut self) -> FKEY_W {
        FKEY_W { w: self }
    }
}
