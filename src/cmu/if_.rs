///Register `IF` reader
pub type R = crate::R<IFrs>;
///Field `HFRCORDY` reader - HFRCO Ready Interrupt Flag
pub type HfrcordyR = crate::BitReader;
///Field `HFXORDY` reader - HFXO Ready Interrupt Flag
pub type HfxordyR = crate::BitReader;
///Field `LFRCORDY` reader - LFRCO Ready Interrupt Flag
pub type LfrcordyR = crate::BitReader;
///Field `LFXORDY` reader - LFXO Ready Interrupt Flag
pub type LfxordyR = crate::BitReader;
///Field `AUXHFRCORDY` reader - AUXHFRCO Ready Interrupt Flag
pub type AuxhfrcordyR = crate::BitReader;
///Field `CALRDY` reader - Calibration Ready Interrupt Flag
pub type CalrdyR = crate::BitReader;
///Field `CALOF` reader - Calibration Overflow Interrupt Flag
pub type CalofR = crate::BitReader;
///Field `HFXODISERR` reader - HFXO Disable Error Interrupt Flag
pub type HfxodiserrR = crate::BitReader;
///Field `HFXOAUTOSW` reader - HFXO Automatic Switch Interrupt Flag
pub type HfxoautoswR = crate::BitReader;
///Field `HFXOPEAKDETERR` reader - HFXO Automatic Peak Detection Error Interrupt Flag
pub type HfxopeakdeterrR = crate::BitReader;
///Field `HFXOPEAKDETRDY` reader - HFXO Automatic Peak Detection Ready Interrupt Flag
pub type HfxopeakdetrdyR = crate::BitReader;
///Field `HFXOSHUNTOPTRDY` reader - HFXO Automatic Shunt Current Optimization Ready Interrupt Flag
pub type HfxoshuntoptrdyR = crate::BitReader;
///Field `HFRCODIS` reader - HFRCO Disable Interrupt Flag
pub type HfrcodisR = crate::BitReader;
///Field `LFTIMEOUTERR` reader - Low Frequency Timeout Error Interrupt Flag
pub type LftimeouterrR = crate::BitReader;
///Field `CMUERR` reader - CMU Error Interrupt Flag
pub type CmuerrR = crate::BitReader;
impl R {
    ///Bit 0 - HFRCO Ready Interrupt Flag
    #[inline(always)]
    pub fn hfrcordy(&self) -> HfrcordyR {
        HfrcordyR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HFXO Ready Interrupt Flag
    #[inline(always)]
    pub fn hfxordy(&self) -> HfxordyR {
        HfxordyR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LFRCO Ready Interrupt Flag
    #[inline(always)]
    pub fn lfrcordy(&self) -> LfrcordyR {
        LfrcordyR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LFXO Ready Interrupt Flag
    #[inline(always)]
    pub fn lfxordy(&self) -> LfxordyR {
        LfxordyR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - AUXHFRCO Ready Interrupt Flag
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AuxhfrcordyR {
        AuxhfrcordyR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Calibration Ready Interrupt Flag
    #[inline(always)]
    pub fn calrdy(&self) -> CalrdyR {
        CalrdyR::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Calibration Overflow Interrupt Flag
    #[inline(always)]
    pub fn calof(&self) -> CalofR {
        CalofR::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - HFXO Disable Error Interrupt Flag
    #[inline(always)]
    pub fn hfxodiserr(&self) -> HfxodiserrR {
        HfxodiserrR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HFXO Automatic Switch Interrupt Flag
    #[inline(always)]
    pub fn hfxoautosw(&self) -> HfxoautoswR {
        HfxoautoswR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HFXO Automatic Peak Detection Error Interrupt Flag
    #[inline(always)]
    pub fn hfxopeakdeterr(&self) -> HfxopeakdeterrR {
        HfxopeakdeterrR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - HFXO Automatic Peak Detection Ready Interrupt Flag
    #[inline(always)]
    pub fn hfxopeakdetrdy(&self) -> HfxopeakdetrdyR {
        HfxopeakdetrdyR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - HFXO Automatic Shunt Current Optimization Ready Interrupt Flag
    #[inline(always)]
    pub fn hfxoshuntoptrdy(&self) -> HfxoshuntoptrdyR {
        HfxoshuntoptrdyR::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - HFRCO Disable Interrupt Flag
    #[inline(always)]
    pub fn hfrcodis(&self) -> HfrcodisR {
        HfrcodisR::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Low Frequency Timeout Error Interrupt Flag
    #[inline(always)]
    pub fn lftimeouterr(&self) -> LftimeouterrR {
        LftimeouterrR::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 31 - CMU Error Interrupt Flag
    #[inline(always)]
    pub fn cmuerr(&self) -> CmuerrR {
        CmuerrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IF")
            .field("hfrcordy", &self.hfrcordy())
            .field("hfxordy", &self.hfxordy())
            .field("lfrcordy", &self.lfrcordy())
            .field("lfxordy", &self.lfxordy())
            .field("auxhfrcordy", &self.auxhfrcordy())
            .field("calrdy", &self.calrdy())
            .field("calof", &self.calof())
            .field("hfxodiserr", &self.hfxodiserr())
            .field("hfxoautosw", &self.hfxoautosw())
            .field("hfxopeakdeterr", &self.hfxopeakdeterr())
            .field("hfxopeakdetrdy", &self.hfxopeakdetrdy())
            .field("hfxoshuntoptrdy", &self.hfxoshuntoptrdy())
            .field("hfrcodis", &self.hfrcodis())
            .field("lftimeouterr", &self.lftimeouterr())
            .field("cmuerr", &self.cmuerr())
            .finish()
    }
}
///Interrupt Flag Register
///
///You can [`read`](crate::Reg::read) this register and get [`if_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct IFrs;
impl crate::RegisterSpec for IFrs {
    type Ux = u32;
}
///`read()` method returns [`if_::R`](R) reader structure
impl crate::Readable for IFrs {}
///`reset()` method sets IF to value 0x01
impl crate::Resettable for IFrs {
    const RESET_VALUE: u32 = 0x01;
}
