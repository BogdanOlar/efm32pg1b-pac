///Register `IF` reader
pub type R = crate::R<IFrs>;
///Field `OF` reader - Overflow Interrupt Flag
pub type OfR = crate::BitReader;
///Field `CC0` reader - Channel 0 Interrupt Flag
pub type Cc0R = crate::BitReader;
///Field `CC1` reader - Channel 1 Interrupt Flag
pub type Cc1R = crate::BitReader;
///Field `CC2` reader - Channel 2 Interrupt Flag
pub type Cc2R = crate::BitReader;
///Field `OSCFAIL` reader - Oscillator Failure Interrupt Flag
pub type OscfailR = crate::BitReader;
///Field `CNTTICK` reader - Main Counter Tick
pub type CnttickR = crate::BitReader;
///Field `MINTICK` reader - Minute Tick
pub type MintickR = crate::BitReader;
///Field `HOURTICK` reader - Hour Tick
pub type HourtickR = crate::BitReader;
///Field `DAYTICK` reader - Day Tick
pub type DaytickR = crate::BitReader;
///Field `DAYOWOF` reader - Day of Week Overflow
pub type DayowofR = crate::BitReader;
///Field `MONTHTICK` reader - Month Tick
pub type MonthtickR = crate::BitReader;
impl R {
    ///Bit 0 - Overflow Interrupt Flag
    #[inline(always)]
    pub fn of(&self) -> OfR {
        OfR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Channel 0 Interrupt Flag
    #[inline(always)]
    pub fn cc0(&self) -> Cc0R {
        Cc0R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Channel 1 Interrupt Flag
    #[inline(always)]
    pub fn cc1(&self) -> Cc1R {
        Cc1R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Channel 2 Interrupt Flag
    #[inline(always)]
    pub fn cc2(&self) -> Cc2R {
        Cc2R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Oscillator Failure Interrupt Flag
    #[inline(always)]
    pub fn oscfail(&self) -> OscfailR {
        OscfailR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Main Counter Tick
    #[inline(always)]
    pub fn cnttick(&self) -> CnttickR {
        CnttickR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Minute Tick
    #[inline(always)]
    pub fn mintick(&self) -> MintickR {
        MintickR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Hour Tick
    #[inline(always)]
    pub fn hourtick(&self) -> HourtickR {
        HourtickR::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Day Tick
    #[inline(always)]
    pub fn daytick(&self) -> DaytickR {
        DaytickR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Day of Week Overflow
    #[inline(always)]
    pub fn dayowof(&self) -> DayowofR {
        DayowofR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Month Tick
    #[inline(always)]
    pub fn monthtick(&self) -> MonthtickR {
        MonthtickR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
            .field("of", &self.of())
            .field("cc0", &self.cc0())
            .field("cc1", &self.cc1())
            .field("cc2", &self.cc2())
            .field("oscfail", &self.oscfail())
            .field("cnttick", &self.cnttick())
            .field("mintick", &self.mintick())
            .field("hourtick", &self.hourtick())
            .field("daytick", &self.daytick())
            .field("dayowof", &self.dayowof())
            .field("monthtick", &self.monthtick())
            .finish()
    }
}
///RTCC Interrupt Flags
///
///You can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IFrs;
impl crate::RegisterSpec for IFrs {
    type Ux = u32;
}
///`read()` method returns [`if_::R`](R) reader structure
impl crate::Readable for IFrs {}
///`reset()` method sets IF to value 0
impl crate::Resettable for IFrs {
    const RESET_VALUE: u32 = 0;
}
