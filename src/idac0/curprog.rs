///Register `CURPROG` reader
pub type R = crate::R<CURPROGrs>;
///Register `CURPROG` writer
pub type W = crate::W<CURPROGrs>;
///Current Range Select
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RANGESEL {
    ///0: Current range set to 0 - 1.6 uA.
    Range0 = 0,
    ///1: Current range set to 1.6 - 4.7 uA.
    Range1 = 1,
    ///2: Current range set to 0.5 - 16 uA.
    Range2 = 2,
    ///3: Current range set to 2 - 64 uA.
    Range3 = 3,
}
impl From<RANGESEL> for u8 {
    #[inline(always)]
    fn from(variant: RANGESEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RANGESEL {
    type Ux = u8;
}
impl crate::IsEnum for RANGESEL {}
///Field `RANGESEL` reader - Current Range Select
pub type RangeselR = crate::FieldReader<RANGESEL>;
impl RangeselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RANGESEL {
        match self.bits {
            0 => RANGESEL::Range0,
            1 => RANGESEL::Range1,
            2 => RANGESEL::Range2,
            3 => RANGESEL::Range3,
            _ => unreachable!(),
        }
    }
    ///Current range set to 0 - 1.6 uA.
    #[inline(always)]
    pub fn is_range0(&self) -> bool {
        *self == RANGESEL::Range0
    }
    ///Current range set to 1.6 - 4.7 uA.
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == RANGESEL::Range1
    }
    ///Current range set to 0.5 - 16 uA.
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == RANGESEL::Range2
    }
    ///Current range set to 2 - 64 uA.
    #[inline(always)]
    pub fn is_range3(&self) -> bool {
        *self == RANGESEL::Range3
    }
}
///Field `RANGESEL` writer - Current Range Select
pub type RangeselW<'a, REG> = crate::FieldWriter<'a, REG, 2, RANGESEL, crate::Safe>;
impl<'a, REG> RangeselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Current range set to 0 - 1.6 uA.
    #[inline(always)]
    pub fn range0(self) -> &'a mut crate::W<REG> {
        self.variant(RANGESEL::Range0)
    }
    ///Current range set to 1.6 - 4.7 uA.
    #[inline(always)]
    pub fn range1(self) -> &'a mut crate::W<REG> {
        self.variant(RANGESEL::Range1)
    }
    ///Current range set to 0.5 - 16 uA.
    #[inline(always)]
    pub fn range2(self) -> &'a mut crate::W<REG> {
        self.variant(RANGESEL::Range2)
    }
    ///Current range set to 2 - 64 uA.
    #[inline(always)]
    pub fn range3(self) -> &'a mut crate::W<REG> {
        self.variant(RANGESEL::Range3)
    }
}
///Field `STEPSEL` reader - Current Step Size Select
pub type StepselR = crate::FieldReader;
///Field `STEPSEL` writer - Current Step Size Select
pub type StepselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `TUNING` reader - Tune the Current to Given Accuracy
pub type TuningR = crate::FieldReader;
///Field `TUNING` writer - Tune the Current to Given Accuracy
pub type TuningW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:1 - Current Range Select
    #[inline(always)]
    pub fn rangesel(&self) -> RangeselR {
        RangeselR::new((self.bits & 3) as u8)
    }
    ///Bits 8:12 - Current Step Size Select
    #[inline(always)]
    pub fn stepsel(&self) -> StepselR {
        StepselR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:23 - Tune the Current to Given Accuracy
    #[inline(always)]
    pub fn tuning(&self) -> TuningR {
        TuningR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CURPROG")
            .field("rangesel", &self.rangesel())
            .field("stepsel", &self.stepsel())
            .field("tuning", &self.tuning())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Current Range Select
    #[inline(always)]
    #[must_use]
    pub fn rangesel(&mut self) -> RangeselW<CURPROGrs> {
        RangeselW::new(self, 0)
    }
    ///Bits 8:12 - Current Step Size Select
    #[inline(always)]
    #[must_use]
    pub fn stepsel(&mut self) -> StepselW<CURPROGrs> {
        StepselW::new(self, 8)
    }
    ///Bits 16:23 - Tune the Current to Given Accuracy
    #[inline(always)]
    #[must_use]
    pub fn tuning(&mut self) -> TuningW<CURPROGrs> {
        TuningW::new(self, 16)
    }
}
///Current Programming Register
///
///You can [`read`](crate::Reg::read) this register and get [`curprog::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`curprog::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CURPROGrs;
impl crate::RegisterSpec for CURPROGrs {
    type Ux = u32;
}
///`read()` method returns [`curprog::R`](R) reader structure
impl crate::Readable for CURPROGrs {}
///`write(|w| ..)` method takes [`curprog::W`](W) writer structure
impl crate::Writable for CURPROGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CURPROG to value 0x009b_0000
impl crate::Resettable for CURPROGrs {
    const RESET_VALUE: u32 = 0x009b_0000;
}
