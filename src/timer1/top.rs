#[doc = "Register `TOP` reader"]
pub type R = crate::R<TOPrs>;
#[doc = "Register `TOP` writer"]
pub type W = crate::W<TOPrs>;
#[doc = "Field `TOP` reader - Counter Top Value"]
pub type TOP_R = crate::FieldReader<u16>;
#[doc = "Field `TOP` writer - Counter Top Value"]
pub type TOP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter Top Value"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter Top Value"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TOP_W<TOPrs> {
        TOP_W::new(self, 0)
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
#[doc = "Counter Top Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`top::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`top::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOPrs;
impl crate::RegisterSpec for TOPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`top::R`](R) reader structure"]
impl crate::Readable for TOPrs {}
#[doc = "`write(|w| ..)` method takes [`top::W`](W) writer structure"]
impl crate::Writable for TOPrs {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOP to value 0xffff"]
impl crate::Resettable for TOPrs {
    const RESET_VALUE: u32 = 0xffff;
}
