#[doc = "Register `TOPB` reader"]
pub type R = crate::R<TOPBrs>;
#[doc = "Register `TOPB` writer"]
pub type W = crate::W<TOPBrs>;
#[doc = "Field `TOPB` reader - Counter Top Buffer"]
pub type TopbR = crate::FieldReader<u16>;
#[doc = "Field `TOPB` writer - Counter Top Buffer"]
pub type TopbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter Top Buffer"]
    #[inline(always)]
    pub fn topb(&self) -> TopbR {
        TopbR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOPB").field("topb", &self.topb()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter Top Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn topb(&mut self) -> TopbW<TOPBrs> {
        TopbW::new(self, 0)
    }
}
#[doc = "Top Value Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`topb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`topb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOPBrs;
impl crate::RegisterSpec for TOPBrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`topb::R`](R) reader structure"]
impl crate::Readable for TOPBrs {}
#[doc = "`write(|w| ..)` method takes [`topb::W`](W) writer structure"]
impl crate::Writable for TOPBrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOPB to value 0xff"]
impl crate::Resettable for TOPBrs {
    const RESET_VALUE: u32 = 0xff;
}
