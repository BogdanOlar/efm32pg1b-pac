///Register `WRITECTRL` reader
pub type R = crate::R<WRITECTRLrs>;
///Register `WRITECTRL` writer
pub type W = crate::W<WRITECTRLrs>;
///Field `WREN` reader - Enable Write/Erase Controller
pub type WrenR = crate::BitReader;
///Field `WREN` writer - Enable Write/Erase Controller
pub type WrenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IRQERASEABORT` reader - Abort Page Erase on Interrupt
pub type IrqeraseabortR = crate::BitReader;
///Field `IRQERASEABORT` writer - Abort Page Erase on Interrupt
pub type IrqeraseabortW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Enable Write/Erase Controller
    #[inline(always)]
    pub fn wren(&self) -> WrenR {
        WrenR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Abort Page Erase on Interrupt
    #[inline(always)]
    pub fn irqeraseabort(&self) -> IrqeraseabortR {
        IrqeraseabortR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRITECTRL")
            .field("wren", &self.wren())
            .field("irqeraseabort", &self.irqeraseabort())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable Write/Erase Controller
    #[inline(always)]
    #[must_use]
    pub fn wren(&mut self) -> WrenW<WRITECTRLrs> {
        WrenW::new(self, 0)
    }
    ///Bit 1 - Abort Page Erase on Interrupt
    #[inline(always)]
    #[must_use]
    pub fn irqeraseabort(&mut self) -> IrqeraseabortW<WRITECTRLrs> {
        IrqeraseabortW::new(self, 1)
    }
}
///Write Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`writectrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writectrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct WRITECTRLrs;
impl crate::RegisterSpec for WRITECTRLrs {
    type Ux = u32;
}
///`read()` method returns [`writectrl::R`](R) reader structure
impl crate::Readable for WRITECTRLrs {}
///`write(|w| ..)` method takes [`writectrl::W`](W) writer structure
impl crate::Writable for WRITECTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WRITECTRL to value 0
impl crate::Resettable for WRITECTRLrs {
    const RESET_VALUE: u32 = 0;
}
