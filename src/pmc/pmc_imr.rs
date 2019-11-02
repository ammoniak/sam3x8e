#[doc = "Reader of register PMC_IMR"]
pub type R = crate::R<u32, super::PMC_IMR>;
#[doc = "Reader of field `MOSCXTS`"]
pub type MOSCXTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCKA`"]
pub type LOCKA_R = crate::R<bool, bool>;
#[doc = "Reader of field `MCKRDY`"]
pub type MCKRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCKU`"]
pub type LOCKU_R = crate::R<bool, bool>;
#[doc = "Reader of field `PCKRDY0`"]
pub type PCKRDY0_R = crate::R<bool, bool>;
#[doc = "Reader of field `PCKRDY1`"]
pub type PCKRDY1_R = crate::R<bool, bool>;
#[doc = "Reader of field `PCKRDY2`"]
pub type PCKRDY2_R = crate::R<bool, bool>;
#[doc = "Reader of field `MOSCSELS`"]
pub type MOSCSELS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MOSCRCS`"]
pub type MOSCRCS_R = crate::R<bool, bool>;
#[doc = "Reader of field `CFDEV`"]
pub type CFDEV_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Main Crystal Oscillator Status Interrupt Mask"]
    #[inline(always)]
    pub fn moscxts(&self) -> MOSCXTS_R {
        MOSCXTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PLLA Lock Interrupt Mask"]
    #[inline(always)]
    pub fn locka(&self) -> LOCKA_R {
        LOCKA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Master Clock Ready Interrupt Mask"]
    #[inline(always)]
    pub fn mckrdy(&self) -> MCKRDY_R {
        MCKRDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UTMI PLL Lock Interrupt Mask"]
    #[inline(always)]
    pub fn locku(&self) -> LOCKU_R {
        LOCKU_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Programmable Clock Ready 0 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy0(&self) -> PCKRDY0_R {
        PCKRDY0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Programmable Clock Ready 1 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy1(&self) -> PCKRDY1_R {
        PCKRDY1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Programmable Clock Ready 2 Interrupt Mask"]
    #[inline(always)]
    pub fn pckrdy2(&self) -> PCKRDY2_R {
        PCKRDY2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Main Oscillator Selection Status Interrupt Mask"]
    #[inline(always)]
    pub fn moscsels(&self) -> MOSCSELS_R {
        MOSCSELS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Main On-Chip RC Status Interrupt Mask"]
    #[inline(always)]
    pub fn moscrcs(&self) -> MOSCRCS_R {
        MOSCRCS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Clock Failure Detector Event Interrupt Mask"]
    #[inline(always)]
    pub fn cfdev(&self) -> CFDEV_R {
        CFDEV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
