#[doc = "Reader of register ECC_PR2"]
pub type R = crate::R<u32, super::ECC_PR2>;
#[doc = "Reader of field `BITADDR`"]
pub type BITADDR_R = crate::R<u8, u8>;
#[doc = "Reader of field `WORDADDR`"]
pub type WORDADDR_R = crate::R<u16, u16>;
#[doc = "Reader of field `NPARITY`"]
pub type NPARITY_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:2 - Corrupted Bit Address in the Page between (i x 512) and ((i + 1) x 512) - 1) Bytes"]
    #[inline(always)]
    pub fn bitaddr(&self) -> BITADDR_R {
        BITADDR_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:11 - Corrupted Word Address in the Page between (i x 512) and ((i + 1) x 512) - 1) Bytes"]
    #[inline(always)]
    pub fn wordaddr(&self) -> WORDADDR_R {
        WORDADDR_R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:23 - Parity N"]
    #[inline(always)]
    pub fn nparity(&self) -> NPARITY_R {
        NPARITY_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
