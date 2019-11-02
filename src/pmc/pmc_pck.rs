#[doc = "Reader of register PMC_PCK[%s]"]
pub type R = crate::R<u32, super::PMC_PCK>;
#[doc = "Writer for register PMC_PCK[%s]"]
pub type W = crate::W<u32, super::PMC_PCK>;
#[doc = "Master Clock Source Selection"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSS_A {
    #[doc = "0: Slow Clock is selected"]
    SLOW_CLK,
    #[doc = "1: Main Clock is selected"]
    MAIN_CLK,
    #[doc = "2: PLLA Clock is selected"]
    PLLA_CLK,
    #[doc = "3: UPLL Clock is selected"]
    UPLL_CLK,
    #[doc = "4: Master Clock is selected"]
    MCK,
}
impl From<CSS_A> for u8 {
    #[inline(always)]
    fn from(variant: CSS_A) -> Self {
        match variant {
            CSS_A::SLOW_CLK => 0,
            CSS_A::MAIN_CLK => 1,
            CSS_A::PLLA_CLK => 2,
            CSS_A::UPLL_CLK => 3,
            CSS_A::MCK => 4,
        }
    }
}
#[doc = "Reader of field `CSS`"]
pub type CSS_R = crate::R<u8, CSS_A>;
impl CSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CSS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CSS_A::SLOW_CLK),
            1 => Val(CSS_A::MAIN_CLK),
            2 => Val(CSS_A::PLLA_CLK),
            3 => Val(CSS_A::UPLL_CLK),
            4 => Val(CSS_A::MCK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLOW_CLK`"]
    #[inline(always)]
    pub fn is_slow_clk(&self) -> bool {
        *self == CSS_A::SLOW_CLK
    }
    #[doc = "Checks if the value of the field is `MAIN_CLK`"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == CSS_A::MAIN_CLK
    }
    #[doc = "Checks if the value of the field is `PLLA_CLK`"]
    #[inline(always)]
    pub fn is_plla_clk(&self) -> bool {
        *self == CSS_A::PLLA_CLK
    }
    #[doc = "Checks if the value of the field is `UPLL_CLK`"]
    #[inline(always)]
    pub fn is_upll_clk(&self) -> bool {
        *self == CSS_A::UPLL_CLK
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == CSS_A::MCK
    }
}
#[doc = "Write proxy for field `CSS`"]
pub struct CSS_W<'a> {
    w: &'a mut W,
}
impl<'a> CSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Slow Clock is selected"]
    #[inline(always)]
    pub fn slow_clk(self) -> &'a mut W {
        self.variant(CSS_A::SLOW_CLK)
    }
    #[doc = "Main Clock is selected"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut W {
        self.variant(CSS_A::MAIN_CLK)
    }
    #[doc = "PLLA Clock is selected"]
    #[inline(always)]
    pub fn plla_clk(self) -> &'a mut W {
        self.variant(CSS_A::PLLA_CLK)
    }
    #[doc = "UPLL Clock is selected"]
    #[inline(always)]
    pub fn upll_clk(self) -> &'a mut W {
        self.variant(CSS_A::UPLL_CLK)
    }
    #[doc = "Master Clock is selected"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(CSS_A::MCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Programmable Clock Prescaler"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRES_A {
    #[doc = "0: Selected clock"]
    CLK_1,
    #[doc = "1: Selected clock divided by 2"]
    CLK_2,
    #[doc = "2: Selected clock divided by 4"]
    CLK_4,
    #[doc = "3: Selected clock divided by 8"]
    CLK_8,
    #[doc = "4: Selected clock divided by 16"]
    CLK_16,
    #[doc = "5: Selected clock divided by 32"]
    CLK_32,
    #[doc = "6: Selected clock divided by 64"]
    CLK_64,
}
impl From<PRES_A> for u8 {
    #[inline(always)]
    fn from(variant: PRES_A) -> Self {
        match variant {
            PRES_A::CLK_1 => 0,
            PRES_A::CLK_2 => 1,
            PRES_A::CLK_4 => 2,
            PRES_A::CLK_8 => 3,
            PRES_A::CLK_16 => 4,
            PRES_A::CLK_32 => 5,
            PRES_A::CLK_64 => 6,
        }
    }
}
#[doc = "Reader of field `PRES`"]
pub type PRES_R = crate::R<u8, PRES_A>;
impl PRES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRES_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRES_A::CLK_1),
            1 => Val(PRES_A::CLK_2),
            2 => Val(PRES_A::CLK_4),
            3 => Val(PRES_A::CLK_8),
            4 => Val(PRES_A::CLK_16),
            5 => Val(PRES_A::CLK_32),
            6 => Val(PRES_A::CLK_64),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLK_1`"]
    #[inline(always)]
    pub fn is_clk_1(&self) -> bool {
        *self == PRES_A::CLK_1
    }
    #[doc = "Checks if the value of the field is `CLK_2`"]
    #[inline(always)]
    pub fn is_clk_2(&self) -> bool {
        *self == PRES_A::CLK_2
    }
    #[doc = "Checks if the value of the field is `CLK_4`"]
    #[inline(always)]
    pub fn is_clk_4(&self) -> bool {
        *self == PRES_A::CLK_4
    }
    #[doc = "Checks if the value of the field is `CLK_8`"]
    #[inline(always)]
    pub fn is_clk_8(&self) -> bool {
        *self == PRES_A::CLK_8
    }
    #[doc = "Checks if the value of the field is `CLK_16`"]
    #[inline(always)]
    pub fn is_clk_16(&self) -> bool {
        *self == PRES_A::CLK_16
    }
    #[doc = "Checks if the value of the field is `CLK_32`"]
    #[inline(always)]
    pub fn is_clk_32(&self) -> bool {
        *self == PRES_A::CLK_32
    }
    #[doc = "Checks if the value of the field is `CLK_64`"]
    #[inline(always)]
    pub fn is_clk_64(&self) -> bool {
        *self == PRES_A::CLK_64
    }
}
#[doc = "Write proxy for field `PRES`"]
pub struct PRES_W<'a> {
    w: &'a mut W,
}
impl<'a> PRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRES_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Selected clock"]
    #[inline(always)]
    pub fn clk_1(self) -> &'a mut W {
        self.variant(PRES_A::CLK_1)
    }
    #[doc = "Selected clock divided by 2"]
    #[inline(always)]
    pub fn clk_2(self) -> &'a mut W {
        self.variant(PRES_A::CLK_2)
    }
    #[doc = "Selected clock divided by 4"]
    #[inline(always)]
    pub fn clk_4(self) -> &'a mut W {
        self.variant(PRES_A::CLK_4)
    }
    #[doc = "Selected clock divided by 8"]
    #[inline(always)]
    pub fn clk_8(self) -> &'a mut W {
        self.variant(PRES_A::CLK_8)
    }
    #[doc = "Selected clock divided by 16"]
    #[inline(always)]
    pub fn clk_16(self) -> &'a mut W {
        self.variant(PRES_A::CLK_16)
    }
    #[doc = "Selected clock divided by 32"]
    #[inline(always)]
    pub fn clk_32(self) -> &'a mut W {
        self.variant(PRES_A::CLK_32)
    }
    #[doc = "Selected clock divided by 64"]
    #[inline(always)]
    pub fn clk_64(self) -> &'a mut W {
        self.variant(PRES_A::CLK_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Master Clock Source Selection"]
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Programmable Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Master Clock Source Selection"]
    #[inline(always)]
    pub fn css(&mut self) -> CSS_W {
        CSS_W { w: self }
    }
    #[doc = "Bits 4:6 - Programmable Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&mut self) -> PRES_W {
        PRES_W { w: self }
    }
}
