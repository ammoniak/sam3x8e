#[doc = "Reader of register CTRLB5"]
pub type R = crate::R<u32, super::CTRLB5>;
#[doc = "Writer for register CTRLB5"]
pub type W = crate::W<u32, super::CTRLB5>;
#[doc = "Register CTRLB5 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLB5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Source Address Descriptor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_DSCR_A {
    #[doc = "0: Source address is updated when the descriptor is fetched from the memory."]
    FETCH_FROM_MEM = 0,
    #[doc = "1: Buffer Descriptor Fetch operation is disabled for the source."]
    FETCH_DISABLE = 1,
}
impl From<SRC_DSCR_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_DSCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRC_DSCR`"]
pub type SRC_DSCR_R = crate::R<bool, SRC_DSCR_A>;
impl SRC_DSCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_DSCR_A {
        match self.bits {
            false => SRC_DSCR_A::FETCH_FROM_MEM,
            true => SRC_DSCR_A::FETCH_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `FETCH_FROM_MEM`"]
    #[inline(always)]
    pub fn is_fetch_from_mem(&self) -> bool {
        *self == SRC_DSCR_A::FETCH_FROM_MEM
    }
    #[doc = "Checks if the value of the field is `FETCH_DISABLE`"]
    #[inline(always)]
    pub fn is_fetch_disable(&self) -> bool {
        *self == SRC_DSCR_A::FETCH_DISABLE
    }
}
#[doc = "Write proxy for field `SRC_DSCR`"]
pub struct SRC_DSCR_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_DSCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_DSCR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Source address is updated when the descriptor is fetched from the memory."]
    #[inline(always)]
    pub fn fetch_from_mem(self) -> &'a mut W {
        self.variant(SRC_DSCR_A::FETCH_FROM_MEM)
    }
    #[doc = "Buffer Descriptor Fetch operation is disabled for the source."]
    #[inline(always)]
    pub fn fetch_disable(self) -> &'a mut W {
        self.variant(SRC_DSCR_A::FETCH_DISABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Destination Address Descriptor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DST_DSCR_A {
    #[doc = "0: Destination address is updated when the descriptor is fetched from the memory."]
    FETCH_FROM_MEM = 0,
    #[doc = "1: Buffer Descriptor Fetch operation is disabled for the destination."]
    FETCH_DISABLE = 1,
}
impl From<DST_DSCR_A> for bool {
    #[inline(always)]
    fn from(variant: DST_DSCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DST_DSCR`"]
