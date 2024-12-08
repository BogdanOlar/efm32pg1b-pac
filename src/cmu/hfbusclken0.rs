///Register `HFBUSCLKEN0` reader
pub type R = crate::R<HFBUSCLKEN0rs>;
///Register `HFBUSCLKEN0` writer
pub type W = crate::W<HFBUSCLKEN0rs>;
///Field `LE` reader - Low Energy Peripheral Interface Clock Enable
pub type LeR = crate::BitReader;
///Field `LE` writer - Low Energy Peripheral Interface Clock Enable
pub type LeW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYPTO` reader - Advanced Encryption Standard Accelerator Clock Enable
pub type CryptoR = crate::BitReader;
///Field `CRYPTO` writer - Advanced Encryption Standard Accelerator Clock Enable
pub type CryptoW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO` reader - General purpose Input/Output Clock Enable
pub type GpioR = crate::BitReader;
///Field `GPIO` writer - General purpose Input/Output Clock Enable
pub type GpioW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRS` reader - Peripheral Reflex System Clock Enable
pub type PrsR = crate::BitReader;
///Field `PRS` writer - Peripheral Reflex System Clock Enable
pub type PrsW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LDMA` reader - Linked Direct Memory Access Controller Clock Enable
pub type LdmaR = crate::BitReader;
///Field `LDMA` writer - Linked Direct Memory Access Controller Clock Enable
pub type LdmaW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPCRC` reader - General Purpose CRC Clock Enable
pub type GpcrcR = crate::BitReader;
///Field `GPCRC` writer - General Purpose CRC Clock Enable
pub type GpcrcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Low Energy Peripheral Interface Clock Enable
    #[inline(always)]
    pub fn le(&self) -> LeR {
        LeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Advanced Encryption Standard Accelerator Clock Enable
    #[inline(always)]
    pub fn crypto(&self) -> CryptoR {
        CryptoR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - General purpose Input/Output Clock Enable
    #[inline(always)]
    pub fn gpio(&self) -> GpioR {
        GpioR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Peripheral Reflex System Clock Enable
    #[inline(always)]
    pub fn prs(&self) -> PrsR {
        PrsR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Linked Direct Memory Access Controller Clock Enable
    #[inline(always)]
    pub fn ldma(&self) -> LdmaR {
        LdmaR::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - General Purpose CRC Clock Enable
    #[inline(always)]
    pub fn gpcrc(&self) -> GpcrcR {
        GpcrcR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFBUSCLKEN0")
            .field("le", &self.le())
            .field("crypto", &self.crypto())
            .field("gpio", &self.gpio())
            .field("prs", &self.prs())
            .field("ldma", &self.ldma())
            .field("gpcrc", &self.gpcrc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Low Energy Peripheral Interface Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn le(&mut self) -> LeW<HFBUSCLKEN0rs> {
        LeW::new(self, 0)
    }
    ///Bit 1 - Advanced Encryption Standard Accelerator Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn crypto(&mut self) -> CryptoW<HFBUSCLKEN0rs> {
        CryptoW::new(self, 1)
    }
    ///Bit 2 - General purpose Input/Output Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self) -> GpioW<HFBUSCLKEN0rs> {
        GpioW::new(self, 2)
    }
    ///Bit 3 - Peripheral Reflex System Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn prs(&mut self) -> PrsW<HFBUSCLKEN0rs> {
        PrsW::new(self, 3)
    }
    ///Bit 4 - Linked Direct Memory Access Controller Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn ldma(&mut self) -> LdmaW<HFBUSCLKEN0rs> {
        LdmaW::new(self, 4)
    }
    ///Bit 5 - General Purpose CRC Clock Enable
    #[inline(always)]
    #[must_use]
    pub fn gpcrc(&mut self) -> GpcrcW<HFBUSCLKEN0rs> {
        GpcrcW::new(self, 5)
    }
}
///High Frequency Bus Clock Enable Register 0
///
///You can [`read`](crate::Reg::read) this register and get [`hfbusclken0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfbusclken0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HFBUSCLKEN0rs;
impl crate::RegisterSpec for HFBUSCLKEN0rs {
    type Ux = u32;
}
///`read()` method returns [`hfbusclken0::R`](R) reader structure
impl crate::Readable for HFBUSCLKEN0rs {}
///`write(|w| ..)` method takes [`hfbusclken0::W`](W) writer structure
impl crate::Writable for HFBUSCLKEN0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HFBUSCLKEN0 to value 0
impl crate::Resettable for HFBUSCLKEN0rs {
    const RESET_VALUE: u32 = 0;
}
