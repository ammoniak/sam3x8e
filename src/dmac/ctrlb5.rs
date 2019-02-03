#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRLB5 {
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
#[doc = "Possible values of the field `SRC_DSCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_DSCRR {
    #[doc = "Source address is updated when the descriptor is fetched from the memory."]
    FETCH_FROM_MEM,
    #[doc = "Buffer Descriptor Fetch operation is disabled for the source."]
    FETCH_DISABLE,
}
impl SRC_DSCRR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SRC_DSCRR::FETCH_FROM_MEM => false,
            SRC_DSCRR::FETCH_DISABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SRC_DSCRR {
        match value {
            false => SRC_DSCRR::FETCH_FROM_MEM,
            true => SRC_DSCRR::FETCH_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `FETCH_FROM_MEM`"]
    #[inline]
    pub fn is_fetch_from_mem(&self) -> bool {
        *self == SRC_DSCRR::FETCH_FROM_MEM
    }
    #[doc = "Checks if the value of the field is `FETCH_DISABLE`"]
    #[inline]
    pub fn is_fetch_disable(&self) -> bool {
        *self == SRC_DSCRR::FETCH_DISABLE
    }
}
#[doc = "Possible values of the field `DST_DSCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DST_DSCRR {
    #[doc = "Destination address is updated when the descriptor is fetched from the memory."]
    FETCH_FROM_MEM,
    #[doc = "Buffer Descriptor Fetch operation is disabled for the destination."]
    FETCH_DISABLE,
}
impl DST_DSCRR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DST_DSCRR::FETCH_FROM_MEM => false,
            DST_DSCRR::FETCH_DISABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DST_DSCRR {
        match value {
            false => DST_DSCRR::FETCH_FROM_MEM,
            true => DST_DSCRR::FETCH_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `FETCH_FROM_MEM`"]
    #[inline]
    pub fn is_fetch_from_mem(&self) -> bool {
        *self == DST_DSCRR::FETCH_FROM_MEM
    }
    #[doc = "Checks if the value of the field is `FETCH_DISABLE`"]
    #[inline]
    pub fn is_fetch_disable(&self) -> bool {
        *self == DST_DSCRR::FETCH_DISABLE
    }
}
#[doc = "Possible values of the field `FC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCR {
    #[doc = "Memory-to-Memory Transfer DMAC is flow controller"]
    MEM2MEM_DMA_FC,
    #[doc = "Memory-to-Peripheral Transfer DMAC is flow controller"]
    MEM2PER_DMA_FC,
    #[doc = "Peripheral-to-Memory Transfer DMAC is flow controller"]
    PER2MEM_DMA_FC,
    #[doc = "Peripheral-to-Peripheral Transfer DMAC is flow controller"]
    PER2PER_DMA_FC,
}
impl FCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FCR::MEM2MEM_DMA_FC => 0,
            FCR::MEM2PER_DMA_FC => 1,
            FCR::PER2MEM_DMA_FC => 2,
            FCR::PER2PER_DMA_FC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FCR {
        match value {
            0 => FCR::MEM2MEM_DMA_FC,
            1 => FCR::MEM2PER_DMA_FC,
            2 => FCR::PER2MEM_DMA_FC,
            3 => FCR::PER2PER_DMA_FC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MEM2MEM_DMA_FC`"]
    #[inline]
    pub fn is_mem2mem_dma_fc(&self) -> bool {
        *self == FCR::MEM2MEM_DMA_FC
    }
    #[doc = "Checks if the value of the field is `MEM2PER_DMA_FC`"]
    #[inline]
    pub fn is_mem2per_dma_fc(&self) -> bool {
        *self == FCR::MEM2PER_DMA_FC
    }
    #[doc = "Checks if the value of the field is `PER2MEM_DMA_FC`"]
    #[inline]
    pub fn is_per2mem_dma_fc(&self) -> bool {
        *self == FCR::PER2MEM_DMA_FC
    }
    #[doc = "Checks if the value of the field is `PER2PER_DMA_FC`"]
    #[inline]
    pub fn is_per2per_dma_fc(&self) -> bool {
        *self == FCR::PER2PER_DMA_FC
    }
}
#[doc = "Possible values of the field `SRC_INCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_INCRR {
    #[doc = "The source address is incremented"]
    INCREMENTING,
    #[doc = "The source address is decremented"]
    DECREMENTING,
    #[doc = "The source address remains unchanged"]
    FIXED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SRC_INCRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRC_INCRR::INCREMENTING => 0,
            SRC_INCRR::DECREMENTING => 1,
            SRC_INCRR::FIXED => 2,
            SRC_INCRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRC_INCRR {
        match value {
            0 => SRC_INCRR::INCREMENTING,
            1 => SRC_INCRR::DECREMENTING,
            2 => SRC_INCRR::FIXED,
            i => SRC_INCRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INCREMENTING`"]
    #[inline]
    pub fn is_incrementing(&self) -> bool {
        *self == SRC_INCRR::INCREMENTING
    }
    #[doc = "Checks if the value of the field is `DECREMENTING`"]
    #[inline]
    pub fn is_decrementing(&self) -> bool {
        *self == SRC_INCRR::DECREMENTING
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline]
    pub fn is_fixed(&self) -> bool {
        *self == SRC_INCRR::FIXED
    }
}
#[doc = "Possible values of the field `DST_INCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DST_INCRR {
    #[doc = "The destination address is incremented"]
    INCREMENTING,
    #[doc = "The destination address is decremented"]
    DECREMENTING,
    #[doc = "The destination address remains unchanged"]
    FIXED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DST_INCRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DST_INCRR::INCREMENTING => 0,
            DST_INCRR::DECREMENTING => 1,
            DST_INCRR::FIXED => 2,
            DST_INCRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DST_INCRR {
        match value {
            0 => DST_INCRR::INCREMENTING,
            1 => DST_INCRR::DECREMENTING,
            2 => DST_INCRR::FIXED,
            i => DST_INCRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `INCREMENTING`"]
    #[inline]
    pub fn is_incrementing(&self) -> bool {
        *self == DST_INCRR::INCREMENTING
    }
    #[doc = "Checks if the value of the field is `DECREMENTING`"]
    #[inline]
    pub fn is_decrementing(&self) -> bool {
        *self == DST_INCRR::DECREMENTING
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline]
    pub fn is_fixed(&self) -> bool {
        *self == DST_INCRR::FIXED
    }
}
#[doc = r" Value of the field"]
pub struct IENR {
    bits: bool,
}
impl IENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Values that can be written to the field `SRC_DSCR`"]
pub enum SRC_DSCRW {
    #[doc = "Source address is updated when the descriptor is fetched from the memory."]
    FETCH_FROM_MEM,
    #[doc = "Buffer Descriptor Fetch operation is disabled for the source."]
    FETCH_DISABLE,
}
impl SRC_DSCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRC_DSCRW::FETCH_FROM_MEM => false,
            SRC_DSCRW::FETCH_DISABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRC_DSCRW<'a> {
    w: &'a mut W,
}
impl<'a> _SRC_DSCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRC_DSCRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Source address is updated when the descriptor is fetched from the memory."]
    #[inline]
    pub fn fetch_from_mem(self) -> &'a mut W {
        self.variant(SRC_DSCRW::FETCH_FROM_MEM)
    }
    #[doc = "Buffer Descriptor Fetch operation is disabled for the source."]
    #[inline]
    pub fn fetch_disable(self) -> &'a mut W {
        self.variant(SRC_DSCRW::FETCH_DISABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DST_DSCR`"]
pub enum DST_DSCRW {
    #[doc = "Destination address is updated when the descriptor is fetched from the memory."]
    FETCH_FROM_MEM,
    #[doc = "Buffer Descriptor Fetch operation is disabled for the destination."]
    FETCH_DISABLE,
}
impl DST_DSCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DST_DSCRW::FETCH_FROM_MEM => false,
            DST_DSCRW::FETCH_DISABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DST_DSCRW<'a> {
    w: &'a mut W,
}
impl<'a> _DST_DSCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DST_DSCRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Destination address is updated when the descriptor is fetched from the memory."]
    #[inline]
    pub fn fetch_from_mem(self) -> &'a mut W {
        self.variant(DST_DSCRW::FETCH_FROM_MEM)
    }
    #[doc = "Buffer Descriptor Fetch operation is disabled for the destination."]
    #[inline]
    pub fn fetch_disable(self) -> &'a mut W {
        self.variant(DST_DSCRW::FETCH_DISABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FC`"]
pub enum FCW {
    #[doc = "Memory-to-Memory Transfer DMAC is flow controller"]
    MEM2MEM_DMA_FC,
    #[doc = "Memory-to-Peripheral Transfer DMAC is flow controller"]
    MEM2PER_DMA_FC,
    #[doc = "Peripheral-to-Memory Transfer DMAC is flow controller"]
    PER2MEM_DMA_FC,
    #[doc = "Peripheral-to-Peripheral Transfer DMAC is flow controller"]
    PER2PER_DMA_FC,
}
impl FCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FCW::MEM2MEM_DMA_FC => 0,
            FCW::MEM2PER_DMA_FC => 1,
            FCW::PER2MEM_DMA_FC => 2,
            FCW::PER2PER_DMA_FC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCW<'a> {
    w: &'a mut W,
}
impl<'a> _FCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Memory-to-Memory Transfer DMAC is flow controller"]
    #[inline]
    pub fn mem2mem_dma_fc(self) -> &'a mut W {
        self.variant(FCW::MEM2MEM_DMA_FC)
    }
    #[doc = "Memory-to-Peripheral Transfer DMAC is flow controller"]
    #[inline]
    pub fn mem2per_dma_fc(self) -> &'a mut W {
        self.variant(FCW::MEM2PER_DMA_FC)
    }
    #[doc = "Peripheral-to-Memory Transfer DMAC is flow controller"]
    #[inline]
    pub fn per2mem_dma_fc(self) -> &'a mut W {
        self.variant(FCW::PER2MEM_DMA_FC)
    }
    #[doc = "Peripheral-to-Peripheral Transfer DMAC is flow controller"]
    #[inline]
    pub fn per2per_dma_fc(self) -> &'a mut W {
        self.variant(FCW::PER2PER_DMA_FC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRC_INCR`"]
pub enum SRC_INCRW {
    #[doc = "The source address is incremented"]
    INCREMENTING,
    #[doc = "The source address is decremented"]
    DECREMENTING,
    #[doc = "The source address remains unchanged"]
    FIXED,
}
impl SRC_INCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRC_INCRW::INCREMENTING => 0,
            SRC_INCRW::DECREMENTING => 1,
            SRC_INCRW::FIXED => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRC_INCRW<'a> {
    w: &'a mut W,
}
impl<'a> _SRC_INCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRC_INCRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The source address is incremented"]
    #[inline]
    pub fn incrementing(self) -> &'a mut W {
        self.variant(SRC_INCRW::INCREMENTING)
    }
    #[doc = "The source address is decremented"]
    #[inline]
    pub fn decrementing(self) -> &'a mut W {
        self.variant(SRC_INCRW::DECREMENTING)
    }
    #[doc = "The source address remains unchanged"]
    #[inline]
    pub fn fixed(self) -> &'a mut W {
        self.variant(SRC_INCRW::FIXED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DST_INCR`"]
pub enum DST_INCRW {
    #[doc = "The destination address is incremented"]
    INCREMENTING,
    #[doc = "The destination address is decremented"]
    DECREMENTING,
    #[doc = "The destination address remains unchanged"]
    FIXED,
}
impl DST_INCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DST_INCRW::INCREMENTING => 0,
            DST_INCRW::DECREMENTING => 1,
            DST_INCRW::FIXED => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DST_INCRW<'a> {
    w: &'a mut W,
}
impl<'a> _DST_INCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DST_INCRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The destination address is incremented"]
    #[inline]
    pub fn incrementing(self) -> &'a mut W {
        self.variant(DST_INCRW::INCREMENTING)
    }
    #[doc = "The destination address is decremented"]
    #[inline]
    pub fn decrementing(self) -> &'a mut W {
        self.variant(DST_INCRW::DECREMENTING)
    }
    #[doc = "The destination address remains unchanged"]
    #[inline]
    pub fn fixed(self) -> &'a mut W {
        self.variant(DST_INCRW::FIXED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IENW<'a> {
    w: &'a mut W,
}
impl<'a> _IENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 30;
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
    #[doc = "Bit 16 - Source Address Descriptor"]
    #[inline]
    pub fn src_dscr(&self) -> SRC_DSCRR {
        SRC_DSCRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Destination Address Descriptor"]
    #[inline]
    pub fn dst_dscr(&self) -> DST_DSCRR {
        DST_DSCRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 21:22 - Flow Control"]
    #[inline]
    pub fn fc(&self) -> FCR {
        FCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Incrementing, Decrementing or Fixed Address for the Source"]
    #[inline]
    pub fn src_incr(&self) -> SRC_INCRR {
        SRC_INCRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Incrementing, Decrementing or Fixed Address for the Destination"]
    #[inline]
    pub fn dst_incr(&self) -> DST_INCRR {
        DST_INCRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 30 - Interrupt Enable Not"]
    #[inline]
    pub fn ien(&self) -> IENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IENR { bits }
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
    #[doc = "Bit 16 - Source Address Descriptor"]
    #[inline]
    pub fn src_dscr(&mut self) -> _SRC_DSCRW {
        _SRC_DSCRW { w: self }
    }
    #[doc = "Bit 20 - Destination Address Descriptor"]
    #[inline]
    pub fn dst_dscr(&mut self) -> _DST_DSCRW {
        _DST_DSCRW { w: self }
    }
    #[doc = "Bits 21:22 - Flow Control"]
    #[inline]
    pub fn fc(&mut self) -> _FCW {
        _FCW { w: self }
    }
    #[doc = "Bits 24:25 - Incrementing, Decrementing or Fixed Address for the Source"]
    #[inline]
    pub fn src_incr(&mut self) -> _SRC_INCRW {
        _SRC_INCRW { w: self }
    }
    #[doc = "Bits 28:29 - Incrementing, Decrementing or Fixed Address for the Destination"]
    #[inline]
    pub fn dst_incr(&mut self) -> _DST_INCRW {
        _DST_INCRW { w: self }
    }
    #[doc = "Bit 30 - Interrupt Enable Not"]
    #[inline]
    pub fn ien(&mut self) -> _IENW {
        _IENW { w: self }
    }
}
