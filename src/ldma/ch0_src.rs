#[doc = "Register `CH0_SRC` reader"]
pub type R = crate::R<CH0_SRCrs>;
#[doc = "Register `CH0_SRC` writer"]
pub type W = crate::W<CH0_SRCrs>;
#[doc = "Field `SRCADDR` reader - Source Data Address"]
pub type SrcaddrR = crate::FieldReader<u32>;
#[doc = "Field `SRCADDR` writer - Source Data Address"]
pub type SrcaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Data Address"]
    #[inline(always)]
    pub fn srcaddr(&self) -> SrcaddrR {
        SrcaddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH0_SRC")
            .field("srcaddr", &self.srcaddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Data Address"]
    #[inline(always)]
    #[must_use]
    pub fn srcaddr(&mut self) -> SrcaddrW<CH0_SRCrs> {
        SrcaddrW::new(self, 0)
    }
}
#[doc = "Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH0_SRCrs;
impl crate::RegisterSpec for CH0_SRCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0_src::R`](R) reader structure"]
impl crate::Readable for CH0_SRCrs {}
#[doc = "`write(|w| ..)` method takes [`ch0_src::W`](W) writer structure"]
impl crate::Writable for CH0_SRCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH0_SRC to value 0"]
impl crate::Resettable for CH0_SRCrs {
    const RESET_VALUE: u32 = 0;
}
