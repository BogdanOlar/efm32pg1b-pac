///Register `IEN` reader
pub type R = crate::R<IENrs>;
///Register `IEN` writer
pub type W = crate::W<IENrs>;
///Field `SINGLE` reader - SINGLE Interrupt Enable
pub type SingleR = crate::BitReader;
///Field `SINGLE` writer - SINGLE Interrupt Enable
pub type SingleW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCAN` reader - SCAN Interrupt Enable
pub type ScanR = crate::BitReader;
///Field `SCAN` writer - SCAN Interrupt Enable
pub type ScanW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SINGLEOF` reader - SINGLEOF Interrupt Enable
pub type SingleofR = crate::BitReader;
///Field `SINGLEOF` writer - SINGLEOF Interrupt Enable
pub type SingleofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCANOF` reader - SCANOF Interrupt Enable
pub type ScanofR = crate::BitReader;
///Field `SCANOF` writer - SCANOF Interrupt Enable
pub type ScanofW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SINGLEUF` reader - SINGLEUF Interrupt Enable
pub type SingleufR = crate::BitReader;
///Field `SINGLEUF` writer - SINGLEUF Interrupt Enable
pub type SingleufW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCANUF` reader - SCANUF Interrupt Enable
pub type ScanufR = crate::BitReader;
///Field `SCANUF` writer - SCANUF Interrupt Enable
pub type ScanufW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SINGLECMP` reader - SINGLECMP Interrupt Enable
pub type SinglecmpR = crate::BitReader;
///Field `SINGLECMP` writer - SINGLECMP Interrupt Enable
pub type SinglecmpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCANCMP` reader - SCANCMP Interrupt Enable
pub type ScancmpR = crate::BitReader;
///Field `SCANCMP` writer - SCANCMP Interrupt Enable
pub type ScancmpW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VREFOV` reader - VREFOV Interrupt Enable
pub type VrefovR = crate::BitReader;
///Field `VREFOV` writer - VREFOV Interrupt Enable
pub type VrefovW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PROGERR` reader - PROGERR Interrupt Enable
pub type ProgerrR = crate::BitReader;
///Field `PROGERR` writer - PROGERR Interrupt Enable
pub type ProgerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SINGLE Interrupt Enable
    #[inline(always)]
    pub fn single(&self) -> SingleR {
        SingleR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SCAN Interrupt Enable
    #[inline(always)]
    pub fn scan(&self) -> ScanR {
        ScanR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - SINGLEOF Interrupt Enable
    #[inline(always)]
    pub fn singleof(&self) -> SingleofR {
        SingleofR::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SCANOF Interrupt Enable
    #[inline(always)]
    pub fn scanof(&self) -> ScanofR {
        ScanofR::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SINGLEUF Interrupt Enable
    #[inline(always)]
    pub fn singleuf(&self) -> SingleufR {
        SingleufR::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SCANUF Interrupt Enable
    #[inline(always)]
    pub fn scanuf(&self) -> ScanufR {
        ScanufR::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - SINGLECMP Interrupt Enable
    #[inline(always)]
    pub fn singlecmp(&self) -> SinglecmpR {
        SinglecmpR::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SCANCMP Interrupt Enable
    #[inline(always)]
    pub fn scancmp(&self) -> ScancmpR {
        ScancmpR::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 24 - VREFOV Interrupt Enable
    #[inline(always)]
    pub fn vrefov(&self) -> VrefovR {
        VrefovR::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - PROGERR Interrupt Enable
    #[inline(always)]
    pub fn progerr(&self) -> ProgerrR {
        ProgerrR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEN")
            .field("single", &self.single())
            .field("scan", &self.scan())
            .field("singleof", &self.singleof())
            .field("scanof", &self.scanof())
            .field("singleuf", &self.singleuf())
            .field("scanuf", &self.scanuf())
            .field("singlecmp", &self.singlecmp())
            .field("scancmp", &self.scancmp())
            .field("vrefov", &self.vrefov())
            .field("progerr", &self.progerr())
            .finish()
    }
}
impl W {
    ///Bit 0 - SINGLE Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn single(&mut self) -> SingleW<IENrs> {
        SingleW::new(self, 0)
    }
    ///Bit 1 - SCAN Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn scan(&mut self) -> ScanW<IENrs> {
        ScanW::new(self, 1)
    }
    ///Bit 8 - SINGLEOF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn singleof(&mut self) -> SingleofW<IENrs> {
        SingleofW::new(self, 8)
    }
    ///Bit 9 - SCANOF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn scanof(&mut self) -> ScanofW<IENrs> {
        ScanofW::new(self, 9)
    }
    ///Bit 10 - SINGLEUF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn singleuf(&mut self) -> SingleufW<IENrs> {
        SingleufW::new(self, 10)
    }
    ///Bit 11 - SCANUF Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn scanuf(&mut self) -> ScanufW<IENrs> {
        ScanufW::new(self, 11)
    }
    ///Bit 16 - SINGLECMP Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn singlecmp(&mut self) -> SinglecmpW<IENrs> {
        SinglecmpW::new(self, 16)
    }
    ///Bit 17 - SCANCMP Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn scancmp(&mut self) -> ScancmpW<IENrs> {
        ScancmpW::new(self, 17)
    }
    ///Bit 24 - VREFOV Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn vrefov(&mut self) -> VrefovW<IENrs> {
        VrefovW::new(self, 24)
    }
    ///Bit 25 - PROGERR Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn progerr(&mut self) -> ProgerrW<IENrs> {
        ProgerrW::new(self, 25)
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
