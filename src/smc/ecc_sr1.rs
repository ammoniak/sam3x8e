#[doc = "Reader of register ECC_SR1"]
pub type R = crate::R<u32, super::ECC_SR1>;
#[doc = "Reader of field `RECERR0`"]
pub type RECERR0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECCERR0`"]
pub type ECCERR0_R = crate::R<bool, bool>;
#[doc = "Reader of field `MULERR0`"]
pub type MULERR0_R = crate::R<bool, bool>;
#[doc = "Reader of field `RECERR1`"]
pub type RECERR1_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECCERR1`"]
pub type ECCERR1_R = crate::R<bool, bool>;
#[doc = "Reader of field `MULERR1`"]
pub type MULERR1_R = crate::R<bool, bool>;
#[doc = "Reader of field `RECERR2`"]
pub type RECERR2_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECCERR2`"]
pub type ECCERR2_R = crate::R<bool, bool>;
#[doc = "Reader of field `MULERR2`"]
pub type MULERR2_R = crate::R<bool, bool>;
#[doc = "Reader of field `RECERR3`"]
pub type RECERR3_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECCERR3`"]
pub type ECCERR3_R = crate::R<bool, bool>;
#[doc = "Reader of field `MULERR3`"]
pub type MULERR3_R = crate::R<bool, bool>;
#[doc = "Reader of field `RECERR4`"]
pub type RECERR4_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECCERR4`"]
pub type ECCERR4_R = crate::R<bool, bool>;
#[doc = "Reader of field `MULERR4`"]
pub type MULERR4_R = crate::R<bool, bool>;
#[doc = "Reader of field `RECERR5`"]
pub type RECERR5_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECCERR5`"]
pub type ECCERR5_R = crate::R<bool, bool>;
#[doc = "Reader of field `MULERR5`"]
pub type MULERR5_R = crate::R<bool, bool>;
#[doc = "Reader of field `RECERR6`"]
pub type RECERR6_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECCERR6`"]
pub type ECCERR6_R = crate::R<bool, bool>;
#[doc = "Reader of field `MULERR6`"]
pub type MULERR6_R = crate::R<bool, bool>;
#[doc = "Reader of field `RECERR7`"]
pub type RECERR7_R = crate::R<bool, bool>;
#[doc = "Reader of field `ECCERR7`"]
pub type ECCERR7_R = crate::R<bool, bool>;
#[doc = "Reader of field `MULERR7`"]
pub type MULERR7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Recoverable Error"]
    #[inline(always)]
    pub fn recerr0(&self) -> RECERR0_R {
        RECERR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ECC Error"]
    #[inline(always)]
    pub fn eccerr0(&self) -> ECCERR0_R {
        ECCERR0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Multiple Error"]
    #[inline(always)]
    pub fn mulerr0(&self) -> MULERR0_R {
        MULERR0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Recoverable Error in the page between the 256th and the 511th bytes or the 512nd and the 1023rd bytes"]
    #[inline(always)]
    pub fn recerr1(&self) -> RECERR1_R {
        RECERR1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ECC Error in the page between the 256th and the 511th bytes or between the 512nd and the 1023rd bytes"]
    #[inline(always)]
    pub fn eccerr1(&self) -> ECCERR1_R {
        ECCERR1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Multiple Error in the page between the 256th and the 511th bytes or between the 512nd and the 1023rd bytes"]
    #[inline(always)]
    pub fn mulerr1(&self) -> MULERR1_R {
        MULERR1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Recoverable Error in the page between the 512nd and the 767th bytes or between the 1024th and the 1535th bytes"]
    #[inline(always)]
    pub fn recerr2(&self) -> RECERR2_R {
        RECERR2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ECC Error in the page between the 512nd and the 767th bytes or between the 1024th and the 1535th bytes"]
    #[inline(always)]
    pub fn eccerr2(&self) -> ECCERR2_R {
        ECCERR2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Multiple Error in the page between the 512nd and the 767th bytes or between the 1024th and the 1535th bytes"]
    #[inline(always)]
    pub fn mulerr2(&self) -> MULERR2_R {
        MULERR2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Recoverable Error in the page between the 768th and the 1023rd bytes or between the 1536th and the 2047th bytes"]
    #[inline(always)]
    pub fn recerr3(&self) -> RECERR3_R {
        RECERR3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - ECC Error in the page between the 768th and the 1023rd bytes or between the 1536th and the 2047th bytes"]
    #[inline(always)]
    pub fn eccerr3(&self) -> ECCERR3_R {
        ECCERR3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Multiple Error in the page between the 768th and the 1023rd bytes or between the 1536th and the 2047th bytes"]
    #[inline(always)]
    pub fn mulerr3(&self) -> MULERR3_R {
        MULERR3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Recoverable Error in the page between the 1024th and the 1279th bytes or between the 2048th and the 2559th bytes"]
    #[inline(always)]
    pub fn recerr4(&self) -> RECERR4_R {
        RECERR4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ECC Error in the page between the 1024th and the 1279th bytes or between the 2048th and the 2559th bytes"]
    #[inline(always)]
    pub fn eccerr4(&self) -> ECCERR4_R {
        ECCERR4_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Multiple Error in the page between the 1024th and the 1279th bytes or between the 2048th and the 2559th bytes"]
    #[inline(always)]
    pub fn mulerr4(&self) -> MULERR4_R {
        MULERR4_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Recoverable Error in the page between the 1280th and the 1535th bytes or between the 2560th and the 3071st bytes"]
    #[inline(always)]
    pub fn recerr5(&self) -> RECERR5_R {
        RECERR5_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ECC Error in the page between the 1280th and the 1535th bytes or between the 2560th and the 3071st bytes"]
    #[inline(always)]
    pub fn eccerr5(&self) -> ECCERR5_R {
        ECCERR5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Multiple Error in the page between the 1280th and the 1535th bytes or between the 2560th and the 3071st bytes"]
    #[inline(always)]
    pub fn mulerr5(&self) -> MULERR5_R {
        MULERR5_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Recoverable Error in the page between the 1536th and the 1791st bytes or between the 3072nd and the 3583rd bytes"]
    #[inline(always)]
    pub fn recerr6(&self) -> RECERR6_R {
        RECERR6_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - ECC Error in the page between the 1536th and the 1791st bytes or between the 3072nd and the 3583rd bytes"]
    #[inline(always)]
    pub fn eccerr6(&self) -> ECCERR6_R {
        ECCERR6_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Multiple Error in the page between the 1536th and the 1791st bytes or between the 3072nd and the 3583rd bytes"]
    #[inline(always)]
    pub fn mulerr6(&self) -> MULERR6_R {
        MULERR6_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Recoverable Error in the page between the 1792nd and the 2047th bytes or between the 3584th and the 4095th bytes"]
    #[inline(always)]
    pub fn recerr7(&self) -> RECERR7_R {
        RECERR7_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - ECC Error in the page between the 1792nd and the 2047th bytes or between the 3584th and the 4095th bytes"]
    #[inline(always)]
    pub fn eccerr7(&self) -> ECCERR7_R {
        ECCERR7_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Multiple Error in the page between the 1792nd and the 2047th bytes or between the 3584th and the 4095th bytes"]
    #[inline(always)]
    pub fn mulerr7(&self) -> MULERR7_R {
        MULERR7_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
