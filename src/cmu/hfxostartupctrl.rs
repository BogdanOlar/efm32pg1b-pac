///Register `HFXOSTARTUPCTRL` reader
pub type R = crate::R<HFXOSTARTUPCTRLrs>;
///Register `HFXOSTARTUPCTRL` writer
pub type W = crate::W<HFXOSTARTUPCTRLrs>;
///Field `IBTRIMXOCORE` reader - Sets the Startup Oscillator Core Bias Current
pub type IbtrimxocoreR = crate::FieldReader;
///Field `IBTRIMXOCORE` writer - Sets the Startup Oscillator Core Bias Current
pub type IbtrimxocoreW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `CTUNE` reader - Sets Oscillator Tuning Capacitance
pub type CtuneR = crate::FieldReader<u16>;
///Field `CTUNE` writer - Sets Oscillator Tuning Capacitance
pub type CtuneW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `RESERVED0` reader - This Field is Reserved. It Should Be Set to 0x9
pub type Reserved0R = crate::FieldReader;
///Field `RESERVED0` writer - This Field is Reserved. It Should Be Set to 0x9
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `RESERVED1` reader - Sets the Regulator Output Current Level (shunt Regulator)
pub type Reserved1R = crate::FieldReader;
///Field `RESERVED1` writer - Sets the Regulator Output Current Level (shunt Regulator)
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:6 - Sets the Startup Oscillator Core Bias Current
    #[inline(always)]
    pub fn ibtrimxocore(&self) -> IbtrimxocoreR {
        IbtrimxocoreR::new((self.bits & 0x7f) as u8)
    }
    ///Bits 11:19 - Sets Oscillator Tuning Capacitance
    #[inline(always)]
    pub fn ctune(&self) -> CtuneR {
        CtuneR::new(((self.bits >> 11) & 0x01ff) as u16)
    }
    ///Bits 21:27 - This Field is Reserved. It Should Be Set to 0x9
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 21) & 0x7f) as u8)
    }
    ///Bits 28:31 - Sets the Regulator Output Current Level (shunt Regulator)
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFXOSTARTUPCTRL")
            .field("ibtrimxocore", &self.ibtrimxocore())
            .field("ctune", &self.ctune())
            .field("reserved0", &self.reserved0())
            .field("reserved1", &self.reserved1())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Sets the Startup Oscillator Core Bias Current
    #[inline(always)]
    #[must_use]
    pub fn ibtrimxocore(&mut self) -> IbtrimxocoreW<HFXOSTARTUPCTRLrs> {
        IbtrimxocoreW::new(self, 0)
    }
    ///Bits 11:19 - Sets Oscillator Tuning Capacitance
    #[inline(always)]
    #[must_use]
    pub fn ctune(&mut self) -> CtuneW<HFXOSTARTUPCTRLrs> {
        CtuneW::new(self, 11)
    }
    ///Bits 21:27 - This Field is Reserved. It Should Be Set to 0x9
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<HFXOSTARTUPCTRLrs> {
        Reserved0W::new(self, 21)
    }
    ///Bits 28:31 - Sets the Regulator Output Current Level (shunt Regulator)
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<HFXOSTARTUPCTRLrs> {
        Reserved1W::new(self, 28)
    }
}
///HFXO Startup Control
///
///You can [`read`](crate::Reg::read) this register and get [`hfxostartupctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfxostartupctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HFXOSTARTUPCTRLrs;
impl crate::RegisterSpec for HFXOSTARTUPCTRLrs {
    type Ux = u32;
}
///`read()` method returns [`hfxostartupctrl::R`](R) reader structure
impl crate::Readable for HFXOSTARTUPCTRLrs {}
///`write(|w| ..)` method takes [`hfxostartupctrl::W`](W) writer structure
impl crate::Writable for HFXOSTARTUPCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HFXOSTARTUPCTRL to value 0xa125_0060
impl crate::Resettable for HFXOSTARTUPCTRLrs {
    const RESET_VALUE: u32 = 0xa125_0060;
}
