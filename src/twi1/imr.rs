#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `TXCOMP`"]
pub type TXCOMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXRDY`"]
pub type RXRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXRDY`"]
pub type TXRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `SVACC`"]
pub type SVACC_R = crate::R<bool, bool>;
#[doc = "Reader of field `GACC`"]
pub type GACC_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE`"]
pub type OVRE_R = crate::R<bool, bool>;
#[doc = "Reader of field `NACK`"]
pub type NACK_R = crate::R<bool, bool>;
#[doc = "Reader of field `ARBLST`"]
pub type ARBLST_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCL_WS`"]
pub type SCL_WS_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOSACC`"]
pub type EOSACC_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENDRX`"]
pub type ENDRX_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENDTX`"]
pub type ENDTX_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXBUFF`"]
pub type RXBUFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXBUFE`"]
pub type TXBUFE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Transmission Completed Interrupt Mask"]
    #[inline(always)]
    pub fn txcomp(&self) -> TXCOMP_R {
        TXCOMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Holding Register Ready Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit Holding Register Ready Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slave Access Interrupt Mask"]
    #[inline(always)]
    pub fn svacc(&self) -> SVACC_R {
        SVACC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - General Call Access Interrupt Mask"]
    #[inline(always)]
    pub fn gacc(&self) -> GACC_R {
        GACC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Not Acknowledge Interrupt Mask"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Arbitration Lost Interrupt Mask"]
    #[inline(always)]
    pub fn arblst(&self) -> ARBLST_R {
        ARBLST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Clock Wait State Interrupt Mask"]
    #[inline(always)]
    pub fn scl_ws(&self) -> SCL_WS_R {
        SCL_WS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - End Of Slave Access Interrupt Mask"]
    #[inline(always)]
    pub fn eosacc(&self) -> EOSACC_R {
        EOSACC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - End of Receive Buffer Interrupt Mask"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - End of Transmit Buffer Interrupt Mask"]
    #[inline(always)]
    pub fn endtx(&self) -> ENDTX_R {
        ENDTX_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Receive Buffer Full Interrupt Mask"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Transmit Buffer Empty Interrupt Mask"]
    #[inline(always)]
    pub fn txbufe(&self) -> TXBUFE_R {
        TXBUFE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
