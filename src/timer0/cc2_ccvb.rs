#[doc = "Register `CC2_CCVB` reader"]
pub type R = crate::R<CC2_CCVBrs>;
#[doc = "Register `CC2_CCVB` writer"]
pub type W = crate::W<CC2_CCVBrs>;
#[doc = "Field `CCVB` reader - CC Channel Value Buffer"]
pub type CCVB_R = crate::FieldReader<u16>;
#[doc = "Field `CCVB` writer - CC Channel Value Buffer"]
pub type CCVB_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CC Channel Value Buffer"]
    #[inline(always)]
    pub fn ccvb(&self) -> CCVB_R {
        CCVB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CC Channel Value Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn ccvb(&mut self) -> CCVB_W<CC2_CCVBrs> {
        CCVB_W::new(self, 0)
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
#[doc = "CC Channel Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc2_ccvb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc2_ccvb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC2_CCVBrs;
impl crate::RegisterSpec for CC2_CCVBrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc2_ccvb::R`](R) reader structure"]
impl crate::Readable for CC2_CCVBrs {}
#[doc = "`write(|w| ..)` method takes [`cc2_ccvb::W`](W) writer structure"]
impl crate::Writable for CC2_CCVBrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC2_CCVB to value 0"]
impl crate::Resettable for CC2_CCVBrs {
    const RESET_VALUE: u32 = 0;
}
