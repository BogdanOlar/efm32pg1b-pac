#[doc = "Register `IF` reader"]
pub type R = crate::R<IFrs>;
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
#[doc = "RTCC Interrupt Flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFrs;
impl crate::RegisterSpec for IFrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IFrs {}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IFrs {
    const RESET_VALUE: u32 = 0;
}
