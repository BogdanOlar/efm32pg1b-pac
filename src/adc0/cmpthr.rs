///Register `CMPTHR` reader
pub type R = crate::R<CMPTHRrs>;
///Register `CMPTHR` writer
pub type W = crate::W<CMPTHRrs>;
///Field `ADLT` reader - Less Than Compare Threshold
pub type AdltR = crate::FieldReader<u16>;
///Field `ADLT` writer - Less Than Compare Threshold
pub type AdltW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `ADGT` reader - Greater Than Compare Threshold
pub type AdgtR = crate::FieldReader<u16>;
///Field `ADGT` writer - Greater Than Compare Threshold
pub type AdgtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Less Than Compare Threshold
    #[inline(always)]
    pub fn adlt(&self) -> AdltR {
        AdltR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Greater Than Compare Threshold
    #[inline(always)]
    pub fn adgt(&self) -> AdgtR {
        AdgtR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPTHR")
            .field("adlt", &self.adlt())
            .field("adgt", &self.adgt())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Less Than Compare Threshold
    #[inline(always)]
    #[must_use]
    pub fn adlt(&mut self) -> AdltW<CMPTHRrs> {
        AdltW::new(self, 0)
    }
    ///Bits 16:31 - Greater Than Compare Threshold
    #[inline(always)]
    #[must_use]
    pub fn adgt(&mut self) -> AdgtW<CMPTHRrs> {
        AdgtW::new(self, 16)
    }
}
///Compare Threshold Register
///
///You can [`read`](crate::Reg::read) this register and get [`cmpthr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpthr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CMPTHRrs;
impl crate::RegisterSpec for CMPTHRrs {
    type Ux = u32;
}
///`read()` method returns [`cmpthr::R`](R) reader structure
impl crate::Readable for CMPTHRrs {}
///`write(|w| ..)` method takes [`cmpthr::W`](W) writer structure
impl crate::Writable for CMPTHRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CMPTHR to value 0
impl crate::Resettable for CMPTHRrs {
    const RESET_VALUE: u32 = 0;
}
