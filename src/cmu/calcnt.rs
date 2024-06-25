#[doc = "Register `CALCNT` reader"]
pub type R = crate::R<CALCNTrs>;
#[doc = "Register `CALCNT` writer"]
pub type W = crate::W<CALCNTrs>;
#[doc = "Field `CALCNT` reader - Calibration Counter"]
pub type CalcntR = crate::FieldReader<u32>;
#[doc = "Field `CALCNT` writer - Calibration Counter"]
pub type CalcntW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Calibration Counter"]
    #[inline(always)]
    pub fn calcnt(&self) -> CalcntR {
        CalcntR::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALCNT")
            .field("calcnt", &self.calcnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - Calibration Counter"]
    #[inline(always)]
    #[must_use]
    pub fn calcnt(&mut self) -> CalcntW<CALCNTrs> {
        CalcntW::new(self, 0)
    }
}
#[doc = "Calibration Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`calcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALCNTrs;
impl crate::RegisterSpec for CALCNTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calcnt::R`](R) reader structure"]
impl crate::Readable for CALCNTrs {}
#[doc = "`write(|w| ..)` method takes [`calcnt::W`](W) writer structure"]
impl crate::Writable for CALCNTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CALCNT to value 0"]
impl crate::Resettable for CALCNTrs {
    const RESET_VALUE: u32 = 0;
}
