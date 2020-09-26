#[doc = "Reader of register CDR[%s]"]
pub type R = crate::R<u32, super::CDR>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Converted Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0x0fff) as u16)
    }
}