pub type DST_DSCR_R = crate::R<bool, DST_DSCR_A>;
impl DST_DSCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DST_DSCR_A {
        match self.bits {
            false => DST_DSCR_A::FETCH_FROM_MEM,
            true => DST_DSCR_A::FETCH_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `FETCH_FROM_MEM`"]
    #[inline(always)]
    pub fn is_fetch_from_mem(&self) -> bool {
        *self == DST_DSCR_A::FETCH_FROM_MEM
    }
    #[doc = "Checks if the value of the field is `FETCH_DISABLE`"]
    #[inline(always)]
    pub fn is_fetch_disable(&self) -> bool {
        *self == DST_DSCR_A::FETCH_DISABLE
    }
}
#[doc = "Write proxy for field `DST_DSCR`"]
pub struct DST_DSCR_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_DSCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DST_DSCR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Destination address is updated when the descriptor is fetched from the memory."]
    #[inline(always)]
    pub fn fetch_from_mem(self) -> &'a mut W {
        self.variant(DST_DSCR_A::FETCH_FROM_MEM)
    }
    #[doc = "Buffer Descriptor Fetch operation is disabled for the destination."]
    #[inline(always)]
    pub fn fetch_disable(self) -> &'a mut W {
        self.variant(DST_DSCR_A::FETCH_DISABLE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Flow Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FC_A {
    #[doc = "0: Memory-to-Memory Transfer DMAC is flow controller"]
    MEM2MEM_DMA_FC = 0,
    #[doc = "1: Memory-to-Peripheral Transfer DMAC is flow controller"]
    MEM2PER_DMA_FC = 1,
    #[doc = "2: Peripheral-to-Memory Transfer DMAC is flow controller"]
    PER2MEM_DMA_FC = 2,
    #[doc = "3: Peripheral-to-Peripheral Transfer DMAC is flow controller"]
    PER2PER_DMA_FC = 3,
}
impl From<FC_A> for u8 {
    #[inline(always)]
    fn from(variant: FC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FC`"]
pub type FC_R = crate::R<u8, FC_A>;
impl FC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FC_A {
        match self.bits {
            0 => FC_A::MEM2MEM_DMA_FC,
            1 => FC_A::MEM2PER_DMA_FC,
            2 => FC_A::PER2MEM_DMA_FC,
            3 => FC_A::PER2PER_DMA_FC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MEM2MEM_DMA_FC`"]
    #[inline(always)]
    pub fn is_mem2mem_dma_fc(&self) -> bool {
        *self == FC_A::MEM2MEM_DMA_FC
    }
    #[doc = "Checks if the value of the field is `MEM2PER_DMA_FC`"]
    #[inline(always)]
    pub fn is_mem2per_dma_fc(&self) -> bool {
        *self == FC_A::MEM2PER_DMA_FC
    }
    #[doc = "Checks if the value of the field is `PER2MEM_DMA_FC`"]
    #[inline(always)]
    pub fn is_per2mem_dma_fc(&self) -> bool {
        *self == FC_A::PER2MEM_DMA_FC
    }
    #[doc = "Checks if the value of the field is `PER2PER_DMA_FC`"]
    #[inline(always)]
    pub fn is_per2per_dma_fc(&self) -> bool {
        *self == FC_A::PER2PER_DMA_FC
    }
}
#[doc = "Write proxy for field `FC`"]
pub struct FC_W<'a> {
    w: &'a mut W,
}
impl<'a> FC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Memory-to-Memory Transfer DMAC is flow controller"]
    #[inline(always)]
    pub fn mem2mem_dma_fc(self) -> &'a mut W {
        self.variant(FC_A::MEM2MEM_DMA_FC)
    }
    #[doc = "Memory-to-Peripheral Transfer DMAC is flow controller"]
    #[inline(always)]
    pub fn mem2per_dma_fc(self) -> &'a mut W {
        self.variant(FC_A::MEM2PER_DMA_FC)
    }
    #[doc = "Peripheral-to-Memory Transfer DMAC is flow controller"]
    #[inline(always)]
    pub fn per2mem_dma_fc(self) -> &'a mut W {
        self.variant(FC_A::PER2MEM_DMA_FC)
    }
    #[doc = "Peripheral-to-Peripheral Transfer DMAC is flow controller"]
    #[inline(always)]
    pub fn per2per_dma_fc(self) -> &'a mut W {
        self.variant(FC_A::PER2PER_DMA_FC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Incrementing, Decrementing or Fixed Address for the Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_INCR_A {
    #[doc = "0: The source address is incremented"]
    INCREMENTING = 0,
    #[doc = "1: The source address is decremented"]
    DECREMENTING = 1,
    #[doc = "2: The source address remains unchanged"]
    FIXED = 2,
}
impl From<SRC_INCR_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_INCR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRC_INCR`"]
pub type SRC_INCR_R = crate::R<u8, SRC_INCR_A>;
impl SRC_INCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRC_INCR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRC_INCR_A::INCREMENTING),
            1 => Val(SRC_INCR_A::DECREMENTING),
            2 => Val(SRC_INCR_A::FIXED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INCREMENTING`"]
    #[inline(always)]
    pub fn is_incrementing(&self) -> bool {
        *self == SRC_INCR_A::INCREMENTING
    }
    #[doc = "Checks if the value of the field is `DECREMENTING`"]
    #[inline(always)]
    pub fn is_decrementing(&self) -> bool {
        *self == SRC_INCR_A::DECREMENTING
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == SRC_INCR_A::FIXED
    }
}
#[doc = "Write proxy for field `SRC_INCR`"]
pub struct SRC_INCR_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_INCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_INCR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The source address is incremented"]
    #[inline(always)]
    pub fn incrementing(self) -> &'a mut W {
        self.variant(SRC_INCR_A::INCREMENTING)
    }
    #[doc = "The source address is decremented"]
    #[inline(always)]
    pub fn decrementing(self) -> &'a mut W {
        self.variant(SRC_INCR_A::DECREMENTING)
    }
    #[doc = "The source address remains unchanged"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(SRC_INCR_A::FIXED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Incrementing, Decrementing or Fixed Address for the Destination\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DST_INCR_A {
    #[doc = "0: The destination address is incremented"]
    INCREMENTING = 0,
    #[doc = "1: The destination address is decremented"]
    DECREMENTING = 1,
    #[doc = "2: The destination address remains unchanged"]
    FIXED = 2,
}
impl From<DST_INCR_A> for u8 {
    #[inline(always)]
    fn from(variant: DST_INCR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DST_INCR`"]
pub type DST_INCR_R = crate::R<u8, DST_INCR_A>;
impl DST_INCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DST_INCR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DST_INCR_A::INCREMENTING),
            1 => Val(DST_INCR_A::DECREMENTING),
            2 => Val(DST_INCR_A::FIXED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INCREMENTING`"]
    #[inline(always)]
    pub fn is_incrementing(&self) -> bool {
        *self == DST_INCR_A::INCREMENTING
    }
    #[doc = "Checks if the value of the field is `DECREMENTING`"]
    #[inline(always)]
    pub fn is_decrementing(&self) -> bool {
        *self == DST_INCR_A::DECREMENTING
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == DST_INCR_A::FIXED
    }
}
#[doc = "Write proxy for field `DST_INCR`"]
pub struct DST_INCR_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_INCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DST_INCR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The destination address is incremented"]
    #[inline(always)]
    pub fn incrementing(self) -> &'a mut W {
        self.variant(DST_INCR_A::INCREMENTING)
    }
    #[doc = "The destination address is decremented"]
    #[inline(always)]
    pub fn decrementing(self) -> &'a mut W {
        self.variant(DST_INCR_A::DECREMENTING)
    }
    #[doc = "The destination address remains unchanged"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(DST_INCR_A::FIXED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `IEN`"]
