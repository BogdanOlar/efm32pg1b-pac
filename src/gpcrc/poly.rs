///Register `POLY` reader
pub type R = crate::R<POLYrs>;
///Register `POLY` writer
pub type W = crate::W<POLYrs>;
///Field `POLY` reader - CRC Polynomial Value
pub type PolyR = crate::FieldReader<u16>;
///Field `POLY` writer - CRC Polynomial Value
pub type PolyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - CRC Polynomial Value
    #[inline(always)]
    pub fn poly(&self) -> PolyR {
        PolyR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POLY").field("poly", &self.poly()).finish()
    }
}
impl W {
    ///Bits 0:15 - CRC Polynomial Value
    #[inline(always)]
    #[must_use]
    pub fn poly(&mut self) -> PolyW<POLYrs> {
        PolyW::new(self, 0)
    }
}
///CRC Polynomial Value
///
///You can [`read`](crate::Reg::read) this register and get [`poly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct POLYrs;
impl crate::RegisterSpec for POLYrs {
    type Ux = u32;
}
///`read()` method returns [`poly::R`](R) reader structure
impl crate::Readable for POLYrs {}
///`write(|w| ..)` method takes [`poly::W`](W) writer structure
impl crate::Writable for POLYrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets POLY to value 0
impl crate::Resettable for POLYrs {
    const RESET_VALUE: u32 = 0;
}
