#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `SMCSTS`"]
pub type SMCSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RB_RISE`"]
pub type RB_RISE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RB_FALL`"]
pub type RB_FALL_R = crate::R<bool, bool>;
#[doc = "Reader of field `NFCBUSY`"]
pub type NFCBUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `NFCWR`"]
pub type NFCWR_R = crate::R<bool, bool>;
#[doc = "Reader of field `NFCSID`"]
pub type NFCSID_R = crate::R<u8, u8>;
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
    #[doc = "Bit 0 - NAND Flash Controller status (this field cannot be reset)"]
    #[inline(always)]
    pub fn smcsts(&self) -> SMCSTS_R {
        SMCSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Selected Ready Busy Rising Edge Detected"]
    #[inline(always)]
    pub fn rb_rise(&self) -> RB_RISE_R {
        RB_RISE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Selected Ready Busy Falling Edge Detected"]
    #[inline(always)]
    pub fn rb_fall(&self) -> RB_FALL_R {
        RB_FALL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - NFC Busy (this field cannot be reset)"]
    #[inline(always)]
    pub fn nfcbusy(&self) -> NFCBUSY_R {
        NFCBUSY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - NFC Write/Read Operation (this field cannot be reset)"]
    #[inline(always)]
    pub fn nfcwr(&self) -> NFCWR_R {
        NFCWR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - NFC Chip Select ID (this field cannot be reset)"]
    #[inline(always)]
    pub fn nfcsid(&self) -> NFCSID_R {
        NFCSID_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 16 - NFC Data Transfer Terminated"]
    #[inline(always)]
    pub fn xfrdone(&self) -> XFRDONE_R {
        XFRDONE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Command Done"]
    #[inline(always)]
    pub fn cmddone(&self) -> CMDDONE_R {
        CMDDONE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error"]
    #[inline(always)]
    pub fn dtoe(&self) -> DTOE_R {
        DTOE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Undefined Area Error"]
    #[inline(always)]
    pub fn undef(&self) -> UNDEF_R {
        UNDEF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Accessing While Busy"]
    #[inline(always)]
    pub fn awb(&self) -> AWB_R {
        AWB_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - NFC Access Size Error"]
    #[inline(always)]
    pub fn nfcase(&self) -> NFCASE_R {
        NFCASE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Ready/Busy Line 0 Edge Detected"]
    #[inline(always)]
    pub fn rb_edge0(&self) -> RB_EDGE0_R {
        RB_EDGE0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
