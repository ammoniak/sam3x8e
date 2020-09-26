#[doc = "Reader of register PTSR"]
pub type R = crate::R<u32, super::PTSR>;
#[doc = "Reader of field `RXTEN`"]
pub type RXTEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXTEN`"]
pub type TXTEN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Receiver Transfer Enable"]
    #[inline(always)]
    pub fn rxten(&self) -> RXTEN_R {
        RXTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmitter Transfer Enable"]
    #[inline(always)]
    pub fn txten(&self) -> TXTEN_R {
        TXTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
