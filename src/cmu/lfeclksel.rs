///Register `LFECLKSEL` reader
pub type R = crate::R<LFECLKSELrs>;
///Register `LFECLKSEL` writer
pub type W = crate::W<LFECLKSELrs>;
///Clock Select for LFE
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFE {
    ///0: LFECLK is disabled
    Disabled = 0,
    ///1: LFRCO selected as LFECLK
    Lfrco = 1,
    ///2: LFXO selected as LFECLK
    Lfxo = 2,
    ///4: ULFRCO selected as LFECLK
    Ulfrco = 4,
}
impl From<LFE> for u8 {
    #[inline(always)]
    fn from(variant: LFE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LFE {
    type Ux = u8;
}
impl crate::IsEnum for LFE {}
///Field `LFE` reader - Clock Select for LFE
pub type LfeR = crate::FieldReader<LFE>;
impl LfeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<LFE> {
        match self.bits {
            0 => Some(LFE::Disabled),
            1 => Some(LFE::Lfrco),
            2 => Some(LFE::Lfxo),
            4 => Some(LFE::Ulfrco),
            _ => None,
        }
    }
    ///LFECLK is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFE::Disabled
    }
    ///LFRCO selected as LFECLK
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == LFE::Lfrco
    }
    ///LFXO selected as LFECLK
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == LFE::Lfxo
    }
    ///ULFRCO selected as LFECLK
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == LFE::Ulfrco
    }
}
///Field `LFE` writer - Clock Select for LFE
pub type LfeW<'a, REG> = crate::FieldWriter<'a, REG, 3, LFE>;
impl<'a, REG> LfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///LFECLK is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LFE::Disabled)
    }
    ///LFRCO selected as LFECLK
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut crate::W<REG> {
        self.variant(LFE::Lfrco)
    }
    ///LFXO selected as LFECLK
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut crate::W<REG> {
        self.variant(LFE::Lfxo)
    }
    ///ULFRCO selected as LFECLK
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut crate::W<REG> {
        self.variant(LFE::Ulfrco)
    }
}
impl R {
    ///Bits 0:2 - Clock Select for LFE
    #[inline(always)]
    pub fn lfe(&self) -> LfeR {
        LfeR::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LFECLKSEL")
            .field("lfe", &self.lfe())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Clock Select for LFE
    #[inline(always)]
    #[must_use]
    pub fn lfe(&mut self) -> LfeW<LFECLKSELrs> {
        LfeW::new(self, 0)
    }
}
///Low Frequency E Clock Select Register
///
///You can [`read`](crate::Reg::read) this register and get [`lfeclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfeclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LFECLKSELrs;
impl crate::RegisterSpec for LFECLKSELrs {
    type Ux = u32;
}
///`read()` method returns [`lfeclksel::R`](R) reader structure
impl crate::Readable for LFECLKSELrs {}
///`write(|w| ..)` method takes [`lfeclksel::W`](W) writer structure
impl crate::Writable for LFECLKSELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LFECLKSEL to value 0
impl crate::Resettable for LFECLKSELrs {
    const RESET_VALUE: u32 = 0;
}
