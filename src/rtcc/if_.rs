#[doc = "Register `IF` reader"]
pub type R = crate::R<IF_SPEC>;
#[doc = "Field `OF` reader - Overflow Interrupt Flag"]
pub type OF_R = crate::BitReader;
#[doc = "Field `CC0` reader - Channel 0 Interrupt Flag"]
pub type CC0_R = crate::BitReader;
#[doc = "Field `CC1` reader - Channel 1 Interrupt Flag"]
pub type CC1_R = crate::BitReader;
#[doc = "Field `CC2` reader - Channel 2 Interrupt Flag"]
pub type CC2_R = crate::BitReader;
#[doc = "Field `OSCFAIL` reader - Oscillator Failure Interrupt Flag"]
pub type OSCFAIL_R = crate::BitReader;
#[doc = "Field `CNTTICK` reader - Main Counter Tick"]
pub type CNTTICK_R = crate::BitReader;
#[doc = "Field `MINTICK` reader - Minute Tick"]
pub type MINTICK_R = crate::BitReader;
#[doc = "Field `HOURTICK` reader - Hour Tick"]
pub type HOURTICK_R = crate::BitReader;
#[doc = "Field `DAYTICK` reader - Day Tick"]
pub type DAYTICK_R = crate::BitReader;
#[doc = "Field `DAYOWOF` reader - Day of Week Overflow"]
pub type DAYOWOF_R = crate::BitReader;
#[doc = "Field `MONTHTICK` reader - Month Tick"]
pub type MONTHTICK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 0 Interrupt Flag"]
    #[inline(always)]
    pub fn cc0(&self) -> CC0_R {
        CC0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 Interrupt Flag"]
    #[inline(always)]
    pub fn cc1(&self) -> CC1_R {
        CC1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 2 Interrupt Flag"]
    #[inline(always)]
    pub fn cc2(&self) -> CC2_R {
        CC2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Oscillator Failure Interrupt Flag"]
    #[inline(always)]
    pub fn oscfail(&self) -> OSCFAIL_R {
        OSCFAIL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Main Counter Tick"]
    #[inline(always)]
    pub fn cnttick(&self) -> CNTTICK_R {
        CNTTICK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Minute Tick"]
    #[inline(always)]
    pub fn mintick(&self) -> MINTICK_R {
        MINTICK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Hour Tick"]
    #[inline(always)]
    pub fn hourtick(&self) -> HOURTICK_R {
        HOURTICK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Day Tick"]
    #[inline(always)]
    pub fn daytick(&self) -> DAYTICK_R {
        DAYTICK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Day of Week Overflow"]
    #[inline(always)]
    pub fn dayowof(&self) -> DAYOWOF_R {
        DAYOWOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Month Tick"]
    #[inline(always)]
    pub fn monthtick(&self) -> MONTHTICK_R {
        MONTHTICK_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
            .field("of", &format_args!("{}", self.of().bit()))
            .field("cc0", &format_args!("{}", self.cc0().bit()))
            .field("cc1", &format_args!("{}", self.cc1().bit()))
            .field("cc2", &format_args!("{}", self.cc2().bit()))
            .field("oscfail", &format_args!("{}", self.oscfail().bit()))
            .field("cnttick", &format_args!("{}", self.cnttick().bit()))
            .field("mintick", &format_args!("{}", self.mintick().bit()))
            .field("hourtick", &format_args!("{}", self.hourtick().bit()))
            .field("daytick", &format_args!("{}", self.daytick().bit()))
            .field("dayowof", &format_args!("{}", self.dayowof().bit()))
            .field("monthtick", &format_args!("{}", self.monthtick().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<IF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "RTCC Interrupt Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IF_SPEC {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
