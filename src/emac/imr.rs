#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `MFD`"]
pub type MFD_R = crate::R<bool, bool>;
#[doc = "Reader of field `RCOMP`"]
pub type RCOMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXUBR`"]
pub type RXUBR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXUBR`"]
pub type TXUBR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TUND`"]
pub type TUND_R = crate::R<bool, bool>;
#[doc = "Reader of field `RLE`"]
pub type RLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXERR`"]
pub type TXERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCOMP`"]
pub type TCOMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `ROVR`"]
pub type ROVR_R = crate::R<bool, bool>;
#[doc = "Reader of field `HRESP`"]
pub type HRESP_R = crate::R<bool, bool>;
#[doc = "Reader of field `PFR`"]
pub type PFR_R = crate::R<bool, bool>;
#[doc = "Reader of field `PTZ`"]
pub type PTZ_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Management Frame sent"]
    #[inline(always)]
    pub fn mfd(&self) -> MFD_R {
        MFD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    pub fn rcomp(&self) -> RCOMP_R {
        RCOMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Used Bit Read"]
    #[inline(always)]
    pub fn rxubr(&self) -> RXUBR_R {
        RXUBR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit Used Bit Read"]
    #[inline(always)]
    pub fn txubr(&self) -> TXUBR_R {
        TXUBR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Ethernet Transmit Buffer Underrun"]
    #[inline(always)]
    pub fn tund(&self) -> TUND_R {
        TUND_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded"]
    #[inline(always)]
    pub fn rle(&self) -> RLE_R {
        RLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    pub fn tcomp(&self) -> TCOMP_R {
        TCOMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&self) -> ROVR_R {
        ROVR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Hresp not OK"]
    #[inline(always)]
    pub fn hresp(&self) -> HRESP_R {
        HRESP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pause Frame Received"]
    #[inline(always)]
    pub fn pfr(&self) -> PFR_R {
        PFR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pause Time Zero"]
    #[inline(always)]
    pub fn ptz(&self) -> PTZ_R {
        PTZ_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
