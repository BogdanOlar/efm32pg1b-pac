///Register `HFPERPRESC` reader
pub type R = crate::R<HFPERPRESCrs>;
///Register `HFPERPRESC` writer
pub type W = crate::W<HFPERPRESCrs>;
///HFPERCLK Prescaler
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PRESC {
    ///0: `0`
    Nodivision = 0,
}
impl From<PRESC> for u16 {
    #[inline(always)]
    fn from(variant: PRESC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PRESC {
    type Ux = u16;
}
impl crate::IsEnum for PRESC {}
///Field `PRESC` reader - HFPERCLK Prescaler
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
///Field `PRESC` writer - HFPERCLK Prescaler
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 9, PRESC>;
impl<'a, REG> PrescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    ///`0`
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Nodivision)
    }
}
impl R {
    ///Bits 8:16 - HFPERCLK Prescaler
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFPERPRESC")
            .field("presc", &self.presc())
            .finish()
    }
}
impl W {
    ///Bits 8:16 - HFPERCLK Prescaler
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PrescW<HFPERPRESCrs> {
        PrescW::new(self, 8)
    }
}
///High Frequency Peripheral Clock Prescaler Register
///
///You can [`read`](crate::Reg::read) this register and get [`hfperpresc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfperpresc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HFPERPRESCrs;
impl crate::RegisterSpec for HFPERPRESCrs {
    type Ux = u32;
}
///`read()` method returns [`hfperpresc::R`](R) reader structure
impl crate::Readable for HFPERPRESCrs {}
///`write(|w| ..)` method takes [`hfperpresc::W`](W) writer structure
impl crate::Writable for HFPERPRESCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HFPERPRESC to value 0
impl crate::Resettable for HFPERPRESCrs {
    const RESET_VALUE: u32 = 0;
}
