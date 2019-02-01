#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CIDR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct VERSIONR {
    bits: u8,
}
impl VERSIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `EPROC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPROCR {
    #[doc = "ARM946ES"]
    ARM946ES,
    #[doc = "ARM7TDMI"]
    ARM7TDMI,
    #[doc = "Cortex-M3"]
    CM3,
    #[doc = "ARM920T"]
    ARM920T,
    #[doc = "ARM926EJS"]
    ARM926EJS,
    #[doc = "Cortex-A5"]
    CA5,
    #[doc = "Cortex-M4"]
    CM4,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EPROCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EPROCR::ARM946ES => 1,
            EPROCR::ARM7TDMI => 2,
            EPROCR::CM3 => 3,
            EPROCR::ARM920T => 4,
            EPROCR::ARM926EJS => 5,
            EPROCR::CA5 => 6,
            EPROCR::CM4 => 7,
            EPROCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EPROCR {
        match value {
            1 => EPROCR::ARM946ES,
            2 => EPROCR::ARM7TDMI,
            3 => EPROCR::CM3,
            4 => EPROCR::ARM920T,
            5 => EPROCR::ARM926EJS,
            6 => EPROCR::CA5,
            7 => EPROCR::CM4,
            i => EPROCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ARM946ES`"]
    #[inline]
    pub fn is_arm946es(&self) -> bool {
        *self == EPROCR::ARM946ES
    }
    #[doc = "Checks if the value of the field is `ARM7TDMI`"]
    #[inline]
    pub fn is_arm7tdmi(&self) -> bool {
        *self == EPROCR::ARM7TDMI
    }
    #[doc = "Checks if the value of the field is `CM3`"]
    #[inline]
    pub fn is_cm3(&self) -> bool {
        *self == EPROCR::CM3
    }
    #[doc = "Checks if the value of the field is `ARM920T`"]
    #[inline]
    pub fn is_arm920t(&self) -> bool {
        *self == EPROCR::ARM920T
    }
    #[doc = "Checks if the value of the field is `ARM926EJS`"]
    #[inline]
    pub fn is_arm926ejs(&self) -> bool {
        *self == EPROCR::ARM926EJS
    }
    #[doc = "Checks if the value of the field is `CA5`"]
    #[inline]
    pub fn is_ca5(&self) -> bool {
        *self == EPROCR::CA5
    }
    #[doc = "Checks if the value of the field is `CM4`"]
    #[inline]
    pub fn is_cm4(&self) -> bool {
        *self == EPROCR::CM4
    }
}
#[doc = "Possible values of the field `NVPSIZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NVPSIZR {
    #[doc = "None"]
    NONE,
    #[doc = "8K bytes"]
    _8K,
    #[doc = "16K bytes"]
    _16K,
    #[doc = "32K bytes"]
    _32K,
    #[doc = "64K bytes"]
    _64K,
    #[doc = "128K bytes"]
    _128K,
    #[doc = "256K bytes"]
    _256K,
    #[doc = "512K bytes"]
    _512K,
    #[doc = "1024K bytes"]
    _1024K,
    #[doc = "2048K bytes"]
    _2048K,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NVPSIZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NVPSIZR::NONE => 0,
            NVPSIZR::_8K => 1,
            NVPSIZR::_16K => 2,
            NVPSIZR::_32K => 3,
            NVPSIZR::_64K => 5,
            NVPSIZR::_128K => 7,
            NVPSIZR::_256K => 9,
            NVPSIZR::_512K => 10,
            NVPSIZR::_1024K => 12,
            NVPSIZR::_2048K => 14,
            NVPSIZR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NVPSIZR {
        match value {
            0 => NVPSIZR::NONE,
            1 => NVPSIZR::_8K,
            2 => NVPSIZR::_16K,
            3 => NVPSIZR::_32K,
            5 => NVPSIZR::_64K,
            7 => NVPSIZR::_128K,
            9 => NVPSIZR::_256K,
            10 => NVPSIZR::_512K,
            12 => NVPSIZR::_1024K,
            14 => NVPSIZR::_2048K,
            i => NVPSIZR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == NVPSIZR::NONE
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline]
    pub fn is_8k(&self) -> bool {
        *self == NVPSIZR::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline]
    pub fn is_16k(&self) -> bool {
        *self == NVPSIZR::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline]
    pub fn is_32k(&self) -> bool {
        *self == NVPSIZR::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline]
    pub fn is_64k(&self) -> bool {
        *self == NVPSIZR::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline]
    pub fn is_128k(&self) -> bool {
        *self == NVPSIZR::_128K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline]
    pub fn is_256k(&self) -> bool {
        *self == NVPSIZR::_256K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline]
    pub fn is_512k(&self) -> bool {
        *self == NVPSIZR::_512K
    }
    #[doc = "Checks if the value of the field is `_1024K`"]
    #[inline]
    pub fn is_1024k(&self) -> bool {
        *self == NVPSIZR::_1024K
    }
    #[doc = "Checks if the value of the field is `_2048K`"]
    #[inline]
    pub fn is_2048k(&self) -> bool {
        *self == NVPSIZR::_2048K
    }
}
#[doc = "Possible values of the field `NVPSIZ2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NVPSIZ2R {
    #[doc = "None"]
    NONE,
    #[doc = "8K bytes"]
    _8K,
    #[doc = "16K bytes"]
    _16K,
    #[doc = "32K bytes"]
    _32K,
    #[doc = "64K bytes"]
    _64K,
    #[doc = "128K bytes"]
    _128K,
    #[doc = "256K bytes"]
    _256K,
    #[doc = "512K bytes"]
    _512K,
    #[doc = "1024K bytes"]
    _1024K,
    #[doc = "2048K bytes"]
    _2048K,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NVPSIZ2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NVPSIZ2R::NONE => 0,
            NVPSIZ2R::_8K => 1,
            NVPSIZ2R::_16K => 2,
            NVPSIZ2R::_32K => 3,
            NVPSIZ2R::_64K => 5,
            NVPSIZ2R::_128K => 7,
            NVPSIZ2R::_256K => 9,
            NVPSIZ2R::_512K => 10,
            NVPSIZ2R::_1024K => 12,
            NVPSIZ2R::_2048K => 14,
            NVPSIZ2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NVPSIZ2R {
        match value {
            0 => NVPSIZ2R::NONE,
            1 => NVPSIZ2R::_8K,
            2 => NVPSIZ2R::_16K,
            3 => NVPSIZ2R::_32K,
            5 => NVPSIZ2R::_64K,
            7 => NVPSIZ2R::_128K,
            9 => NVPSIZ2R::_256K,
            10 => NVPSIZ2R::_512K,
            12 => NVPSIZ2R::_1024K,
            14 => NVPSIZ2R::_2048K,
            i => NVPSIZ2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == NVPSIZ2R::NONE
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline]
    pub fn is_8k(&self) -> bool {
        *self == NVPSIZ2R::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline]
    pub fn is_16k(&self) -> bool {
        *self == NVPSIZ2R::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline]
    pub fn is_32k(&self) -> bool {
        *self == NVPSIZ2R::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline]
    pub fn is_64k(&self) -> bool {
        *self == NVPSIZ2R::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline]
    pub fn is_128k(&self) -> bool {
        *self == NVPSIZ2R::_128K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline]
    pub fn is_256k(&self) -> bool {
        *self == NVPSIZ2R::_256K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline]
    pub fn is_512k(&self) -> bool {
        *self == NVPSIZ2R::_512K
    }
    #[doc = "Checks if the value of the field is `_1024K`"]
    #[inline]
    pub fn is_1024k(&self) -> bool {
        *self == NVPSIZ2R::_1024K
    }
    #[doc = "Checks if the value of the field is `_2048K`"]
    #[inline]
    pub fn is_2048k(&self) -> bool {
        *self == NVPSIZ2R::_2048K
    }
}
#[doc = "Possible values of the field `SRAMSIZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMSIZR {
    #[doc = "48K bytes"]
    _48K,
    #[doc = "1K bytes"]
    _1K,
    #[doc = "2K bytes"]
    _2K,
    #[doc = "6K bytes"]
    _6K,
    #[doc = "24K bytes"]
    _24K,
    #[doc = "4K bytes"]
    _4K,
    #[doc = "80K bytes"]
    _80K,
    #[doc = "160K bytes"]
    _160K,
    #[doc = "8K bytes"]
    _8K,
    #[doc = "16K bytes"]
    _16K,
    #[doc = "32K bytes"]
    _32K,
    #[doc = "64K bytes"]
    _64K,
    #[doc = "128K bytes"]
    _128K,
    #[doc = "256K bytes"]
    _256K,
    #[doc = "96K bytes"]
    _96K,
    #[doc = "512K bytes"]
    _512K,
}
impl SRAMSIZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRAMSIZR::_48K => 0,
            SRAMSIZR::_1K => 1,
            SRAMSIZR::_2K => 2,
            SRAMSIZR::_6K => 3,
            SRAMSIZR::_24K => 4,
            SRAMSIZR::_4K => 5,
            SRAMSIZR::_80K => 6,
            SRAMSIZR::_160K => 7,
            SRAMSIZR::_8K => 8,
            SRAMSIZR::_16K => 9,
            SRAMSIZR::_32K => 10,
            SRAMSIZR::_64K => 11,
            SRAMSIZR::_128K => 12,
            SRAMSIZR::_256K => 13,
            SRAMSIZR::_96K => 14,
            SRAMSIZR::_512K => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRAMSIZR {
        match value {
            0 => SRAMSIZR::_48K,
            1 => SRAMSIZR::_1K,
            2 => SRAMSIZR::_2K,
            3 => SRAMSIZR::_6K,
            4 => SRAMSIZR::_24K,
            5 => SRAMSIZR::_4K,
            6 => SRAMSIZR::_80K,
            7 => SRAMSIZR::_160K,
            8 => SRAMSIZR::_8K,
            9 => SRAMSIZR::_16K,
            10 => SRAMSIZR::_32K,
            11 => SRAMSIZR::_64K,
            12 => SRAMSIZR::_128K,
            13 => SRAMSIZR::_256K,
            14 => SRAMSIZR::_96K,
            15 => SRAMSIZR::_512K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_48K`"]
    #[inline]
    pub fn is_48k(&self) -> bool {
        *self == SRAMSIZR::_48K
    }
    #[doc = "Checks if the value of the field is `_1K`"]
    #[inline]
    pub fn is_1k(&self) -> bool {
        *self == SRAMSIZR::_1K
    }
    #[doc = "Checks if the value of the field is `_2K`"]
    #[inline]
    pub fn is_2k(&self) -> bool {
        *self == SRAMSIZR::_2K
    }
    #[doc = "Checks if the value of the field is `_6K`"]
    #[inline]
    pub fn is_6k(&self) -> bool {
        *self == SRAMSIZR::_6K
    }
    #[doc = "Checks if the value of the field is `_24K`"]
    #[inline]
    pub fn is_24k(&self) -> bool {
        *self == SRAMSIZR::_24K
    }
    #[doc = "Checks if the value of the field is `_4K`"]
    #[inline]
    pub fn is_4k(&self) -> bool {
        *self == SRAMSIZR::_4K
    }
    #[doc = "Checks if the value of the field is `_80K`"]
    #[inline]
    pub fn is_80k(&self) -> bool {
        *self == SRAMSIZR::_80K
    }
    #[doc = "Checks if the value of the field is `_160K`"]
    #[inline]
    pub fn is_160k(&self) -> bool {
        *self == SRAMSIZR::_160K
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline]
    pub fn is_8k(&self) -> bool {
        *self == SRAMSIZR::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline]
    pub fn is_16k(&self) -> bool {
        *self == SRAMSIZR::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline]
    pub fn is_32k(&self) -> bool {
        *self == SRAMSIZR::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline]
    pub fn is_64k(&self) -> bool {
        *self == SRAMSIZR::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline]
    pub fn is_128k(&self) -> bool {
        *self == SRAMSIZR::_128K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline]
    pub fn is_256k(&self) -> bool {
        *self == SRAMSIZR::_256K
    }
    #[doc = "Checks if the value of the field is `_96K`"]
    #[inline]
    pub fn is_96k(&self) -> bool {
        *self == SRAMSIZR::_96K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline]
    pub fn is_512k(&self) -> bool {
        *self == SRAMSIZR::_512K
    }
}
#[doc = "Possible values of the field `ARCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARCHR {
    #[doc = "AT91SAM9xx Series"]
    AT91SAM9XX,
    #[doc = "AT91SAM9XExx Series"]
    AT91SAM9XEXX,
    #[doc = "AT91x34 Series"]
    AT91X34,
    #[doc = "CAP7 Series"]
    CAP7,
    #[doc = "CAP9 Series"]
    CAP9,
    #[doc = "CAP11 Series"]
    CAP11,
    #[doc = "AT91x40 Series"]
    AT91X40,
    #[doc = "AT91x42 Series"]
    AT91X42,
    #[doc = "AT91x55 Series"]
    AT91X55,
    #[doc = "AT91SAM7Axx Series"]
    AT91SAM7AXX,
    #[doc = "AT91SAM7AQxx Series"]
    AT91SAM7AQXX,
    #[doc = "AT91x63 Series"]
    AT91X63,
    #[doc = "AT91SAM7Sxx Series"]
    AT91SAM7SXX,
    #[doc = "AT91SAM7XCxx Series"]
    AT91SAM7XCXX,
    #[doc = "AT91SAM7SExx Series"]
    AT91SAM7SEXX,
    #[doc = "AT91SAM7Lxx Series"]
    AT91SAM7LXX,
    #[doc = "AT91SAM7Xxx Series"]
    AT91SAM7XXX,
    #[doc = "AT91SAM7SLxx Series"]
    AT91SAM7SLXX,
    #[doc = "SAM3UxC Series (100-pin version)"]
    SAM3UXC,
    #[doc = "SAM3UxE Series (144-pin version)"]
    SAM3UXE,
    #[doc = "SAM3AxC Series (100-pin version)"]
    SAM3AXC,
    #[doc = "SAM4AxC Series (100-pin version)"]
    SAM4AXC,
    #[doc = "SAM3XxC Series (100-pin version)"]
    SAM3XXC,
    #[doc = "SAM4XxC Series (100-pin version)"]
    SAM4XXC,
    #[doc = "SAM3XxE Series (144-pin version)"]
    SAM3XXE,
    #[doc = "SAM4XxE Series (144-pin version)"]
    SAM4XXE,
    #[doc = "SAM3XxG Series (208/217-pin version)"]
    SAM3XXG,
    #[doc = "SAM4XxG Series (208/217-pin version)"]
    SAM4XXG,
    #[doc = "SAM3SxASeries (48-pin version)"]
    SAM3SXA,
    #[doc = "SAM4SxA Series (48-pin version)"]
    SAM4SXA,
    #[doc = "SAM3SxB Series (64-pin version)"]
    SAM3SXB,
    #[doc = "SAM4SxB Series (64-pin version)"]
    SAM4SXB,
    #[doc = "SAM3SxC Series (100-pin version)"]
    SAM3SXC,
    #[doc = "SAM4SxC Series (100-pin version)"]
    SAM4SXC,
    #[doc = "AT91x92 Series"]
    AT91X92,
    #[doc = "SAM3NxA Series (48-pin version)"]
    SAM3NXA,
    #[doc = "SAM3NxB Series (64-pin version)"]
    SAM3NXB,
    #[doc = "SAM3NxC Series (100-pin version)"]
    SAM3NXC,
    #[doc = "SAM3SDxB Series (64-pin version)"]
    SAM3SDXB,
    #[doc = "SAM3SDxC Series (100-pin version)"]
    SAM3SDXC,
    #[doc = "SAM5A"]
    SAM5A,
    #[doc = "AT75Cxx Series"]
    AT75CXX,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ARCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ARCHR::AT91SAM9XX => 25,
            ARCHR::AT91SAM9XEXX => 41,
            ARCHR::AT91X34 => 52,
            ARCHR::CAP7 => 55,
            ARCHR::CAP9 => 57,
            ARCHR::CAP11 => 59,
            ARCHR::AT91X40 => 64,
            ARCHR::AT91X42 => 66,
            ARCHR::AT91X55 => 85,
            ARCHR::AT91SAM7AXX => 96,
            ARCHR::AT91SAM7AQXX => 97,
            ARCHR::AT91X63 => 99,
            ARCHR::AT91SAM7SXX => 112,
            ARCHR::AT91SAM7XCXX => 113,
            ARCHR::AT91SAM7SEXX => 114,
            ARCHR::AT91SAM7LXX => 115,
            ARCHR::AT91SAM7XXX => 117,
            ARCHR::AT91SAM7SLXX => 118,
            ARCHR::SAM3UXC => 128,
            ARCHR::SAM3UXE => 129,
            ARCHR::SAM3AXC => 131,
            ARCHR::SAM4AXC => 131,
            ARCHR::SAM3XXC => 132,
            ARCHR::SAM4XXC => 132,
            ARCHR::SAM3XXE => 133,
            ARCHR::SAM4XXE => 133,
            ARCHR::SAM3XXG => 134,
            ARCHR::SAM4XXG => 134,
            ARCHR::SAM3SXA => 136,
            ARCHR::SAM4SXA => 136,
            ARCHR::SAM3SXB => 137,
            ARCHR::SAM4SXB => 137,
            ARCHR::SAM3SXC => 138,
            ARCHR::SAM4SXC => 138,
            ARCHR::AT91X92 => 146,
            ARCHR::SAM3NXA => 147,
            ARCHR::SAM3NXB => 148,
            ARCHR::SAM3NXC => 149,
            ARCHR::SAM3SDXB => 153,
            ARCHR::SAM3SDXC => 154,
            ARCHR::SAM5A => 165,
            ARCHR::AT75CXX => 240,
            ARCHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ARCHR {
        match value {
            25 => ARCHR::AT91SAM9XX,
            41 => ARCHR::AT91SAM9XEXX,
            52 => ARCHR::AT91X34,
            55 => ARCHR::CAP7,
            57 => ARCHR::CAP9,
            59 => ARCHR::CAP11,
            64 => ARCHR::AT91X40,
            66 => ARCHR::AT91X42,
            85 => ARCHR::AT91X55,
            96 => ARCHR::AT91SAM7AXX,
            97 => ARCHR::AT91SAM7AQXX,
            99 => ARCHR::AT91X63,
            112 => ARCHR::AT91SAM7SXX,
            113 => ARCHR::AT91SAM7XCXX,
            114 => ARCHR::AT91SAM7SEXX,
            115 => ARCHR::AT91SAM7LXX,
            117 => ARCHR::AT91SAM7XXX,
            118 => ARCHR::AT91SAM7SLXX,
            128 => ARCHR::SAM3UXC,
            129 => ARCHR::SAM3UXE,
            131 => ARCHR::SAM3AXC,
            131 => ARCHR::SAM4AXC,
            132 => ARCHR::SAM3XXC,
            132 => ARCHR::SAM4XXC,
            133 => ARCHR::SAM3XXE,
            133 => ARCHR::SAM4XXE,
            134 => ARCHR::SAM3XXG,
            134 => ARCHR::SAM4XXG,
            136 => ARCHR::SAM3SXA,
            136 => ARCHR::SAM4SXA,
            137 => ARCHR::SAM3SXB,
            137 => ARCHR::SAM4SXB,
            138 => ARCHR::SAM3SXC,
            138 => ARCHR::SAM4SXC,
            146 => ARCHR::AT91X92,
            147 => ARCHR::SAM3NXA,
            148 => ARCHR::SAM3NXB,
            149 => ARCHR::SAM3NXC,
            153 => ARCHR::SAM3SDXB,
            154 => ARCHR::SAM3SDXC,
            165 => ARCHR::SAM5A,
            240 => ARCHR::AT75CXX,
            i => ARCHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AT91SAM9XX`"]
    #[inline]
    pub fn is_at91sam9xx(&self) -> bool {
        *self == ARCHR::AT91SAM9XX
    }
    #[doc = "Checks if the value of the field is `AT91SAM9XEXX`"]
    #[inline]
    pub fn is_at91sam9xexx(&self) -> bool {
        *self == ARCHR::AT91SAM9XEXX
    }
    #[doc = "Checks if the value of the field is `AT91X34`"]
    #[inline]
    pub fn is_at91x34(&self) -> bool {
        *self == ARCHR::AT91X34
    }
    #[doc = "Checks if the value of the field is `CAP7`"]
    #[inline]
    pub fn is_cap7(&self) -> bool {
        *self == ARCHR::CAP7
    }
    #[doc = "Checks if the value of the field is `CAP9`"]
    #[inline]
    pub fn is_cap9(&self) -> bool {
        *self == ARCHR::CAP9
    }
    #[doc = "Checks if the value of the field is `CAP11`"]
    #[inline]
    pub fn is_cap11(&self) -> bool {
        *self == ARCHR::CAP11
    }
    #[doc = "Checks if the value of the field is `AT91X40`"]
    #[inline]
    pub fn is_at91x40(&self) -> bool {
        *self == ARCHR::AT91X40
    }
    #[doc = "Checks if the value of the field is `AT91X42`"]
    #[inline]
    pub fn is_at91x42(&self) -> bool {
        *self == ARCHR::AT91X42
    }
    #[doc = "Checks if the value of the field is `AT91X55`"]
    #[inline]
    pub fn is_at91x55(&self) -> bool {
        *self == ARCHR::AT91X55
    }
    #[doc = "Checks if the value of the field is `AT91SAM7AXX`"]
    #[inline]
    pub fn is_at91sam7axx(&self) -> bool {
        *self == ARCHR::AT91SAM7AXX
    }
    #[doc = "Checks if the value of the field is `AT91SAM7AQXX`"]
    #[inline]
    pub fn is_at91sam7aqxx(&self) -> bool {
        *self == ARCHR::AT91SAM7AQXX
    }
    #[doc = "Checks if the value of the field is `AT91X63`"]
    #[inline]
    pub fn is_at91x63(&self) -> bool {
        *self == ARCHR::AT91X63
    }
    #[doc = "Checks if the value of the field is `AT91SAM7SXX`"]
    #[inline]
    pub fn is_at91sam7sxx(&self) -> bool {
        *self == ARCHR::AT91SAM7SXX
    }
    #[doc = "Checks if the value of the field is `AT91SAM7XCXX`"]
    #[inline]
    pub fn is_at91sam7xcxx(&self) -> bool {
        *self == ARCHR::AT91SAM7XCXX
    }
    #[doc = "Checks if the value of the field is `AT91SAM7SEXX`"]
    #[inline]
    pub fn is_at91sam7sexx(&self) -> bool {
        *self == ARCHR::AT91SAM7SEXX
    }
    #[doc = "Checks if the value of the field is `AT91SAM7LXX`"]
    #[inline]
    pub fn is_at91sam7lxx(&self) -> bool {
        *self == ARCHR::AT91SAM7LXX
    }
    #[doc = "Checks if the value of the field is `AT91SAM7XXX`"]
    #[inline]
    pub fn is_at91sam7xxx(&self) -> bool {
        *self == ARCHR::AT91SAM7XXX
    }
    #[doc = "Checks if the value of the field is `AT91SAM7SLXX`"]
    #[inline]
    pub fn is_at91sam7slxx(&self) -> bool {
        *self == ARCHR::AT91SAM7SLXX
    }
    #[doc = "Checks if the value of the field is `SAM3UXC`"]
    #[inline]
    pub fn is_sam3ux_c(&self) -> bool {
        *self == ARCHR::SAM3UXC
    }
    #[doc = "Checks if the value of the field is `SAM3UXE`"]
    #[inline]
    pub fn is_sam3ux_e(&self) -> bool {
        *self == ARCHR::SAM3UXE
    }
    #[doc = "Checks if the value of the field is `SAM3AXC`"]
    #[inline]
    pub fn is_sam3ax_c(&self) -> bool {
        *self == ARCHR::SAM3AXC
    }
    #[doc = "Checks if the value of the field is `SAM4AXC`"]
    #[inline]
    pub fn is_sam4ax_c(&self) -> bool {
        *self == ARCHR::SAM4AXC
    }
    #[doc = "Checks if the value of the field is `SAM3XXC`"]
    #[inline]
    pub fn is_sam3xx_c(&self) -> bool {
        *self == ARCHR::SAM3XXC
    }
    #[doc = "Checks if the value of the field is `SAM4XXC`"]
    #[inline]
    pub fn is_sam4xx_c(&self) -> bool {
        *self == ARCHR::SAM4XXC
    }
    #[doc = "Checks if the value of the field is `SAM3XXE`"]
    #[inline]
    pub fn is_sam3xx_e(&self) -> bool {
        *self == ARCHR::SAM3XXE
    }
    #[doc = "Checks if the value of the field is `SAM4XXE`"]
    #[inline]
    pub fn is_sam4xx_e(&self) -> bool {
        *self == ARCHR::SAM4XXE
    }
    #[doc = "Checks if the value of the field is `SAM3XXG`"]
    #[inline]
    pub fn is_sam3xx_g(&self) -> bool {
        *self == ARCHR::SAM3XXG
    }
    #[doc = "Checks if the value of the field is `SAM4XXG`"]
    #[inline]
    pub fn is_sam4xx_g(&self) -> bool {
        *self == ARCHR::SAM4XXG
    }
    #[doc = "Checks if the value of the field is `SAM3SXA`"]
    #[inline]
    pub fn is_sam3sx_a(&self) -> bool {
        *self == ARCHR::SAM3SXA
    }
    #[doc = "Checks if the value of the field is `SAM4SXA`"]
    #[inline]
    pub fn is_sam4sx_a(&self) -> bool {
        *self == ARCHR::SAM4SXA
    }
    #[doc = "Checks if the value of the field is `SAM3SXB`"]
    #[inline]
    pub fn is_sam3sx_b(&self) -> bool {
        *self == ARCHR::SAM3SXB
    }
    #[doc = "Checks if the value of the field is `SAM4SXB`"]
    #[inline]
    pub fn is_sam4sx_b(&self) -> bool {
        *self == ARCHR::SAM4SXB
    }
    #[doc = "Checks if the value of the field is `SAM3SXC`"]
    #[inline]
    pub fn is_sam3sx_c(&self) -> bool {
        *self == ARCHR::SAM3SXC
    }
    #[doc = "Checks if the value of the field is `SAM4SXC`"]
    #[inline]
    pub fn is_sam4sx_c(&self) -> bool {
        *self == ARCHR::SAM4SXC
    }
    #[doc = "Checks if the value of the field is `AT91X92`"]
    #[inline]
    pub fn is_at91x92(&self) -> bool {
        *self == ARCHR::AT91X92
    }
    #[doc = "Checks if the value of the field is `SAM3NXA`"]
    #[inline]
    pub fn is_sam3nx_a(&self) -> bool {
        *self == ARCHR::SAM3NXA
    }
    #[doc = "Checks if the value of the field is `SAM3NXB`"]
    #[inline]
    pub fn is_sam3nx_b(&self) -> bool {
        *self == ARCHR::SAM3NXB
    }
    #[doc = "Checks if the value of the field is `SAM3NXC`"]
    #[inline]
    pub fn is_sam3nx_c(&self) -> bool {
        *self == ARCHR::SAM3NXC
    }
    #[doc = "Checks if the value of the field is `SAM3SDXB`"]
    #[inline]
    pub fn is_sam3sdx_b(&self) -> bool {
        *self == ARCHR::SAM3SDXB
    }
    #[doc = "Checks if the value of the field is `SAM3SDXC`"]
    #[inline]
    pub fn is_sam3sdx_c(&self) -> bool {
        *self == ARCHR::SAM3SDXC
    }
    #[doc = "Checks if the value of the field is `SAM5A`"]
    #[inline]
    pub fn is_sam5a(&self) -> bool {
        *self == ARCHR::SAM5A
    }
    #[doc = "Checks if the value of the field is `AT75CXX`"]
    #[inline]
    pub fn is_at75cxx(&self) -> bool {
        *self == ARCHR::AT75CXX
    }
}
#[doc = "Possible values of the field `NVPTYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NVPTYPR {
    #[doc = "ROM"]
    ROM,
    #[doc = "ROMless or on-chip Flash"]
    ROMLESS,
    #[doc = "Embedded Flash Memory"]
    FLASH,
    #[doc = "ROM and Embedded Flash MemoryNVPSIZ is ROM size      NVPSIZ2 is Flash size"]
    ROM_FLASH,
    #[doc = "SRAM emulating ROM"]
    SRAM,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NVPTYPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NVPTYPR::ROM => 0,
            NVPTYPR::ROMLESS => 1,
            NVPTYPR::FLASH => 2,
            NVPTYPR::ROM_FLASH => 3,
            NVPTYPR::SRAM => 4,
            NVPTYPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NVPTYPR {
        match value {
            0 => NVPTYPR::ROM,
            1 => NVPTYPR::ROMLESS,
            2 => NVPTYPR::FLASH,
            3 => NVPTYPR::ROM_FLASH,
            4 => NVPTYPR::SRAM,
            i => NVPTYPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ROM`"]
    #[inline]
    pub fn is_rom(&self) -> bool {
        *self == NVPTYPR::ROM
    }
    #[doc = "Checks if the value of the field is `ROMLESS`"]
    #[inline]
    pub fn is_romless(&self) -> bool {
        *self == NVPTYPR::ROMLESS
    }
    #[doc = "Checks if the value of the field is `FLASH`"]
    #[inline]
    pub fn is_flash(&self) -> bool {
        *self == NVPTYPR::FLASH
    }
    #[doc = "Checks if the value of the field is `ROM_FLASH`"]
    #[inline]
    pub fn is_rom_flash(&self) -> bool {
        *self == NVPTYPR::ROM_FLASH
    }
    #[doc = "Checks if the value of the field is `SRAM`"]
    #[inline]
    pub fn is_sram(&self) -> bool {
        *self == NVPTYPR::SRAM
    }
}
#[doc = r" Value of the field"]
pub struct EXTR {
    bits: bool,
}
impl EXTR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Version of the Device"]
    #[inline]
    pub fn version(&self) -> VERSIONR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VERSIONR { bits }
    }
    #[doc = "Bits 5:7 - Embedded Processor"]
    #[inline]
    pub fn eproc(&self) -> EPROCR {
        EPROCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Nonvolatile Program Memory Size"]
    #[inline]
    pub fn nvpsiz(&self) -> NVPSIZR {
        NVPSIZR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Second Nonvolatile Program Memory Size"]
    #[inline]
    pub fn nvpsiz2(&self) -> NVPSIZ2R {
        NVPSIZ2R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Internal SRAM Size"]
    #[inline]
    pub fn sramsiz(&self) -> SRAMSIZR {
        SRAMSIZR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:27 - Architecture Identifier"]
    #[inline]
    pub fn arch(&self) -> ARCHR {
        ARCHR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:30 - Nonvolatile Program Memory Type"]
    #[inline]
    pub fn nvptyp(&self) -> NVPTYPR {
        NVPTYPR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - Extension Flag"]
    #[inline]
    pub fn ext(&self) -> EXTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EXTR { bits }
    }
}
