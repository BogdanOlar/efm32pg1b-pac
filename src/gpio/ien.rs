///Register `IEN` reader
pub type R = crate::R<IENrs>;
///Register `IEN` writer
pub type W = crate::W<IENrs>;
///Field `EXT` reader - EXT Interrupt Enable
pub type ExtR = crate::FieldReader<u16>;
///Field `EXT` writer - EXT Interrupt Enable
pub type ExtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `EM4WU` reader - EM4WU Interrupt Enable
pub type Em4wuR = crate::FieldReader<u16>;
///Field `EM4WU` writer - EM4WU Interrupt Enable
pub type Em4wuW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - EXT Interrupt Enable
    #[inline(always)]
    pub fn ext(&self) -> ExtR {
        ExtR::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - EM4WU Interrupt Enable
    #[inline(always)]
    pub fn em4wu(&self) -> Em4wuR {
        Em4wuR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEN")
            .field("ext", &self.ext())
            .field("em4wu", &self.em4wu())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - EXT Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn ext(&mut self) -> ExtW<IENrs> {
        ExtW::new(self, 0)
    }
    ///Bits 16:31 - EM4WU Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn em4wu(&mut self) -> Em4wuW<IENrs> {
        Em4wuW::new(self, 16)
    }
}
///Interrupt Enable Register
///
///You can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IENrs;
impl crate::RegisterSpec for IENrs {
    type Ux = u32;
}
///`read()` method returns [`ien::R`](R) reader structure
impl crate::Readable for IENrs {}
///`write(|w| ..)` method takes [`ien::W`](W) writer structure
impl crate::Writable for IENrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IEN to value 0
impl crate::Resettable for IENrs {
    const RESET_VALUE: u32 = 0;
}
