#[doc = "Register `CURPROG` reader"]
pub type R = crate::R<CURPROGrs>;
#[doc = "Register `CURPROG` writer"]
pub type W = crate::W<CURPROGrs>;
#[doc = "Field `RANGESEL` reader - Current Range Select"]
pub type RANGESEL_R = crate::FieldReader<RANGESEL>;
#[doc = "Current Range Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RANGESEL {
    #[doc = "0: Current range set to 0 - 1.6 uA."]
    Range0 = 0,
    #[doc = "1: Current range set to 1.6 - 4.7 uA."]
    Range1 = 1,
    #[doc = "2: Current range set to 0.5 - 16 uA."]
    Range2 = 2,
    #[doc = "3: Current range set to 2 - 64 uA."]
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
impl RANGESEL_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Current range set to 0 - 1.6 uA."]
    #[inline(always)]
    pub fn is_range0(&self) -> bool {
        *self == RANGESEL::Range0
    }
    #[doc = "Current range set to 1.6 - 4.7 uA."]
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == RANGESEL::Range1
    }
    #[doc = "Current range set to 0.5 - 16 uA."]
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == RANGESEL::Range2
    }
    #[doc = "Current range set to 2 - 64 uA."]
    #[inline(always)]
    pub fn is_range3(&self) -> bool {
        *self == RANGESEL::Range3
    }
}
#[doc = "Field `RANGESEL` writer - Current Range Select"]
pub type RANGESEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, RANGESEL>;
impl<'a, REG> RANGESEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Current range set to 0 - 1.6 uA."]
    #[inline(always)]
    pub fn range0(self) -> &'a mut crate::W<REG> {
        self.variant(RANGESEL::Range0)
    }
    #[doc = "Current range set to 1.6 - 4.7 uA."]
    #[inline(always)]
    pub fn range1(self) -> &'a mut crate::W<REG> {
        self.variant(RANGESEL::Range1)
    }
    #[doc = "Current range set to 0.5 - 16 uA."]
    #[inline(always)]
    pub fn range2(self) -> &'a mut crate::W<REG> {
        self.variant(RANGESEL::Range2)
    }
    #[doc = "Current range set to 2 - 64 uA."]
    #[inline(always)]
    pub fn range3(self) -> &'a mut crate::W<REG> {
        self.variant(RANGESEL::Range3)
    }
}
#[doc = "Field `STEPSEL` reader - Current Step Size Select"]
pub type STEPSEL_R = crate::FieldReader;
#[doc = "Field `STEPSEL` writer - Current Step Size Select"]
pub type STEPSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TUNING` reader - Tune the Current to Given Accuracy"]
pub type TUNING_R = crate::FieldReader;
#[doc = "Field `TUNING` writer - Tune the Current to Given Accuracy"]
pub type TUNING_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Current Range Select"]
    #[inline(always)]
    pub fn rangesel(&self) -> RANGESEL_R {
        RANGESEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:12 - Current Step Size Select"]
    #[inline(always)]
    pub fn stepsel(&self) -> STEPSEL_R {
        STEPSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - Tune the Current to Given Accuracy"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Current Range Select"]
    #[inline(always)]
    #[must_use]
    pub fn rangesel(&mut self) -> RANGESEL_W<CURPROGrs> {
        RANGESEL_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Current Step Size Select"]
    #[inline(always)]
    #[must_use]
    pub fn stepsel(&mut self) -> STEPSEL_W<CURPROGrs> {
        STEPSEL_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Tune the Current to Given Accuracy"]
    #[inline(always)]
    #[must_use]
    pub fn tuning(&mut self) -> TUNING_W<CURPROGrs> {
        TUNING_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Current Programming Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`curprog::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`curprog::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CURPROGrs;
impl crate::RegisterSpec for CURPROGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curprog::R`](R) reader structure"]
impl crate::Readable for CURPROGrs {}
#[doc = "`write(|w| ..)` method takes [`curprog::W`](W) writer structure"]
impl crate::Writable for CURPROGrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CURPROG to value 0x009b_0000"]
impl crate::Resettable for CURPROGrs {
    const RESET_VALUE: u32 = 0x009b_0000;
}
