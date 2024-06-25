#[doc = "Register `EM4WUEN` reader"]
pub type R = crate::R<EM4WUENrs>;
#[doc = "Register `EM4WUEN` writer"]
pub type W = crate::W<EM4WUENrs>;
#[doc = "Field `EM4WU` reader - EM4 Wake-up Enable"]
pub type Em4wuR = crate::BitReader;
#[doc = "Field `EM4WU` writer - EM4 Wake-up Enable"]
pub type Em4wuW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EM4 Wake-up Enable"]
    #[inline(always)]
    pub fn em4wu(&self) -> Em4wuR {
        Em4wuR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EM4WUEN")
            .field("em4wu", &self.em4wu())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - EM4 Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu(&mut self) -> Em4wuW<EM4WUENrs> {
        Em4wuW::new(self, 0)
    }
}
#[doc = "Wake Up Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`em4wuen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4wuen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EM4WUENrs;
impl crate::RegisterSpec for EM4WUENrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`em4wuen::R`](R) reader structure"]
impl crate::Readable for EM4WUENrs {}
#[doc = "`write(|w| ..)` method takes [`em4wuen::W`](W) writer structure"]
impl crate::Writable for EM4WUENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EM4WUEN to value 0"]
impl crate::Resettable for EM4WUENrs {
    const RESET_VALUE: u32 = 0;
}
