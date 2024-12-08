///Register `CTRL` reader
pub type R = crate::R<CTRLrs>;
///Register `CTRL` writer
pub type W = crate::W<CTRLrs>;
///Field `ADDRFAULTEN` reader - Invalid Address Bus Fault Response Enable
pub type AddrfaultenR = crate::BitReader;
///Field `ADDRFAULTEN` writer - Invalid Address Bus Fault Response Enable
pub type AddrfaultenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLKDISFAULTEN` reader - Clock-disabled Bus Fault Response Enable
pub type ClkdisfaultenR = crate::BitReader;
///Field `CLKDISFAULTEN` writer - Clock-disabled Bus Fault Response Enable
pub type ClkdisfaultenW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRUPONDEMAND` reader - Power Up on Demand During Wake Up
pub type PwrupondemandR = crate::BitReader;
///Field `PWRUPONDEMAND` writer - Power Up on Demand During Wake Up
pub type PwrupondemandW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IFCREADCLEAR` reader - IFC Read Clears IF
pub type IfcreadclearR = crate::BitReader;
///Field `IFCREADCLEAR` writer - IFC Read Clears IF
pub type IfcreadclearW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Invalid Address Bus Fault Response Enable
    #[inline(always)]
    pub fn addrfaulten(&self) -> AddrfaultenR {
        AddrfaultenR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clock-disabled Bus Fault Response Enable
    #[inline(always)]
    pub fn clkdisfaulten(&self) -> ClkdisfaultenR {
        ClkdisfaultenR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Power Up on Demand During Wake Up
    #[inline(always)]
    pub fn pwrupondemand(&self) -> PwrupondemandR {
        PwrupondemandR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IFC Read Clears IF
    #[inline(always)]
    pub fn ifcreadclear(&self) -> IfcreadclearR {
        IfcreadclearR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("addrfaulten", &self.addrfaulten())
            .field("clkdisfaulten", &self.clkdisfaulten())
            .field("pwrupondemand", &self.pwrupondemand())
            .field("ifcreadclear", &self.ifcreadclear())
            .finish()
    }
}
impl W {
    ///Bit 0 - Invalid Address Bus Fault Response Enable
    #[inline(always)]
    #[must_use]
    pub fn addrfaulten(&mut self) -> AddrfaultenW<CTRLrs> {
        AddrfaultenW::new(self, 0)
    }
    ///Bit 1 - Clock-disabled Bus Fault Response Enable
    #[inline(always)]
    #[must_use]
    pub fn clkdisfaulten(&mut self) -> ClkdisfaultenW<CTRLrs> {
        ClkdisfaultenW::new(self, 1)
    }
    ///Bit 2 - Power Up on Demand During Wake Up
    #[inline(always)]
    #[must_use]
    pub fn pwrupondemand(&mut self) -> PwrupondemandW<CTRLrs> {
        PwrupondemandW::new(self, 2)
    }
    ///Bit 3 - IFC Read Clears IF
    #[inline(always)]
    #[must_use]
    pub fn ifcreadclear(&mut self) -> IfcreadclearW<CTRLrs> {
        IfcreadclearW::new(self, 3)
    }
}
///Memory System Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct CTRLrs;
impl crate::RegisterSpec for CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`ctrl::R`](R) reader structure
impl crate::Readable for CTRLrs {}
///`write(|w| ..)` method takes [`ctrl::W`](W) writer structure
impl crate::Writable for CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CTRL to value 0x01
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0x01;
}
