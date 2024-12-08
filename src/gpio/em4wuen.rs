///Register `EM4WUEN` reader
pub type R = crate::R<EM4WUENrs>;
///Register `EM4WUEN` writer
pub type W = crate::W<EM4WUENrs>;
///Field `EM4WUEN` reader - EM4 Wake Up Enable
pub type Em4wuenR = crate::FieldReader<u16>;
///Field `EM4WUEN` writer - EM4 Wake Up Enable
pub type Em4wuenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 16:31 - EM4 Wake Up Enable
    #[inline(always)]
    pub fn em4wuen(&self) -> Em4wuenR {
        Em4wuenR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EM4WUEN")
            .field("em4wuen", &self.em4wuen())
            .finish()
    }
}
impl W {
    ///Bits 16:31 - EM4 Wake Up Enable
    #[inline(always)]
    #[must_use]
    pub fn em4wuen(&mut self) -> Em4wuenW<EM4WUENrs> {
        Em4wuenW::new(self, 16)
    }
}
///EM4 Wake Up Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`em4wuen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4wuen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct EM4WUENrs;
impl crate::RegisterSpec for EM4WUENrs {
    type Ux = u32;
}
///`read()` method returns [`em4wuen::R`](R) reader structure
impl crate::Readable for EM4WUENrs {}
///`write(|w| ..)` method takes [`em4wuen::W`](W) writer structure
impl crate::Writable for EM4WUENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EM4WUEN to value 0
impl crate::Resettable for EM4WUENrs {
    const RESET_VALUE: u32 = 0;
}
