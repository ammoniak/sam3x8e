#[doc = "Reader of register MR"]
pub type R = crate::R<u32, super::MR>;
#[doc = "Writer for register MR"]
pub type W = crate::W<u32, super::MR>;
#[doc = "Register MR `reset()`'s with value 0"]
impl crate::ResetValue for super::MR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGEN_A {
    #[doc = "0: Hardware triggers are disabled. Starting a conversion is only possible by software."]
    DIS,
    #[doc = "1: Hardware trigger selected by TRGSEL field is enabled."]
    EN,
}
impl From<TRGEN_A> for bool {
    #[inline(always)]
    fn from(variant: TRGEN_A) -> Self {
        match variant {
            TRGEN_A::DIS => false,
            TRGEN_A::EN => true,
        }
    }
}
#[doc = "Reader of field `TRGEN`"]
pub type TRGEN_R = crate::R<bool, TRGEN_A>;
impl TRGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGEN_A {
        match self.bits {
            false => TRGEN_A::DIS,
            true => TRGEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TRGEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TRGEN_A::EN
    }
}
#[doc = "Write proxy for field `TRGEN`"]
pub struct TRGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Hardware triggers are disabled. Starting a conversion is only possible by software."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TRGEN_A::DIS)
    }
    #[doc = "Hardware trigger selected by TRGSEL field is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TRGEN_A::EN)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Trigger Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSEL_A {
    #[doc = "0: External : ADCTRG"]
    ADC_TRIG0,
    #[doc = "1: TIOA Output of the Timer Counter Channel 0"]
    ADC_TRIG1,
    #[doc = "2: TIOA Output of the Timer Counter Channel 1"]
    ADC_TRIG2,
    #[doc = "3: TIOA Output of the Timer Counter Channel 2"]
    ADC_TRIG3,
    #[doc = "4: PWM Event Line 0"]
    ADC_TRIG4,
    #[doc = "5: PWM Event Line 0"]
    ADC_TRIG5,
}
impl From<TRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL_A) -> Self {
        match variant {
            TRGSEL_A::ADC_TRIG0 => 0,
            TRGSEL_A::ADC_TRIG1 => 1,
            TRGSEL_A::ADC_TRIG2 => 2,
            TRGSEL_A::ADC_TRIG3 => 3,
            TRGSEL_A::ADC_TRIG4 => 4,
            TRGSEL_A::ADC_TRIG5 => 5,
        }
    }
}
#[doc = "Reader of field `TRGSEL`"]
pub type TRGSEL_R = crate::R<u8, TRGSEL_A>;
impl TRGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRGSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRGSEL_A::ADC_TRIG0),
            1 => Val(TRGSEL_A::ADC_TRIG1),
            2 => Val(TRGSEL_A::ADC_TRIG2),
            3 => Val(TRGSEL_A::ADC_TRIG3),
            4 => Val(TRGSEL_A::ADC_TRIG4),
            5 => Val(TRGSEL_A::ADC_TRIG5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_TRIG0`"]
    #[inline(always)]
    pub fn is_adc_trig0(&self) -> bool {
        *self == TRGSEL_A::ADC_TRIG0
    }
    #[doc = "Checks if the value of the field is `ADC_TRIG1`"]
    #[inline(always)]
    pub fn is_adc_trig1(&self) -> bool {
        *self == TRGSEL_A::ADC_TRIG1
    }
    #[doc = "Checks if the value of the field is `ADC_TRIG2`"]
    #[inline(always)]
    pub fn is_adc_trig2(&self) -> bool {
        *self == TRGSEL_A::ADC_TRIG2
    }
    #[doc = "Checks if the value of the field is `ADC_TRIG3`"]
    #[inline(always)]
    pub fn is_adc_trig3(&self) -> bool {
        *self == TRGSEL_A::ADC_TRIG3
    }
    #[doc = "Checks if the value of the field is `ADC_TRIG4`"]
    #[inline(always)]
    pub fn is_adc_trig4(&self) -> bool {
        *self == TRGSEL_A::ADC_TRIG4
    }
    #[doc = "Checks if the value of the field is `ADC_TRIG5`"]
    #[inline(always)]
    pub fn is_adc_trig5(&self) -> bool {
        *self == TRGSEL_A::ADC_TRIG5
    }
}
#[doc = "Write proxy for field `TRGSEL`"]
pub struct TRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "External : ADCTRG"]
    #[inline(always)]
    pub fn adc_trig0(self) -> &'a mut W {
        self.variant(TRGSEL_A::ADC_TRIG0)
    }
    #[doc = "TIOA Output of the Timer Counter Channel 0"]
    #[inline(always)]
    pub fn adc_trig1(self) -> &'a mut W {
        self.variant(TRGSEL_A::ADC_TRIG1)
    }
    #[doc = "TIOA Output of the Timer Counter Channel 1"]
    #[inline(always)]
    pub fn adc_trig2(self) -> &'a mut W {
        self.variant(TRGSEL_A::ADC_TRIG2)
    }
    #[doc = "TIOA Output of the Timer Counter Channel 2"]
    #[inline(always)]
    pub fn adc_trig3(self) -> &'a mut W {
        self.variant(TRGSEL_A::ADC_TRIG3)
    }
    #[doc = "PWM Event Line 0"]
    #[inline(always)]
    pub fn adc_trig4(self) -> &'a mut W {
        self.variant(TRGSEL_A::ADC_TRIG4)
    }
    #[doc = "PWM Event Line 0"]
    #[inline(always)]
    pub fn adc_trig5(self) -> &'a mut W {
        self.variant(TRGSEL_A::ADC_TRIG5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEP_A {
    #[doc = "0: Normal Mode: The ADC Core and reference voltage circuitry are kept ON between conversions"]
    NORMAL,
    #[doc = "1: Sleep Mode: The wake-up time can be modified by programming FWUP bit"]
    SLEEP,
}
impl From<SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_A) -> Self {
        match variant {
            SLEEP_A::NORMAL => false,
            SLEEP_A::SLEEP => true,
        }
    }
}
#[doc = "Reader of field `SLEEP`"]
pub type SLEEP_R = crate::R<bool, SLEEP_A>;
impl SLEEP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEP_A {
        match self.bits {
            false => SLEEP_A::NORMAL,
            true => SLEEP_A::SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SLEEP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SLEEP`"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == SLEEP_A::SLEEP
    }
}
#[doc = "Write proxy for field `SLEEP`"]
pub struct SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Mode: The ADC Core and reference voltage circuitry are kept ON between conversions"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SLEEP_A::NORMAL)
    }
    #[doc = "Sleep Mode: The wake-up time can be modified by programming FWUP bit"]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut W {
        self.variant(SLEEP_A::SLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Fast Wake Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWUP_A {
    #[doc = "0: If SLEEP is 1 then both ADC Core and reference voltage circuitry are OFF between conversions"]
    OFF,
    #[doc = "1: If SLEEP is 1 then Fast Wake-up Sleep Mode: The Voltage reference is ON between conversions and ADC Core is OFF"]
    ON,
}
impl From<FWUP_A> for bool {
    #[inline(always)]
    fn from(variant: FWUP_A) -> Self {
        match variant {
            FWUP_A::OFF => false,
            FWUP_A::ON => true,
        }
    }
}
#[doc = "Reader of field `FWUP`"]
pub type FWUP_R = crate::R<bool, FWUP_A>;
impl FWUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWUP_A {
        match self.bits {
            false => FWUP_A::OFF,
            true => FWUP_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == FWUP_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == FWUP_A::ON
    }
}
#[doc = "Write proxy for field `FWUP`"]
pub struct FWUP_W<'a> {
    w: &'a mut W,
}
impl<'a> FWUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FWUP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "If SLEEP is 1 then both ADC Core and reference voltage circuitry are OFF between conversions"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(FWUP_A::OFF)
    }
    #[doc = "If SLEEP is 1 then Fast Wake-up Sleep Mode: The Voltage reference is ON between conversions and ADC Core is OFF"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(FWUP_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Free Run Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREERUN_A {
    #[doc = "0: Normal Mode"]
    OFF,
    #[doc = "1: Free Run Mode: Never wait for any trigger."]
    ON,
}
impl From<FREERUN_A> for bool {
    #[inline(always)]
    fn from(variant: FREERUN_A) -> Self {
        match variant {
            FREERUN_A::OFF => false,
            FREERUN_A::ON => true,
        }
    }
}
#[doc = "Reader of field `FREERUN`"]
pub type FREERUN_R = crate::R<bool, FREERUN_A>;
impl FREERUN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREERUN_A {
        match self.bits {
            false => FREERUN_A::OFF,
            true => FREERUN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == FREERUN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == FREERUN_A::ON
    }
}
#[doc = "Write proxy for field `FREERUN`"]
pub struct FREERUN_W<'a> {
    w: &'a mut W,
}
impl<'a> FREERUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREERUN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(FREERUN_A::OFF)
    }
    #[doc = "Free Run Mode: Never wait for any trigger."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(FREERUN_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `PRESCAL`"]
