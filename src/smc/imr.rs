#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `RB_RISE`"]
pub type RB_RISE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RB_FALL`"]
pub type RB_FALL_R = crate::R<bool, bool>;
#[doc = "Reader of field `XFRDONE`"]
pub type XFRDONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMDDONE`"]
pub type CMDDONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DTOE`"]
pub type DTOE_R = crate::R<bool, bool>;
#[doc = "Reader of field `UNDEF`"]
pub type UNDEF_R = crate::R<bool, bool>;
#[doc = "Reader of field `AWB`"]
pub type AWB_R = crate::R<bool, bool>;
#[doc = "Reader of field `NFCASE`"]
pub type NFCASE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RB_EDGE0`"]
pub type RB_EDGE0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 4 - Ready Busy Rising Edge Detection Interrupt Mask"]
    #[inline(always)]
    pub fn rb_rise(&self) -> RB_RISE_R {
        RB_RISE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Ready Busy Falling Edge Detection Interrupt Mask"]
    #[inline(always)]
    pub fn rb_fall(&self) -> RB_FALL_R {
        RB_FALL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transfer Done Interrupt Mask"]
    #[inline(always)]
    pub fn xfrdone(&self) -> XFRDONE_R {
        XFRDONE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Command Done Interrupt Mask"]
    #[inline(always)]
    pub fn cmddone(&self) -> CMDDONE_R {
        CMDDONE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error Interrupt Mask"]
    #[inline(always)]
    pub fn dtoe(&self) -> DTOE_R {
        DTOE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Undefined Area Access Interrupt Mask5"]
    #[inline(always)]
    pub fn undef(&self) -> UNDEF_R {
        UNDEF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Accessing While Busy Interrupt Mask"]
    #[inline(always)]
    pub fn awb(&self) -> AWB_R {
        AWB_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - NFC Access Size Error Interrupt Mask"]
    #[inline(always)]
    pub fn nfcase(&self) -> NFCASE_R {
        NFCASE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Ready/Busy Line 0 Interrupt Mask"]
    #[inline(always)]
    pub fn rb_edge0(&self) -> RB_EDGE0_R {
        RB_EDGE0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
