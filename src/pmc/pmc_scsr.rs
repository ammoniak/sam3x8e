#[doc = "Reader of register PMC_SCSR"]
pub type R = crate::R<u32, super::PMC_SCSR>;
#[doc = "Reader of field `UOTGCLK`"]
pub type UOTGCLK_R = crate::R<bool, bool>;
#[doc = "Reader of field `PCK0`"]
pub type PCK0_R = crate::R<bool, bool>;
#[doc = "Reader of field `PCK1`"]
pub type PCK1_R = crate::R<bool, bool>;
#[doc = "Reader of field `PCK2`"]
pub type PCK2_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 5 - USB OTG Clock (48 MHz, USB_48M) Clock Status"]
    #[inline(always)]
    pub fn uotgclk(&self) -> UOTGCLK_R {
        UOTGCLK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Programmable Clock 0 Output Status"]
    #[inline(always)]
    pub fn pck0(&self) -> PCK0_R {
        PCK0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Programmable Clock 1 Output Status"]
    #[inline(always)]
    pub fn pck1(&self) -> PCK1_R {
        PCK1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Programmable Clock 2 Output Status"]
    #[inline(always)]
    pub fn pck2(&self) -> PCK2_R {
        PCK2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
