#[doc = "Register `CC2_CCV` reader"]
pub type R = crate::R<CC2_CCVrs>;
#[doc = "Register `CC2_CCV` writer"]
pub type W = crate::W<CC2_CCVrs>;
#[doc = "Field `CCV` reader - CC Channel Value"]
pub type CcvR = crate::FieldReader<u16>;
#[doc = "Field `CCV` writer - CC Channel Value"]
pub type CcvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CC Channel Value"]
    #[inline(always)]
    pub fn ccv(&self) -> CcvR {
        CcvR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CC Channel Value"]
    #[inline(always)]
    #[must_use]
    pub fn ccv(&mut self) -> CcvW<CC2_CCVrs> {
        CcvW::new(self, 0)
    }
}
#[doc = "CC Channel Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc2_ccv::R`](R). WARN: One or more dependent resources other than the current register are immediately affected by a read operation. You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc2_ccv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC2_CCVrs;
impl crate::RegisterSpec for CC2_CCVrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc2_ccv::R`](R) reader structure"]
impl crate::Readable for CC2_CCVrs {}
#[doc = "`write(|w| ..)` method takes [`cc2_ccv::W`](W) writer structure"]
impl crate::Writable for CC2_CCVrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC2_CCV to value 0"]
impl crate::Resettable for CC2_CCVrs {
    const RESET_VALUE: u32 = 0;
}
