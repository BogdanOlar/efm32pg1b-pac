///Register `IFS` writer
pub type W = crate::W<IFSrs>;
///Field `EXT` writer - Set EXT Interrupt Flag
pub type ExtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `EM4WU` writer - Set EM4WU Interrupt Flag
pub type Em4wuW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl core::fmt::Debug for crate::generic::Reg<IFSrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:15 - Set EXT Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn ext(&mut self) -> ExtW<IFSrs> {
        ExtW::new(self, 0)
    }
    ///Bits 16:31 - Set EM4WU Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn em4wu(&mut self) -> Em4wuW<IFSrs> {
        Em4wuW::new(self, 16)
    }
}
///Interrupt Flag Set Register
///
///You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IFSrs;
impl crate::RegisterSpec for IFSrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ifs::W`](W) writer structure
impl crate::Writable for IFSrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IFS to value 0
impl crate::Resettable for IFSrs {
    const RESET_VALUE: u32 = 0;
}