pub type PRESCAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRESCAL`"]
pub struct PRESCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Start Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTUP_A {
    #[doc = "0: 0 periods of ADCClock"]
    SUT0,
    #[doc = "1: 8 periods of ADCClock"]
    SUT8,
    #[doc = "2: 16 periods of ADCClock"]
    SUT16,
    #[doc = "3: 24 periods of ADCClock"]
    SUT24,
    #[doc = "4: 64 periods of ADCClock"]
    SUT64,
    #[doc = "5: 80 periods of ADCClock"]
    SUT80,
    #[doc = "6: 96 periods of ADCClock"]
    SUT96,
    #[doc = "7: 112 periods of ADCClock"]
    SUT112,
    #[doc = "8: 512 periods of ADCClock"]
    SUT512,
    #[doc = "9: 576 periods of ADCClock"]
    SUT576,
    #[doc = "10: 640 periods of ADCClock"]
    SUT640,
    #[doc = "11: 704 periods of ADCClock"]
    SUT704,
    #[doc = "12: 768 periods of ADCClock"]
    SUT768,
    #[doc = "13: 832 periods of ADCClock"]
    SUT832,
    #[doc = "14: 896 periods of ADCClock"]
    SUT896,
    #[doc = "15: 960 periods of ADCClock"]
    SUT960,
}
impl From<STARTUP_A> for u8 {
    #[inline(always)]
    fn from(variant: STARTUP_A) -> Self {
        match variant {
            STARTUP_A::SUT0 => 0,
            STARTUP_A::SUT8 => 1,
            STARTUP_A::SUT16 => 2,
            STARTUP_A::SUT24 => 3,
            STARTUP_A::SUT64 => 4,
            STARTUP_A::SUT80 => 5,
            STARTUP_A::SUT96 => 6,
            STARTUP_A::SUT112 => 7,
            STARTUP_A::SUT512 => 8,
            STARTUP_A::SUT576 => 9,
            STARTUP_A::SUT640 => 10,
            STARTUP_A::SUT704 => 11,
            STARTUP_A::SUT768 => 12,
            STARTUP_A::SUT832 => 13,
            STARTUP_A::SUT896 => 14,
            STARTUP_A::SUT960 => 15,
        }
    }
}
#[doc = "Reader of field `STARTUP`"]
pub type STARTUP_R = crate::R<u8, STARTUP_A>;
impl STARTUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTUP_A {
        match self.bits {
            0 => STARTUP_A::SUT0,
            1 => STARTUP_A::SUT8,
            2 => STARTUP_A::SUT16,
            3 => STARTUP_A::SUT24,
            4 => STARTUP_A::SUT64,
            5 => STARTUP_A::SUT80,
            6 => STARTUP_A::SUT96,
            7 => STARTUP_A::SUT112,
            8 => STARTUP_A::SUT512,
            9 => STARTUP_A::SUT576,
            10 => STARTUP_A::SUT640,
            11 => STARTUP_A::SUT704,
            12 => STARTUP_A::SUT768,
            13 => STARTUP_A::SUT832,
            14 => STARTUP_A::SUT896,
            15 => STARTUP_A::SUT960,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SUT0`"]
    #[inline(always)]
    pub fn is_sut0(&self) -> bool {
        *self == STARTUP_A::SUT0
    }
    #[doc = "Checks if the value of the field is `SUT8`"]
    #[inline(always)]
    pub fn is_sut8(&self) -> bool {
        *self == STARTUP_A::SUT8
    }
    #[doc = "Checks if the value of the field is `SUT16`"]
    #[inline(always)]
    pub fn is_sut16(&self) -> bool {
        *self == STARTUP_A::SUT16
    }
    #[doc = "Checks if the value of the field is `SUT24`"]
    #[inline(always)]
    pub fn is_sut24(&self) -> bool {
        *self == STARTUP_A::SUT24
    }
    #[doc = "Checks if the value of the field is `SUT64`"]
    #[inline(always)]
    pub fn is_sut64(&self) -> bool {
        *self == STARTUP_A::SUT64
    }
    #[doc = "Checks if the value of the field is `SUT80`"]
    #[inline(always)]
    pub fn is_sut80(&self) -> bool {
        *self == STARTUP_A::SUT80
    }
    #[doc = "Checks if the value of the field is `SUT96`"]
    #[inline(always)]
    pub fn is_sut96(&self) -> bool {
        *self == STARTUP_A::SUT96
    }
    #[doc = "Checks if the value of the field is `SUT112`"]
    #[inline(always)]
    pub fn is_sut112(&self) -> bool {
        *self == STARTUP_A::SUT112
    }
    #[doc = "Checks if the value of the field is `SUT512`"]
    #[inline(always)]
    pub fn is_sut512(&self) -> bool {
        *self == STARTUP_A::SUT512
    }
    #[doc = "Checks if the value of the field is `SUT576`"]
    #[inline(always)]
    pub fn is_sut576(&self) -> bool {
        *self == STARTUP_A::SUT576
    }
    #[doc = "Checks if the value of the field is `SUT640`"]
    #[inline(always)]
    pub fn is_sut640(&self) -> bool {
        *self == STARTUP_A::SUT640
    }
    #[doc = "Checks if the value of the field is `SUT704`"]
    #[inline(always)]
    pub fn is_sut704(&self) -> bool {
        *self == STARTUP_A::SUT704
    }
    #[doc = "Checks if the value of the field is `SUT768`"]
    #[inline(always)]
    pub fn is_sut768(&self) -> bool {
        *self == STARTUP_A::SUT768
    }
    #[doc = "Checks if the value of the field is `SUT832`"]
    #[inline(always)]
    pub fn is_sut832(&self) -> bool {
        *self == STARTUP_A::SUT832
    }
    #[doc = "Checks if the value of the field is `SUT896`"]
    #[inline(always)]
    pub fn is_sut896(&self) -> bool {
        *self == STARTUP_A::SUT896
    }
    #[doc = "Checks if the value of the field is `SUT960`"]
    #[inline(always)]
    pub fn is_sut960(&self) -> bool {
        *self == STARTUP_A::SUT960
    }
}
#[doc = "Write proxy for field `STARTUP`"]
pub struct STARTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTUP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "0 periods of ADCClock"]
    #[inline(always)]
    pub fn sut0(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT0)
    }
    #[doc = "8 periods of ADCClock"]
    #[inline(always)]
    pub fn sut8(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT8)
    }
    #[doc = "16 periods of ADCClock"]
    #[inline(always)]
    pub fn sut16(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT16)
    }
    #[doc = "24 periods of ADCClock"]
    #[inline(always)]
    pub fn sut24(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT24)
    }
    #[doc = "64 periods of ADCClock"]
    #[inline(always)]
    pub fn sut64(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT64)
    }
    #[doc = "80 periods of ADCClock"]
    #[inline(always)]
    pub fn sut80(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT80)
    }
    #[doc = "96 periods of ADCClock"]
    #[inline(always)]
    pub fn sut96(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT96)
    }
    #[doc = "112 periods of ADCClock"]
    #[inline(always)]
    pub fn sut112(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT112)
    }
    #[doc = "512 periods of ADCClock"]
    #[inline(always)]
    pub fn sut512(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT512)
    }
    #[doc = "576 periods of ADCClock"]
    #[inline(always)]
    pub fn sut576(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT576)
    }
    #[doc = "640 periods of ADCClock"]
    #[inline(always)]
    pub fn sut640(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT640)
    }
    #[doc = "704 periods of ADCClock"]
    #[inline(always)]
    pub fn sut704(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT704)
    }
    #[doc = "768 periods of ADCClock"]
    #[inline(always)]
    pub fn sut768(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT768)
    }
    #[doc = "832 periods of ADCClock"]
    #[inline(always)]
    pub fn sut832(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT832)
    }
    #[doc = "896 periods of ADCClock"]
    #[inline(always)]
    pub fn sut896(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT896)
    }
    #[doc = "960 periods of ADCClock"]
    #[inline(always)]
    pub fn sut960(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT960)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Analog Settling Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETTLING_A {
    #[doc = "0: 3 periods of ADCClock"]
    AST3,
    #[doc = "1: 5 periods of ADCClock"]
    AST5,
    #[doc = "2: 9 periods of ADCClock"]
    AST9,
    #[doc = "3: 17 periods of ADCClock"]
    AST17,
}
impl From<SETTLING_A> for u8 {
    #[inline(always)]
    fn from(variant: SETTLING_A) -> Self {
        match variant {
            SETTLING_A::AST3 => 0,
            SETTLING_A::AST5 => 1,
            SETTLING_A::AST9 => 2,
            SETTLING_A::AST17 => 3,
        }
    }
}
#[doc = "Reader of field `SETTLING`"]
pub type SETTLING_R = crate::R<u8, SETTLING_A>;
impl SETTLING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SETTLING_A {
        match self.bits {
            0 => SETTLING_A::AST3,
            1 => SETTLING_A::AST5,
            2 => SETTLING_A::AST9,
            3 => SETTLING_A::AST17,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AST3`"]
    #[inline(always)]
    pub fn is_ast3(&self) -> bool {
        *self == SETTLING_A::AST3
    }
    #[doc = "Checks if the value of the field is `AST5`"]
    #[inline(always)]
    pub fn is_ast5(&self) -> bool {
        *self == SETTLING_A::AST5
    }
    #[doc = "Checks if the value of the field is `AST9`"]
    #[inline(always)]
    pub fn is_ast9(&self) -> bool {
        *self == SETTLING_A::AST9
    }
    #[doc = "Checks if the value of the field is `AST17`"]
    #[inline(always)]
    pub fn is_ast17(&self) -> bool {
        *self == SETTLING_A::AST17
    }
}
#[doc = "Write proxy for field `SETTLING`"]
pub struct SETTLING_W<'a> {
    w: &'a mut W,
}
impl<'a> SETTLING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETTLING_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "3 periods of ADCClock"]
    #[inline(always)]
    pub fn ast3(self) -> &'a mut W {
        self.variant(SETTLING_A::AST3)
    }
    #[doc = "5 periods of ADCClock"]
    #[inline(always)]
    pub fn ast5(self) -> &'a mut W {
        self.variant(SETTLING_A::AST5)
    }
    #[doc = "9 periods of ADCClock"]
    #[inline(always)]
    pub fn ast9(self) -> &'a mut W {
        self.variant(SETTLING_A::AST9)
    }
    #[doc = "17 periods of ADCClock"]
    #[inline(always)]
    pub fn ast17(self) -> &'a mut W {
        self.variant(SETTLING_A::AST17)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Analog Change\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANACH_A {
    #[doc = "0: No analog change on channel switching: DIFF0, GAIN0 and OFF0 are used for all channels"]
    NONE,
    #[doc = "1: Allows different analog settings for each channel. See ADC_CGR and ADC_COR Registers"]
    ALLOWED,
}
impl From<ANACH_A> for bool {
    #[inline(always)]
    fn from(variant: ANACH_A) -> Self {
        match variant {
            ANACH_A::NONE => false,
            ANACH_A::ALLOWED => true,
        }
    }
}
#[doc = "Reader of field `ANACH`"]
pub type ANACH_R = crate::R<bool, ANACH_A>;
impl ANACH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANACH_A {
        match self.bits {
            false => ANACH_A::NONE,
            true => ANACH_A::ALLOWED,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ANACH_A::NONE
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == ANACH_A::ALLOWED
    }
}
#[doc = "Write proxy for field `ANACH`"]
pub struct ANACH_W<'a> {
    w: &'a mut W,
}
impl<'a> ANACH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANACH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No analog change on channel switching: DIFF0, GAIN0 and OFF0 are used for all channels"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ANACH_A::NONE)
    }
    #[doc = "Allows different analog settings for each channel. See ADC_CGR and ADC_COR Registers"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(ANACH_A::ALLOWED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `TRACKTIM`"]
pub type TRACKTIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRACKTIM`"]
pub struct TRACKTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACKTIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `TRANSFER`"]
pub type TRANSFER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRANSFER`"]
pub struct TRANSFER_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSFER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Use Sequence Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USEQ_A {
    #[doc = "0: Normal Mode: The controller converts channels in a simple numeric order depending only on the channel index."]
    NUM_ORDER,
    #[doc = "1: User Sequence Mode: The sequence respects what is defined in ADC_SEQR1 and ADC_SEQR2 registers and can be used to convert several times the same channel."]
    REG_ORDER,
}
impl From<USEQ_A> for bool {
    #[inline(always)]
    fn from(variant: USEQ_A) -> Self {
        match variant {
            USEQ_A::NUM_ORDER => false,
            USEQ_A::REG_ORDER => true,
        }
    }
}
#[doc = "Reader of field `USEQ`"]
pub type USEQ_R = crate::R<bool, USEQ_A>;
impl USEQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USEQ_A {
        match self.bits {
            false => USEQ_A::NUM_ORDER,
            true => USEQ_A::REG_ORDER,
        }
    }
    #[doc = "Checks if the value of the field is `NUM_ORDER`"]
    #[inline(always)]
    pub fn is_num_order(&self) -> bool {
        *self == USEQ_A::NUM_ORDER
    }
    #[doc = "Checks if the value of the field is `REG_ORDER`"]
    #[inline(always)]
    pub fn is_reg_order(&self) -> bool {
        *self == USEQ_A::REG_ORDER
    }
}
#[doc = "Write proxy for field `USEQ`"]
pub struct USEQ_W<'a> {
    w: &'a mut W,
}
impl<'a> USEQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USEQ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal Mode: The controller converts channels in a simple numeric order depending only on the channel index."]
    #[inline(always)]
    pub fn num_order(self) -> &'a mut W {
        self.variant(USEQ_A::NUM_ORDER)
    }
    #[doc = "User Sequence Mode: The sequence respects what is defined in ADC_SEQR1 and ADC_SEQR2 registers and can be used to convert several times the same channel."]
    #[inline(always)]
    pub fn reg_order(self) -> &'a mut W {
        self.variant(USEQ_A::REG_ORDER)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    pub fn trgen(&self) -> TRGEN_R {
        TRGEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fast Wake Up"]
    #[inline(always)]
    pub fn fwup(&self) -> FWUP_R {
        FWUP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Free Run Mode"]
    #[inline(always)]
    pub fn freerun(&self) -> FREERUN_R {
        FREERUN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Prescaler Rate Selection"]
    #[inline(always)]
    pub fn prescal(&self) -> PRESCAL_R {
        PRESCAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Start Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Analog Settling Time"]
    #[inline(always)]
    pub fn settling(&self) -> SETTLING_R {
        SETTLING_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 23 - Analog Change"]
    #[inline(always)]
    pub fn anach(&self) -> ANACH_R {
        ANACH_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Tracking Time"]
    #[inline(always)]
    pub fn tracktim(&self) -> TRACKTIM_R {
        TRACKTIM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Transfer Period"]
    #[inline(always)]
    pub fn transfer(&self) -> TRANSFER_R {
        TRANSFER_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 31 - Use Sequence Enable"]
    #[inline(always)]
    pub fn useq(&self) -> USEQ_R {
        USEQ_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    pub fn trgen(&mut self) -> TRGEN_W {
        TRGEN_W { w: self }
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TRGSEL_W {
        TRGSEL_W { w: self }
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W {
        SLEEP_W { w: self }
    }
    #[doc = "Bit 6 - Fast Wake Up"]
    #[inline(always)]
    pub fn fwup(&mut self) -> FWUP_W {
        FWUP_W { w: self }
    }
    #[doc = "Bit 7 - Free Run Mode"]
    #[inline(always)]
    pub fn freerun(&mut self) -> FREERUN_W {
        FREERUN_W { w: self }
    }
    #[doc = "Bits 8:15 - Prescaler Rate Selection"]
    #[inline(always)]
    pub fn prescal(&mut self) -> PRESCAL_W {
        PRESCAL_W { w: self }
    }
    #[doc = "Bits 16:19 - Start Up Time"]
    #[inline(always)]
    pub fn startup(&mut self) -> STARTUP_W {
        STARTUP_W { w: self }
    }
    #[doc = "Bits 20:21 - Analog Settling Time"]
    #[inline(always)]
    pub fn settling(&mut self) -> SETTLING_W {
        SETTLING_W { w: self }
    }
    #[doc = "Bit 23 - Analog Change"]
    #[inline(always)]
    pub fn anach(&mut self) -> ANACH_W {
        ANACH_W { w: self }
    }
    #[doc = "Bits 24:27 - Tracking Time"]
    #[inline(always)]
    pub fn tracktim(&mut self) -> TRACKTIM_W {
        TRACKTIM_W { w: self }
    }
    #[doc = "Bits 28:29 - Transfer Period"]
    #[inline(always)]
    pub fn transfer(&mut self) -> TRANSFER_W {
        TRANSFER_W { w: self }
    }
    #[doc = "Bit 31 - Use Sequence Enable"]
    #[inline(always)]
    pub fn useq(&mut self) -> USEQ_W {
        USEQ_W { w: self }
    }
}
