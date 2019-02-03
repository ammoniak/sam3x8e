#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ECC_MD {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `ECC_PAGESIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECC_PAGESIZER {
    #[doc = "Main area 512 Words"]
    PS512,
    #[doc = "Main area 1024 Words"]
    PS1024,
    #[doc = "Main area 2048 Words"]
    PS2048,
    #[doc = "Main area 4096 Words"]
    PS4096,
}
impl ECC_PAGESIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ECC_PAGESIZER::PS512 => 0,
            ECC_PAGESIZER::PS1024 => 1,
            ECC_PAGESIZER::PS2048 => 2,
            ECC_PAGESIZER::PS4096 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ECC_PAGESIZER {
        match value {
            0 => ECC_PAGESIZER::PS512,
            1 => ECC_PAGESIZER::PS1024,
            2 => ECC_PAGESIZER::PS2048,
            3 => ECC_PAGESIZER::PS4096,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PS512`"]
    #[inline]
    pub fn is_ps512(&self) -> bool {
        *self == ECC_PAGESIZER::PS512
    }
    #[doc = "Checks if the value of the field is `PS1024`"]
    #[inline]
    pub fn is_ps1024(&self) -> bool {
        *self == ECC_PAGESIZER::PS1024
    }
    #[doc = "Checks if the value of the field is `PS2048`"]
    #[inline]
    pub fn is_ps2048(&self) -> bool {
        *self == ECC_PAGESIZER::PS2048
    }
    #[doc = "Checks if the value of the field is `PS4096`"]
    #[inline]
    pub fn is_ps4096(&self) -> bool {
        *self == ECC_PAGESIZER::PS4096
    }
}
#[doc = "Possible values of the field `TYPCORREC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPCORRECR {
    #[doc = "1 bit correction for a page of 512/1024/2048/4096 Bytes  (for 8 or 16-bit NAND Flash)"]
    CPAGE,
    #[doc = "1 bit correction for 256 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    C256B,
    #[doc = "1 bit correction for 512 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    C512B,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TYPCORRECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TYPCORRECR::CPAGE => 0,
            TYPCORRECR::C256B => 1,
            TYPCORRECR::C512B => 2,
            TYPCORRECR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TYPCORRECR {
        match value {
            0 => TYPCORRECR::CPAGE,
            1 => TYPCORRECR::C256B,
            2 => TYPCORRECR::C512B,
            i => TYPCORRECR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CPAGE`"]
    #[inline]
    pub fn is_cpage(&self) -> bool {
        *self == TYPCORRECR::CPAGE
    }
    #[doc = "Checks if the value of the field is `C256B`"]
    #[inline]
    pub fn is_c256b(&self) -> bool {
        *self == TYPCORRECR::C256B
    }
    #[doc = "Checks if the value of the field is `C512B`"]
    #[inline]
    pub fn is_c512b(&self) -> bool {
        *self == TYPCORRECR::C512B
    }
}
#[doc = "Values that can be written to the field `ECC_PAGESIZE`"]
pub enum ECC_PAGESIZEW {
    #[doc = "Main area 512 Words"]
    PS512,
    #[doc = "Main area 1024 Words"]
    PS1024,
    #[doc = "Main area 2048 Words"]
    PS2048,
    #[doc = "Main area 4096 Words"]
    PS4096,
}
impl ECC_PAGESIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ECC_PAGESIZEW::PS512 => 0,
            ECC_PAGESIZEW::PS1024 => 1,
            ECC_PAGESIZEW::PS2048 => 2,
            ECC_PAGESIZEW::PS4096 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECC_PAGESIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _ECC_PAGESIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECC_PAGESIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Main area 512 Words"]
    #[inline]
    pub fn ps512(self) -> &'a mut W {
        self.variant(ECC_PAGESIZEW::PS512)
    }
    #[doc = "Main area 1024 Words"]
    #[inline]
    pub fn ps1024(self) -> &'a mut W {
        self.variant(ECC_PAGESIZEW::PS1024)
    }
    #[doc = "Main area 2048 Words"]
    #[inline]
    pub fn ps2048(self) -> &'a mut W {
        self.variant(ECC_PAGESIZEW::PS2048)
    }
    #[doc = "Main area 4096 Words"]
    #[inline]
    pub fn ps4096(self) -> &'a mut W {
        self.variant(ECC_PAGESIZEW::PS4096)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TYPCORREC`"]
pub enum TYPCORRECW {
    #[doc = "1 bit correction for a page of 512/1024/2048/4096 Bytes  (for 8 or 16-bit NAND Flash)"]
    CPAGE,
    #[doc = "1 bit correction for 256 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    C256B,
    #[doc = "1 bit correction for 512 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    C512B,
}
impl TYPCORRECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TYPCORRECW::CPAGE => 0,
            TYPCORRECW::C256B => 1,
            TYPCORRECW::C512B => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TYPCORRECW<'a> {
    w: &'a mut W,
}
impl<'a> _TYPCORRECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TYPCORRECW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 bit correction for a page of 512/1024/2048/4096 Bytes (for 8 or 16-bit NAND Flash)"]
    #[inline]
    pub fn cpage(self) -> &'a mut W {
        self.variant(TYPCORRECW::CPAGE)
    }
    #[doc = "1 bit correction for 256 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    #[inline]
    pub fn c256b(self) -> &'a mut W {
        self.variant(TYPCORRECW::C256B)
    }
    #[doc = "1 bit correction for 512 Bytes of data for a page of 512/2048/4096 bytes (for 8-bit NAND Flash only)"]
    #[inline]
    pub fn c512b(self) -> &'a mut W {
        self.variant(TYPCORRECW::C512B)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - ECC Page Size"]
    #[inline]
    pub fn ecc_pagesize(&self) -> ECC_PAGESIZER {
        ECC_PAGESIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Type of Correction"]
    #[inline]
    pub fn typcorrec(&self) -> TYPCORRECR {
        TYPCORRECR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - ECC Page Size"]
    #[inline]
    pub fn ecc_pagesize(&mut self) -> _ECC_PAGESIZEW {
        _ECC_PAGESIZEW { w: self }
    }
    #[doc = "Bits 4:5 - Type of Correction"]
    #[inline]
    pub fn typcorrec(&mut self) -> _TYPCORRECW {
        _TYPCORRECW { w: self }
    }
}
