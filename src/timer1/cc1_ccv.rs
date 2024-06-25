#[doc = "Register `CC1_CCV` reader"]
pub type R = crate::R<CC1_CCVrs>;
#[doc = "Register `CC1_CCV` writer"]
pub type W = crate::W<CC1_CCVrs>;
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
impl core::fmt::Debug for crate::generic::Reg<CC1_CCVrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - CC Channel Value"]
    #[inline(always)]
    #[must_use]
    pub fn ccv(&mut self) -> CcvW<CC1_CCVrs> {
        CcvW::new(self, 0)
    }
}
#[doc = "CC Channel Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_ccv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1_ccv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct CC1_CCVrs;
impl crate::RegisterSpec for CC1_CCVrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc1_ccv::R`](R) reader structure"]
impl crate::Readable for CC1_CCVrs {}
#[doc = "`write(|w| ..)` method takes [`cc1_ccv::W`](W) writer structure"]
impl crate::Writable for CC1_CCVrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC1_CCV to value 0"]
impl crate::Resettable for CC1_CCVrs {
    const RESET_VALUE: u32 = 0;
}
