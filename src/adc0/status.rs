///Register `STATUS` reader
pub type R = crate::R<STATUSrs>;
///Field `SINGLEACT` reader - Single Channel Conversion Active
pub type SingleactR = crate::BitReader;
///Field `SCANACT` reader - Scan Conversion Active
pub type ScanactR = crate::BitReader;
///Field `SINGLEREFWARM` reader - Single Channel Reference Warmed Up
pub type SinglerefwarmR = crate::BitReader;
///Field `SCANREFWARM` reader - Scan Reference Warmed Up
pub type ScanrefwarmR = crate::BitReader;
///Programming Error Status
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PROGERR {
    ///1: `1`
    Busconf = 1,
    ///2: `10`
    Negselconf = 2,
}
impl From<PROGERR> for u8 {
    #[inline(always)]
    fn from(variant: PROGERR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PROGERR {
    type Ux = u8;
}
impl crate::IsEnum for PROGERR {}
///Field `PROGERR` reader - Programming Error Status
pub type ProgerrR = crate::FieldReader<PROGERR>;
impl ProgerrR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PROGERR> {
        match self.bits {
            1 => Some(PROGERR::Busconf),
            2 => Some(PROGERR::Negselconf),
            _ => None,
        }
    }
    ///`1`
    #[inline(always)]
    pub fn is_busconf(&self) -> bool {
        *self == PROGERR::Busconf
    }
    ///`10`
    #[inline(always)]
    pub fn is_negselconf(&self) -> bool {
        *self == PROGERR::Negselconf
    }
}
///Field `WARM` reader - ADC Warmed Up
pub type WarmR = crate::BitReader;
///Field `SINGLEDV` reader - Single Channel Data Valid
pub type SingledvR = crate::BitReader;
///Field `SCANDV` reader - Scan Data Valid
pub type ScandvR = crate::BitReader;
impl R {
    ///Bit 0 - Single Channel Conversion Active
    #[inline(always)]
    pub fn singleact(&self) -> SingleactR {
        SingleactR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Scan Conversion Active
    #[inline(always)]
    pub fn scanact(&self) -> ScanactR {
        ScanactR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Single Channel Reference Warmed Up
    #[inline(always)]
    pub fn singlerefwarm(&self) -> SinglerefwarmR {
        SinglerefwarmR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Scan Reference Warmed Up
    #[inline(always)]
    pub fn scanrefwarm(&self) -> ScanrefwarmR {
        ScanrefwarmR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:11 - Programming Error Status
    #[inline(always)]
    pub fn progerr(&self) -> ProgerrR {
        ProgerrR::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - ADC Warmed Up
    #[inline(always)]
    pub fn warm(&self) -> WarmR {
        WarmR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Single Channel Data Valid
    #[inline(always)]
    pub fn singledv(&self) -> SingledvR {
        SingledvR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Scan Data Valid
    #[inline(always)]
    pub fn scandv(&self) -> ScandvR {
        ScandvR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("singleact", &self.singleact())
            .field("scanact", &self.scanact())
            .field("singlerefwarm", &self.singlerefwarm())
            .field("scanrefwarm", &self.scanrefwarm())
            .field("progerr", &self.progerr())
            .field("warm", &self.warm())
            .field("singledv", &self.singledv())
            .field("scandv", &self.scandv())
            .finish()
    }
}
///Status Register
///
///You can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct STATUSrs;
impl crate::RegisterSpec for STATUSrs {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for STATUSrs {}
///`reset()` method sets STATUS to value 0
impl crate::Resettable for STATUSrs {
    const RESET_VALUE: u32 = 0;
}
