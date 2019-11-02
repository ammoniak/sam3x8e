#[doc = "Reader of register ECC_PR1"]
pub type R = crate::R<u32, super::ECC_PR1>;
#[doc = "Reader of field `NPARITY`"]
pub type NPARITY_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Parity N"]
    #[inline(always)]
    pub fn nparity(&self) -> NPARITY_R {
        NPARITY_R::new((self.bits & 0xffff) as u16)
    }
}
