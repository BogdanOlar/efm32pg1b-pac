#[doc = "Register `CC0_CCVB` reader"]
pub type R = crate::R<CC0_CCVBrs>;
#[doc = "Register `CC0_CCVB` writer"]
pub type W = crate::W<CC0_CCVBrs>;
#[doc = "Field `CCVB` reader - CC Channel Value Buffer"]
pub type CcvbR = crate::FieldReader<u16>;
#[doc = "Field `CCVB` writer - CC Channel Value Buffer"]
pub type CcvbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CC Channel Value Buffer"]
    #[inline(always)]
    pub fn ccvb(&self) -> CcvbR {
        CcvbR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CC0_CCVB")
            .field("ccvb", &self.ccvb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - CC Channel Value Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn ccvb(&mut self) -> CcvbW<CC0_CCVBrs> {
        CcvbW::new(self, 0)
    }
}
#[doc = "CC Channel Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_ccvb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc0_ccvb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC0_CCVBrs;
impl crate::RegisterSpec for CC0_CCVBrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc0_ccvb::R`](R) reader structure"]
impl crate::Readable for CC0_CCVBrs {}
#[doc = "`write(|w| ..)` method takes [`cc0_ccvb::W`](W) writer structure"]
impl crate::Writable for CC0_CCVBrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC0_CCVB to value 0"]
impl crate::Resettable for CC0_CCVBrs {
    const RESET_VALUE: u32 = 0;
}
