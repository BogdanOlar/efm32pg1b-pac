#[doc = "Register `WRITECTRL` reader"]
pub type R = crate::R<WRITECTRLrs>;
#[doc = "Register `WRITECTRL` writer"]
pub type W = crate::W<WRITECTRLrs>;
#[doc = "Field `WREN` reader - Enable Write/Erase Controller"]
pub type WrenR = crate::BitReader;
#[doc = "Field `WREN` writer - Enable Write/Erase Controller"]
pub type WrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRQERASEABORT` reader - Abort Page Erase on Interrupt"]
pub type IrqeraseabortR = crate::BitReader;
#[doc = "Field `IRQERASEABORT` writer - Abort Page Erase on Interrupt"]
pub type IrqeraseabortW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Write/Erase Controller"]
    #[inline(always)]
    pub fn wren(&self) -> WrenR {
        WrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Abort Page Erase on Interrupt"]
    #[inline(always)]
    pub fn irqeraseabort(&self) -> IrqeraseabortR {
        IrqeraseabortR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write/Erase Controller"]
    #[inline(always)]
    #[must_use]
    pub fn wren(&mut self) -> WrenW<WRITECTRLrs> {
        WrenW::new(self, 0)
    }
    #[doc = "Bit 1 - Abort Page Erase on Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn irqeraseabort(&mut self) -> IrqeraseabortW<WRITECTRLrs> {
        IrqeraseabortW::new(self, 1)
    }
}
#[doc = "Write Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`writectrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`writectrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRITECTRLrs;
impl crate::RegisterSpec for WRITECTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`writectrl::R`](R) reader structure"]
impl crate::Readable for WRITECTRLrs {}
#[doc = "`write(|w| ..)` method takes [`writectrl::W`](W) writer structure"]
impl crate::Writable for WRITECTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WRITECTRL to value 0"]
impl crate::Resettable for WRITECTRLrs {
    const RESET_VALUE: u32 = 0;
}
