#[doc = "Register `PA_DOUT` reader"]
pub type R = crate::R<PA_DOUTrs>;
#[doc = "Register `PA_DOUT` writer"]
pub type W = crate::W<PA_DOUTrs>;
#[doc = "Field `DOUT` reader - Data Out"]
pub type DoutR = crate::FieldReader<u16>;
#[doc = "Field `DOUT` writer - Data Out"]
pub type DoutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Out"]
    #[inline(always)]
    pub fn dout(&self) -> DoutR {
        DoutR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Out"]
    #[inline(always)]
    #[must_use]
    pub fn dout(&mut self) -> DoutW<PA_DOUTrs> {
        DoutW::new(self, 0)
    }
}
#[doc = "Port Data Out Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa_dout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa_dout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PA_DOUTrs;
impl crate::RegisterSpec for PA_DOUTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa_dout::R`](R) reader structure"]
impl crate::Readable for PA_DOUTrs {}
#[doc = "`write(|w| ..)` method takes [`pa_dout::W`](W) writer structure"]
impl crate::Writable for PA_DOUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PA_DOUT to value 0"]
impl crate::Resettable for PA_DOUTrs {
    const RESET_VALUE: u32 = 0;
}
