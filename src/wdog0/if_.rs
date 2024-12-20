///Register `IF` reader
pub type R = crate::R<IFrs>;
///Field `TOUT` reader - WDOG Timeout Interrupt Flag
pub type ToutR = crate::BitReader;
///Field `WARN` reader - WDOG Warning Timeout Interrupt Flag
pub type WarnR = crate::BitReader;
///Field `WIN` reader - WDOG Window Interrupt Flag
pub type WinR = crate::BitReader;
///Field `PEM0` reader - PRS Channel Zero Event Missing Interrupt Flag
pub type Pem0R = crate::BitReader;
///Field `PEM1` reader - PRS Channel One Event Missing Interrupt Flag
pub type Pem1R = crate::BitReader;
impl R {
    ///Bit 0 - WDOG Timeout Interrupt Flag
    #[inline(always)]
    pub fn tout(&self) -> ToutR {
        ToutR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - WDOG Warning Timeout Interrupt Flag
    #[inline(always)]
    pub fn warn(&self) -> WarnR {
        WarnR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - WDOG Window Interrupt Flag
    #[inline(always)]
    pub fn win(&self) -> WinR {
        WinR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PRS Channel Zero Event Missing Interrupt Flag
    #[inline(always)]
    pub fn pem0(&self) -> Pem0R {
        Pem0R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PRS Channel One Event Missing Interrupt Flag
    #[inline(always)]
    pub fn pem1(&self) -> Pem1R {
        Pem1R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
            .field("tout", &self.tout())
            .field("warn", &self.warn())
            .field("win", &self.win())
            .field("pem0", &self.pem0())
            .field("pem1", &self.pem1())
            .finish()
    }
}
///Watchdog Interrupt Flags
///
///You can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IFrs;
impl crate::RegisterSpec for IFrs {
    type Ux = u32;
}
///`read()` method returns [`if_::R`](R) reader structure
impl crate::Readable for IFrs {}
///`reset()` method sets IF to value 0
impl crate::Resettable for IFrs {
    const RESET_VALUE: u32 = 0;
}
