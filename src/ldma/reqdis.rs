#[doc = "Register `REQDIS` reader"]
pub type R = crate::R<REQDISrs>;
#[doc = "Register `REQDIS` writer"]
pub type W = crate::W<REQDISrs>;
#[doc = "Field `REQDIS` reader - DMA Request Disables"]
pub type REQDIS_R = crate::FieldReader;
#[doc = "Field `REQDIS` writer - DMA Request Disables"]
pub type REQDIS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DMA Request Disables"]
    #[inline(always)]
    pub fn reqdis(&self) -> REQDIS_R {
        REQDIS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA Request Disables"]
    #[inline(always)]
    #[must_use]
    pub fn reqdis(&mut self) -> REQDIS_W<REQDISrs> {
        REQDIS_W::new(self, 0)
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
#[doc = "DMA Channel Request Disable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reqdis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqdis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REQDISrs;
impl crate::RegisterSpec for REQDISrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reqdis::R`](R) reader structure"]
impl crate::Readable for REQDISrs {}
#[doc = "`write(|w| ..)` method takes [`reqdis::W`](W) writer structure"]
impl crate::Writable for REQDISrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REQDIS to value 0"]
impl crate::Resettable for REQDISrs {
    const RESET_VALUE: u32 = 0;
}
