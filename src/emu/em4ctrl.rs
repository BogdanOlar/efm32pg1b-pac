///Register `EM4CTRL` reader
pub type R = crate::R<EM4CTRLrs>;
///Register `EM4CTRL` writer
pub type W = crate::W<EM4CTRLrs>;
///Field `EM4STATE` reader - Energy Mode 4 State
pub type Em4stateR = crate::BitReader;
///Field `EM4STATE` writer - Energy Mode 4 State
pub type Em4stateW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RETAINLFRCO` reader - LFRCO Retain During EM4
pub type RetainlfrcoR = crate::BitReader;
///Field `RETAINLFRCO` writer - LFRCO Retain During EM4
pub type RetainlfrcoW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RETAINLFXO` reader - LFXO Retain During EM4
pub type RetainlfxoR = crate::BitReader;
///Field `RETAINLFXO` writer - LFXO Retain During EM4
pub type RetainlfxoW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RETAINULFRCO` reader - ULFRCO Retain During EM4S
pub type RetainulfrcoR = crate::BitReader;
///Field `RETAINULFRCO` writer - ULFRCO Retain During EM4S
pub type RetainulfrcoW<'a, REG> = crate::BitWriter<'a, REG>;
///EM4 IO Retention Disable
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EM4IORETMODE {
    ///0: No Retention: Pads enter reset state when entering EM4
    Disable = 0,
    ///1: Retention through EM4: Pads enter reset state when exiting EM4
    Em4exit = 1,
    ///2: Retention through EM4 and Wakeup: software writes UNLATCH register to remove retention
    Swunlatch = 2,
}
impl From<EM4IORETMODE> for u8 {
    #[inline(always)]
    fn from(variant: EM4IORETMODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EM4IORETMODE {
    type Ux = u8;
}
impl crate::IsEnum for EM4IORETMODE {}
///Field `EM4IORETMODE` reader - EM4 IO Retention Disable
pub type Em4ioretmodeR = crate::FieldReader<EM4IORETMODE>;
impl Em4ioretmodeR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<EM4IORETMODE> {
        match self.bits {
            0 => Some(EM4IORETMODE::Disable),
            1 => Some(EM4IORETMODE::Em4exit),
            2 => Some(EM4IORETMODE::Swunlatch),
            _ => None,
        }
    }
    ///No Retention: Pads enter reset state when entering EM4
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EM4IORETMODE::Disable
    }
    ///Retention through EM4: Pads enter reset state when exiting EM4
    #[inline(always)]
    pub fn is_em4exit(&self) -> bool {
        *self == EM4IORETMODE::Em4exit
    }
    ///Retention through EM4 and Wakeup: software writes UNLATCH register to remove retention
    #[inline(always)]
    pub fn is_swunlatch(&self) -> bool {
        *self == EM4IORETMODE::Swunlatch
    }
}
///Field `EM4IORETMODE` writer - EM4 IO Retention Disable
pub type Em4ioretmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, EM4IORETMODE>;
impl<'a, REG> Em4ioretmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No Retention: Pads enter reset state when entering EM4
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EM4IORETMODE::Disable)
    }
    ///Retention through EM4: Pads enter reset state when exiting EM4
    #[inline(always)]
    pub fn em4exit(self) -> &'a mut crate::W<REG> {
        self.variant(EM4IORETMODE::Em4exit)
    }
    ///Retention through EM4 and Wakeup: software writes UNLATCH register to remove retention
    #[inline(always)]
    pub fn swunlatch(self) -> &'a mut crate::W<REG> {
        self.variant(EM4IORETMODE::Swunlatch)
    }
}
///Field `EM4ENTRY` writer - Energy Mode 4 Entry
pub type Em4entryW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Energy Mode 4 State
    #[inline(always)]
    pub fn em4state(&self) -> Em4stateR {
        Em4stateR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LFRCO Retain During EM4
    #[inline(always)]
    pub fn retainlfrco(&self) -> RetainlfrcoR {
        RetainlfrcoR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LFXO Retain During EM4
    #[inline(always)]
    pub fn retainlfxo(&self) -> RetainlfxoR {
        RetainlfxoR::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ULFRCO Retain During EM4S
    #[inline(always)]
    pub fn retainulfrco(&self) -> RetainulfrcoR {
        RetainulfrcoR::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - EM4 IO Retention Disable
    #[inline(always)]
    pub fn em4ioretmode(&self) -> Em4ioretmodeR {
        Em4ioretmodeR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EM4CTRL")
            .field("em4state", &self.em4state())
            .field("retainlfrco", &self.retainlfrco())
            .field("retainlfxo", &self.retainlfxo())
            .field("retainulfrco", &self.retainulfrco())
            .field("em4ioretmode", &self.em4ioretmode())
            .finish()
    }
}
impl W {
    ///Bit 0 - Energy Mode 4 State
    #[inline(always)]
    #[must_use]
    pub fn em4state(&mut self) -> Em4stateW<EM4CTRLrs> {
        Em4stateW::new(self, 0)
    }
    ///Bit 1 - LFRCO Retain During EM4
    #[inline(always)]
    #[must_use]
    pub fn retainlfrco(&mut self) -> RetainlfrcoW<EM4CTRLrs> {
        RetainlfrcoW::new(self, 1)
    }
    ///Bit 2 - LFXO Retain During EM4
    #[inline(always)]
    #[must_use]
    pub fn retainlfxo(&mut self) -> RetainlfxoW<EM4CTRLrs> {
        RetainlfxoW::new(self, 2)
    }
    ///Bit 3 - ULFRCO Retain During EM4S
    #[inline(always)]
    #[must_use]
    pub fn retainulfrco(&mut self) -> RetainulfrcoW<EM4CTRLrs> {
        RetainulfrcoW::new(self, 3)
    }
    ///Bits 4:5 - EM4 IO Retention Disable
    #[inline(always)]
    #[must_use]
    pub fn em4ioretmode(&mut self) -> Em4ioretmodeW<EM4CTRLrs> {
        Em4ioretmodeW::new(self, 4)
    }
    ///Bits 16:17 - Energy Mode 4 Entry
    #[inline(always)]
    #[must_use]
    pub fn em4entry(&mut self) -> Em4entryW<EM4CTRLrs> {
        Em4entryW::new(self, 16)
    }
}
///EM4 Control Register
///
///You can [`read`](crate::Reg::read) this register and get [`em4ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct EM4CTRLrs;
impl crate::RegisterSpec for EM4CTRLrs {
    type Ux = u32;
}
///`read()` method returns [`em4ctrl::R`](R) reader structure
impl crate::Readable for EM4CTRLrs {}
///`write(|w| ..)` method takes [`em4ctrl::W`](W) writer structure
impl crate::Writable for EM4CTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EM4CTRL to value 0
impl crate::Resettable for EM4CTRLrs {
    const RESET_VALUE: u32 = 0;
}
