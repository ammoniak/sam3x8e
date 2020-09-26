#[doc = "Reader of register ECC_MD"]
pub type R = crate::R<u32, super::ECC_MD>;
#[doc = "Writer for register ECC_MD"]
pub type W = crate::W<u32, super::ECC_MD>;
#[doc = "Register ECC_MD `reset()`'s with value 0"]
impl crate::ResetValue for super::ECC_MD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ECC Page Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ECC_PAGESIZE_A {
    #[doc = "0: Main area 512 Words"]
    PS512 = 0,
    #[doc = "1: Main area 1024 Words"]
    PS1024 = 1,
    #[doc = "2: Main area 2048 Words"]
    PS2048 = 2,
    #[doc = "3: Main area 4096 Words"]
    PS4096 = 3,
}
impl From<ECC_PAGESIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: ECC_PAGESIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ECC_PAGESIZE`"]
pub type ECC_PAGESIZE_R = crate::R<u8, ECC_PAGESIZE_A>;
impl ECC_PAGESIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECC_PAGESIZE_A {
        match self.bits {
            0 => ECC_PAGESIZE_A::PS512,
            1 => ECC_PAGESIZE_A::PS1024,
            2 => ECC_PAGESIZE_A::PS2048,
            3 => ECC_PAGESIZE_A::PS4096,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PS512`"]
    #[inline(always)]
    pub fn is_ps512(&self) -> bool {
        *self == ECC_PAGESIZE_A::PS512
    }
    #[doc = "Checks if the value of the field is `PS1024`"]
    #[inline(always)]
    pub fn is_ps1024(&self) -> bool {
        *self == ECC_PAGESIZE_A::PS1024
    }
    #[doc = "Checks if the value of the field is `PS2048`"]
    #[inline(always)]
    pub fn is_ps2048(&self) -> bool {
        *self == ECC_PAGESIZE_A::PS2048
    }
    #[doc = "Checks if the value of the field is `PS4096`"]
    #[inline(always)]
    pub fn is_ps4096(&self) -> bool {
        *self == ECC_PAGESIZE_A::PS4096
    }
}
#[doc = "Write proxy for field `ECC_PAGESIZE`"]
pub struct ECC_PAGESIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_PAGESIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECC_PAGESIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Main area 512 Words"]
    #[inline(always)]
    pub fn ps512(self) -> &'a mut W {
        self.variant(ECC_PAGESIZE_A::PS512)
    }
    #[doc = "Main area 1024 Words"]
    #[inline(always)]
    pub fn ps1024(self) -> &'a mut W {
        self.variant(ECC_PAGESIZE_A::PS1024)
    }
    #[doc = "Main area 2048 Words"]
    #[inline(always)]
    pub fn ps2048(self) -> &'a mut W {
        self.variant(ECC_PAGESIZE_A::PS2048)
    }
    #[doc = "Main area 4096 Words"]
    #[inline(always)]
    pub fn ps4096(self) -> &'a mut W {
        self.variant(ECC_PAGESIZE_A::PS4096)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Type of Correction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TYPCORREC_A {
    #[doc = "0: 1 bit correction for a page of 512/1024/2048/4096 Bytes  (for 8 or 16-bit NAND Flash)"]
    CPAGE = 0,
    #[doc = "1: 1 bit correction for 256 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    C256B = 1,
    #[doc = "2: 1 bit correction for 512 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    C512B = 2,
}
impl From<TYPCORREC_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPCORREC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TYPCORREC`"]
pub type TYPCORREC_R = crate::R<u8, TYPCORREC_A>;
impl TYPCORREC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TYPCORREC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TYPCORREC_A::CPAGE),
            1 => Val(TYPCORREC_A::C256B),
            2 => Val(TYPCORREC_A::C512B),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CPAGE`"]
    #[inline(always)]
    pub fn is_cpage(&self) -> bool {
        *self == TYPCORREC_A::CPAGE
    }
    #[doc = "Checks if the value of the field is `C256B`"]
    #[inline(always)]
    pub fn is_c256b(&self) -> bool {
        *self == TYPCORREC_A::C256B
    }
    #[doc = "Checks if the value of the field is `C512B`"]
    #[inline(always)]
    pub fn is_c512b(&self) -> bool {
        *self == TYPCORREC_A::C512B
    }
}
#[doc = "Write proxy for field `TYPCORREC`"]
pub struct TYPCORREC_W<'a> {
    w: &'a mut W,
}
impl<'a> TYPCORREC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPCORREC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 bit correction for a page of 512/1024/2048/4096 Bytes (for 8 or 16-bit NAND Flash)"]
    #[inline(always)]
    pub fn cpage(self) -> &'a mut W {
        self.variant(TYPCORREC_A::CPAGE)
    }
    #[doc = "1 bit correction for 256 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    #[inline(always)]
    pub fn c256b(self) -> &'a mut W {
        self.variant(TYPCORREC_A::C256B)
    }
    #[doc = "1 bit correction for 512 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    #[inline(always)]
    pub fn c512b(self) -> &'a mut W {
        self.variant(TYPCORREC_A::C512B)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ECC Page Size"]
    #[inline(always)]
    pub fn ecc_pagesize(&self) -> ECC_PAGESIZE_R {
        ECC_PAGESIZE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Type of Correction"]
    #[inline(always)]
    pub fn typcorrec(&self) -> TYPCORREC_R {
        TYPCORREC_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ECC Page Size"]
    #[inline(always)]
    pub fn ecc_pagesize(&mut self) -> ECC_PAGESIZE_W {
        ECC_PAGESIZE_W { w: self }
    }
    #[doc = "Bits 4:5 - Type of Correction"]
    #[inline(always)]
    pub fn typcorrec(&mut self) -> TYPCORREC_W {
        TYPCORREC_W { w: self }
    }
}
