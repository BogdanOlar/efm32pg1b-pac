#[doc = "Register `SCANMASK` reader"]
pub type R = crate::R<SCANMASKrs>;
#[doc = "Register `SCANMASK` writer"]
pub type W = crate::W<SCANMASKrs>;
#[doc = "Field `SCANINPUTEN` reader - Scan Sequence Input Mask"]
pub type SCANINPUTEN_R = crate::FieldReader<u32>;
#[doc = "Field `SCANINPUTEN` writer - Scan Sequence Input Mask"]
pub type SCANINPUTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Scan Sequence Input Mask"]
    #[inline(always)]
    pub fn scaninputen(&self) -> SCANINPUTEN_R {
        SCANINPUTEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Scan Sequence Input Mask"]
    #[inline(always)]
    #[must_use]
    pub fn scaninputen(&mut self) -> SCANINPUTEN_W<SCANMASKrs> {
        SCANINPUTEN_W::new(self, 0)
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
#[doc = "Scan Sequence Input Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scanmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scanmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCANMASKrs;
impl crate::RegisterSpec for SCANMASKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scanmask::R`](R) reader structure"]
impl crate::Readable for SCANMASKrs {}
#[doc = "`write(|w| ..)` method takes [`scanmask::W`](W) writer structure"]
impl crate::Writable for SCANMASKrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCANMASK to value 0"]
impl crate::Resettable for SCANMASKrs {
    const RESET_VALUE: u32 = 0;
}
