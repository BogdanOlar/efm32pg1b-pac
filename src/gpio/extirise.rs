#[doc = "Register `EXTIRISE` reader"]
pub type R = crate::R<EXTIRISErs>;
#[doc = "Register `EXTIRISE` writer"]
pub type W = crate::W<EXTIRISErs>;
#[doc = "Field `EXTIRISE` reader - External Interrupt N Rising Edge Trigger Enable"]
pub type ExtiriseR = crate::FieldReader<u16>;
#[doc = "Field `EXTIRISE` writer - External Interrupt N Rising Edge Trigger Enable"]
pub type ExtiriseW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - External Interrupt N Rising Edge Trigger Enable"]
    #[inline(always)]
    pub fn extirise(&self) -> ExtiriseR {
        ExtiriseR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - External Interrupt N Rising Edge Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extirise(&mut self) -> ExtiriseW<EXTIRISErs> {
        ExtiriseW::new(self, 0)
    }
}
#[doc = "External Interrupt Rising Edge Trigger Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extirise::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extirise::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTIRISErs;
impl crate::RegisterSpec for EXTIRISErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extirise::R`](R) reader structure"]
impl crate::Readable for EXTIRISErs {}
#[doc = "`write(|w| ..)` method takes [`extirise::W`](W) writer structure"]
impl crate::Writable for EXTIRISErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTIRISE to value 0"]
impl crate::Resettable for EXTIRISErs {
    const RESET_VALUE: u32 = 0;
}
