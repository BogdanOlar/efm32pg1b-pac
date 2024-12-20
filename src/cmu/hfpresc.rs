///Register `HFPRESC` reader
pub type R = crate::R<HFPRESCrs>;
///Register `HFPRESC` writer
pub type W = crate::W<HFPRESCrs>;
///HFCLK Prescaler
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC {
    ///0: `0`
    Nodivision = 0,
}
impl From<PRESC> for u8 {
    #[inline(always)]
    fn from(variant: PRESC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESC {
    type Ux = u8;
}
impl crate::IsEnum for PRESC {}
///Field `PRESC` reader - HFCLK Prescaler
pub type PrescR = crate::FieldReader<PRESC>;
impl PrescR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PRESC> {
        match self.bits {
            0 => Some(PRESC::Nodivision),
            _ => None,
        }
    }
    ///`0`
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == PRESC::Nodivision
    }
}
///Field `PRESC` writer - HFCLK Prescaler
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 5, PRESC>;
impl<'a, REG> PrescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///`0`
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Nodivision)
    }
}
///HFCLKLE Prescaler
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFCLKLEPRESC {
    ///0: HFCLKLE is HFBUSCLKLE divided by 2.
    Div2 = 0,
    ///1: HFCLKLE is HFBUSCLKLE divided by 4.
    Div4 = 1,
}
impl From<HFCLKLEPRESC> for bool {
    #[inline(always)]
    fn from(variant: HFCLKLEPRESC) -> Self {
        variant as u8 != 0
    }
}
///Field `HFCLKLEPRESC` reader - HFCLKLE Prescaler
pub type HfclkleprescR = crate::BitReader<HFCLKLEPRESC>;
impl HfclkleprescR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HFCLKLEPRESC {
        match self.bits {
            false => HFCLKLEPRESC::Div2,
            true => HFCLKLEPRESC::Div4,
        }
    }
    ///HFCLKLE is HFBUSCLKLE divided by 2.
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HFCLKLEPRESC::Div2
    }
    ///HFCLKLE is HFBUSCLKLE divided by 4.
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HFCLKLEPRESC::Div4
    }
}
///Field `HFCLKLEPRESC` writer - HFCLKLE Prescaler
pub type HfclkleprescW<'a, REG> = crate::BitWriter<'a, REG, HFCLKLEPRESC>;
impl<'a, REG> HfclkleprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HFCLKLE is HFBUSCLKLE divided by 2.
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(HFCLKLEPRESC::Div2)
    }
    ///HFCLKLE is HFBUSCLKLE divided by 4.
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(HFCLKLEPRESC::Div4)
    }
}
impl R {
    ///Bits 8:12 - HFCLK Prescaler
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bit 24 - HFCLKLE Prescaler
    #[inline(always)]
    pub fn hfclklepresc(&self) -> HfclkleprescR {
        HfclkleprescR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFPRESC")
            .field("presc", &self.presc())
            .field("hfclklepresc", &self.hfclklepresc())
            .finish()
    }
}
impl W {
    ///Bits 8:12 - HFCLK Prescaler
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PrescW<HFPRESCrs> {
        PrescW::new(self, 8)
    }
    ///Bit 24 - HFCLKLE Prescaler
    #[inline(always)]
    #[must_use]
    pub fn hfclklepresc(&mut self) -> HfclkleprescW<HFPRESCrs> {
        HfclkleprescW::new(self, 24)
    }
}
///High Frequency Clock Prescaler Register
///
///You can [`read`](crate::Reg::read) this register and get [`hfpresc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfpresc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HFPRESCrs;
impl crate::RegisterSpec for HFPRESCrs {
    type Ux = u32;
}
///`read()` method returns [`hfpresc::R`](R) reader structure
impl crate::Readable for HFPRESCrs {}
///`write(|w| ..)` method takes [`hfpresc::W`](W) writer structure
impl crate::Writable for HFPRESCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HFPRESC to value 0
impl crate::Resettable for HFPRESCrs {
    const RESET_VALUE: u32 = 0;
}
