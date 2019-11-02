#[doc = "Reader of register WPSR"]
pub type R = crate::R<u32, super::WPSR>;
#[doc = "Reader of field `WP_VS`"]
pub type WP_VS_R = crate::R<u8, u8>;
#[doc = "Reader of field `WP_VSRC`"]
pub type WP_VSRC_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:3 - Write Protection Violation Status"]
    #[inline(always)]
    pub fn wp_vs(&self) -> WP_VS_R {
        WP_VS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - Write Protection Violation Source"]
    #[inline(always)]
    pub fn wp_vsrc(&self) -> WP_VSRC_R {
        WP_VSRC_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