pub type IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IEN`"]
pub struct IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Source Address Descriptor"]
    #[inline(always)]
    pub fn src_dscr(&self) -> SRC_DSCR_R {
        SRC_DSCR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Destination Address Descriptor"]
    #[inline(always)]
    pub fn dst_dscr(&self) -> DST_DSCR_R {
        DST_DSCR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - Flow Control"]
    #[inline(always)]
    pub fn fc(&self) -> FC_R {
        FC_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Incrementing, Decrementing or Fixed Address for the Source"]
    #[inline(always)]
    pub fn src_incr(&self) -> SRC_INCR_R {
        SRC_INCR_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Incrementing, Decrementing or Fixed Address for the Destination"]
    #[inline(always)]
    pub fn dst_incr(&self) -> DST_INCR_R {
        DST_INCR_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - Interrupt Enable Not"]
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Source Address Descriptor"]
    #[inline(always)]
    pub fn src_dscr(&mut self) -> SRC_DSCR_W {
        SRC_DSCR_W { w: self }
    }
    #[doc = "Bit 20 - Destination Address Descriptor"]
    #[inline(always)]
    pub fn dst_dscr(&mut self) -> DST_DSCR_W {
        DST_DSCR_W { w: self }
    }
    #[doc = "Bits 21:22 - Flow Control"]
    #[inline(always)]
    pub fn fc(&mut self) -> FC_W {
        FC_W { w: self }
    }
    #[doc = "Bits 24:25 - Incrementing, Decrementing or Fixed Address for the Source"]
    #[inline(always)]
    pub fn src_incr(&mut self) -> SRC_INCR_W {
        SRC_INCR_W { w: self }
    }
    #[doc = "Bits 28:29 - Incrementing, Decrementing or Fixed Address for the Destination"]
    #[inline(always)]
    pub fn dst_incr(&mut self) -> DST_INCR_W {
        DST_INCR_W { w: self }
    }
    #[doc = "Bit 30 - Interrupt Enable Not"]
    #[inline(always)]
    pub fn ien(&mut self) -> IEN_W {
        IEN_W { w: self }
    }
}
