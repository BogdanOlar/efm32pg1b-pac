#[doc = "Register `TOP` reader"]
pub type R = crate::R<TOPrs>;
#[doc = "Register `TOP` writer"]
pub type W = crate::W<TOPrs>;
#[doc = "Field `TOP` reader - Counter Top Value"]
pub type TopR = crate::FieldReader<u16>;
#[doc = "Field `TOP` writer - Counter Top Value"]
pub type TopW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter Top Value"]
    #[inline(always)]
    pub fn top(&self) -> TopR {
        TopR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOP").field("top", &self.top()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter Top Value"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TopW<TOPrs> {
        TopW::new(self, 0)
    }
}
#[doc = "Counter Top Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`top::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`top::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOPrs;
impl crate::RegisterSpec for TOPrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`top::R`](R) reader structure"]
impl crate::Readable for TOPrs {}
#[doc = "`write(|w| ..)` method takes [`top::W`](W) writer structure"]
impl crate::Writable for TOPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOP to value 0xffff"]
impl crate::Resettable for TOPrs {
    const RESET_VALUE: u32 = 0xffff;
}
