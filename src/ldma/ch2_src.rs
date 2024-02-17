#[doc = "Register `CH2_SRC` reader"]
pub type R = crate::R<CH2_SRCrs>;
#[doc = "Register `CH2_SRC` writer"]
pub type W = crate::W<CH2_SRCrs>;
#[doc = "Field `SRCADDR` reader - Source Data Address"]
pub type SRCADDR_R = crate::FieldReader<u32>;
#[doc = "Field `SRCADDR` writer - Source Data Address"]
pub type SRCADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Data Address"]
    #[inline(always)]
    pub fn srcaddr(&self) -> SRCADDR_R {
        SRCADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Data Address"]
    #[inline(always)]
    #[must_use]
    pub fn srcaddr(&mut self) -> SRCADDR_W<CH2_SRCrs> {
        SRCADDR_W::new(self, 0)
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
#[doc = "Channel Descriptor Source Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_src::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_src::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH2_SRCrs;
impl crate::RegisterSpec for CH2_SRCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2_src::R`](R) reader structure"]
impl crate::Readable for CH2_SRCrs {}
#[doc = "`write(|w| ..)` method takes [`ch2_src::W`](W) writer structure"]
impl crate::Writable for CH2_SRCrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH2_SRC to value 0"]
impl crate::Resettable for CH2_SRCrs {
    const RESET_VALUE: u32 = 0;
}
