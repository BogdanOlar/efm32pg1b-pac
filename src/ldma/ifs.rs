///Register `IFS` writer
pub type W = crate::W<IFSrs>;
///Field `DONE` writer - Set DONE Interrupt Flag
pub type DoneW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `ERROR` writer - Set ERROR Interrupt Flag
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFSrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - Set DONE Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<IFSrs> {
        DoneW::new(self, 0)
    }
    ///Bit 31 - Set ERROR Interrupt Flag
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ErrorW<IFSrs> {
        ErrorW::new(self, 31)
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
