#[doc = "Reader of register CV1"]
pub type R = crate::R<u32, super::CV1>;
#[doc = "Reader of field `CV`"]
pub type CV_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter Value"]
    #[inline(always)]
    pub fn cv(&self) -> CV_R {
        CV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
