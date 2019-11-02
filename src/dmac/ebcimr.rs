#[doc = "Reader of register EBCIMR"]
pub type R = crate::R<u32, super::EBCIMR>;
#[doc = "Reader of field `BTC0`"]
pub type BTC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTC1`"]
pub type BTC1_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTC2`"]
pub type BTC2_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTC3`"]
pub type BTC3_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTC4`"]
pub type BTC4_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTC5`"]
pub type BTC5_R = crate::R<bool, bool>;
#[doc = "Reader of field `CBTC0`"]
pub type CBTC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CBTC1`"]
pub type CBTC1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CBTC2`"]
pub type CBTC2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CBTC3`"]
pub type CBTC3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CBTC4`"]
pub type CBTC4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CBTC5`"]
pub type CBTC5_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERR0`"]
pub type ERR0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERR1`"]
pub type ERR1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERR2`"]
pub type ERR2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERR3`"]
pub type ERR3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERR4`"]
pub type ERR4_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERR5`"]
pub type ERR5_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc0(&self) -> BTC0_R {
        BTC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc1(&self) -> BTC1_R {
        BTC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc2(&self) -> BTC2_R {
        BTC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc3(&self) -> BTC3_R {
        BTC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc4(&self) -> BTC4_R {
        BTC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc5(&self) -> BTC5_R {
        BTC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc0(&self) -> CBTC0_R {
        CBTC0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc1(&self) -> CBTC1_R {
        CBTC1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc2(&self) -> CBTC2_R {
        CBTC2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc3(&self) -> CBTC3_R {
        CBTC3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc4(&self) -> CBTC4_R {
        CBTC4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc5(&self) -> CBTC5_R {
        CBTC5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err0(&self) -> ERR0_R {
        ERR0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err1(&self) -> ERR1_R {
        ERR1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err2(&self) -> ERR2_R {
        ERR2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err3(&self) -> ERR3_R {
        ERR3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err4(&self) -> ERR4_R {
        ERR4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err5(&self) -> ERR5_R {
        ERR5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
