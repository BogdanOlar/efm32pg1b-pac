///Register `CTRL` reader
pub type R = crate::R<CTRLrs>;
///Register `CTRL` writer
pub type W = crate::W<CTRLrs>;
///Field `EN` reader - Enable CRYOTIMER
pub type EnR = crate::BitReader;
///Field `EN` writer - Enable CRYOTIMER
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEBUGRUN` reader - Debug Mode Run Enable
pub type DebugrunR = crate::BitReader;
///Field `DEBUGRUN` writer - Debug Mode Run Enable
pub type DebugrunW<'a, REG> = crate::BitWriter<'a, REG>;
///Select Low Frequency Oscillator
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSCSEL {
    ///0: Select Low Frequency RC Oscillator
    Lfrco = 0,
    ///1: Select Low Frequency Crystal Oscillator
    Lfxo = 1,
    ///2: Select Ultra Low Frequency RC Oscillator
    Ulfrco = 2,
}
impl From<OSCSEL> for u8 {
    #[inline(always)]
    fn from(variant: OSCSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSCSEL {
    type Ux = u8;
}
impl crate::IsEnum for OSCSEL {}
///Field `OSCSEL` reader - Select Low Frequency Oscillator
pub type OscselR = crate::FieldReader<OSCSEL>;
impl OscselR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<OSCSEL> {
        match self.bits {
            0 => Some(OSCSEL::Lfrco),
            1 => Some(OSCSEL::Lfxo),
            2 => Some(OSCSEL::Ulfrco),
            _ => None,
        }
    }
    ///Select Low Frequency RC Oscillator
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == OSCSEL::Lfrco
    }
    ///Select Low Frequency Crystal Oscillator
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == OSCSEL::Lfxo
    }
    ///Select Ultra Low Frequency RC Oscillator
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == OSCSEL::Ulfrco
    }
}
///Field `OSCSEL` writer - Select Low Frequency Oscillator
pub type OscselW<'a, REG> = crate::FieldWriter<'a, REG, 2, OSCSEL>;
impl<'a, REG> OscselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Select Low Frequency RC Oscillator
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(OSCSEL::Lfrco)
    }
    ///Select Low Frequency Crystal Oscillator
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(OSCSEL::Lfxo)
    }
    ///Select Ultra Low Frequency RC Oscillator
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(OSCSEL::Ulfrco)
    }
}
///Prescaler Setting
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC {
    ///0: LF Oscillator frequency undivided
    Div1 = 0,
    ///1: LF Oscillator frequency divided by 2
    Div2 = 1,
    ///2: LF Oscillator frequency divided by 4
    Div4 = 2,
    ///3: LF Oscillator frequency divided by 8
    Div8 = 3,
    ///4: LF Oscillator frequency divided by 16
    Div16 = 4,
    ///5: LF Oscillator frequency divided by 32
    Div32 = 5,
    ///6: LF Oscillator frequency divided by 64
    Div64 = 6,
    ///7: LF Oscillator frequency divided by 128
    Div128 = 7,
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
///Field `PRESC` reader - Prescaler Setting
pub type PrescR = crate::FieldReader<PRESC>;
impl PrescR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRESC {
        match self.bits {
            0 => PRESC::Div1,
            1 => PRESC::Div2,
            2 => PRESC::Div4,
            3 => PRESC::Div8,
            4 => PRESC::Div16,
            5 => PRESC::Div32,
            6 => PRESC::Div64,
            7 => PRESC::Div128,
            _ => unreachable!(),
        }
    }
    ///LF Oscillator frequency undivided
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESC::Div1
    }
    ///LF Oscillator frequency divided by 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC::Div2
    }
    ///LF Oscillator frequency divided by 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC::Div4
    }
    ///LF Oscillator frequency divided by 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESC::Div8
    }
    ///LF Oscillator frequency divided by 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESC::Div16
    }
    ///LF Oscillator frequency divided by 32
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESC::Div32
    }
    ///LF Oscillator frequency divided by 64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESC::Div64
    }
    ///LF Oscillator frequency divided by 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESC::Div128
    }
}
///Field `PRESC` writer - Prescaler Setting
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 3, PRESC, crate::Safe>;
impl<'a, REG> PrescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///LF Oscillator frequency undivided
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div1)
    }
    ///LF Oscillator frequency divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div2)
    }
    ///LF Oscillator frequency divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div4)
    }
    ///LF Oscillator frequency divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div8)
    }
    ///LF Oscillator frequency divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div16)
    }
    ///LF Oscillator frequency divided by 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div32)
    }
    ///LF Oscillator frequency divided by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div64)
    }
    ///LF Oscillator frequency divided by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(PRESC::Div128)
    }
}
impl R {
    ///Bit 0 - Enable CRYOTIMER
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Debug Mode Run Enable
    #[inline(always)]
    pub fn debugrun(&self) -> DebugrunR {
        DebugrunR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Select Low Frequency Oscillator
    #[inline(always)]
    pub fn oscsel(&self) -> OscselR {
        OscselR::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 5:7 - Prescaler Setting
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(((self.bits >> 5) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("en", &self.en())
            .field("debugrun", &self.debugrun())
            .field("oscsel", &self.oscsel())
            .field("presc", &self.presc())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enable CRYOTIMER
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<CTRLrs> {
        EnW::new(self, 0)
    }
    ///Bit 1 - Debug Mode Run Enable
    #[inline(always)]
    #[must_use]
    pub fn debugrun(&mut self) -> DebugrunW<CTRLrs> {
        DebugrunW::new(self, 1)
    }
    ///Bits 2:3 - Select Low Frequency Oscillator
    #[inline(always)]
    #[must_use]
    pub fn oscsel(&mut self) -> OscselW<CTRLrs> {
        OscselW::new(self, 2)
    }
    ///Bits 5:7 - Prescaler Setting
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PrescW<CTRLrs> {
        PrescW::new(self, 5)
    }
}
///Control Register
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
///`reset()` method sets CTRL to value 0
impl crate::Resettable for CTRLrs {
    const RESET_VALUE: u32 = 0;
}
