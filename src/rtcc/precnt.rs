#[doc = "Register `PRECNT` reader"]
pub type R = crate::R<PRECNTrs>;
#[doc = "Register `PRECNT` writer"]
pub type W = crate::W<PRECNTrs>;
#[doc = "Field `PRECNT` reader - Pre-Counter Value"]
pub type PrecntR = crate::FieldReader<u16>;
#[doc = "Field `PRECNT` writer - Pre-Counter Value"]
pub type PrecntW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Pre-Counter Value"]
    #[inline(always)]
    pub fn precnt(&self) -> PrecntR {
        PrecntR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Pre-Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn precnt(&mut self) -> PrecntW<PRECNTrs> {
        PrecntW::new(self, 0)
    }
}
#[doc = "Pre-Counter Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`precnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`precnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRECNTrs;
impl crate::RegisterSpec for PRECNTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`precnt::R`](R) reader structure"]
impl crate::Readable for PRECNTrs {}
#[doc = "`write(|w| ..)` method takes [`precnt::W`](W) writer structure"]
impl crate::Writable for PRECNTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRECNT to value 0"]
impl crate::Resettable for PRECNTrs {
    const RESET_VALUE: u32 = 0;
}
