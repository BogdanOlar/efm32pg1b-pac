///Register `CLKDIV` reader
pub type R = crate::R<CLKDIVrs>;
///Register `CLKDIV` writer
pub type W = crate::W<CLKDIVrs>;
///Field `DIV` reader - Fractional Clock Divider
pub type DivR = crate::FieldReader<u32>;
///Field `DIV` writer - Fractional Clock Divider
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
///Field `AUTOBAUDEN` reader - AUTOBAUD Detection Enable
pub type AutobaudenR = crate::BitReader;
///Field `AUTOBAUDEN` writer - AUTOBAUD Detection Enable
pub type AutobaudenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 3:22 - Fractional Clock Divider
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits >> 3) & 0x000f_ffff)
    }
    ///Bit 31 - AUTOBAUD Detection Enable
    #[inline(always)]
    pub fn autobauden(&self) -> AutobaudenR {
        AutobaudenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKDIV")
            .field("div", &self.div())
            .field("autobauden", &self.autobauden())
            .finish()
    }
}
impl W {
    ///Bits 3:22 - Fractional Clock Divider
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<CLKDIVrs> {
        DivW::new(self, 3)
    }
    ///Bit 31 - AUTOBAUD Detection Enable
    #[inline(always)]
    #[must_use]
    pub fn autobauden(&mut self) -> AutobaudenW<CLKDIVrs> {
        AutobaudenW::new(self, 31)
    }
}
///Clock Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`clkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CLKDIVrs;
impl crate::RegisterSpec for CLKDIVrs {
    type Ux = u32;
}
///`read()` method returns [`clkdiv::R`](R) reader structure
impl crate::Readable for CLKDIVrs {}
///`write(|w| ..)` method takes [`clkdiv::W`](W) writer structure
impl crate::Writable for CLKDIVrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CLKDIV to value 0
impl crate::Resettable for CLKDIVrs {
    const RESET_VALUE: u32 = 0;
}
