///Register `HYSTERESIS0` reader
pub type R = crate::R<HYSTERESIS0rs>;
///Register `HYSTERESIS0` writer
pub type W = crate::W<HYSTERESIS0rs>;
///Hysteresis Select When ACMPOUT=0
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HYST {
    ///0: No hysteresis
    Hyst0 = 0,
    ///1: 14 mV hysteresis
    Hyst1 = 1,
    ///2: 25 mV hysteresis
    Hyst2 = 2,
    ///3: 30 mV hysteresis
    Hyst3 = 3,
    ///4: 35 mV hysteresis
    Hyst4 = 4,
    ///5: 39 mV hysteresis
    Hyst5 = 5,
    ///6: 42 mV hysteresis
    Hyst6 = 6,
    ///7: 45 mV hysteresis
    Hyst7 = 7,
    ///8: No hysteresis
    Hyst8 = 8,
    ///9: -14 mV hysteresis
    Hyst9 = 9,
    ///10: -25 mV hysteresis
    Hyst10 = 10,
    ///11: -30 mV hysteresis
    Hyst11 = 11,
    ///12: -35 mV hysteresis
    Hyst12 = 12,
    ///13: -39 mV hysteresis
    Hyst13 = 13,
    ///14: -42 mV hysteresis
    Hyst14 = 14,
    ///15: -45 mV hysteresis
    Hyst15 = 15,
}
impl From<HYST> for u8 {
    #[inline(always)]
    fn from(variant: HYST) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HYST {
    type Ux = u8;
}
impl crate::IsEnum for HYST {}
///Field `HYST` reader - Hysteresis Select When ACMPOUT=0
pub type HystR = crate::FieldReader<HYST>;
impl HystR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HYST {
        match self.bits {
            0 => HYST::Hyst0,
            1 => HYST::Hyst1,
            2 => HYST::Hyst2,
            3 => HYST::Hyst3,
            4 => HYST::Hyst4,
            5 => HYST::Hyst5,
            6 => HYST::Hyst6,
            7 => HYST::Hyst7,
            8 => HYST::Hyst8,
            9 => HYST::Hyst9,
            10 => HYST::Hyst10,
            11 => HYST::Hyst11,
            12 => HYST::Hyst12,
            13 => HYST::Hyst13,
            14 => HYST::Hyst14,
            15 => HYST::Hyst15,
            _ => unreachable!(),
        }
    }
    ///No hysteresis
    #[inline(always)]
    pub fn is_hyst0(&self) -> bool {
        *self == HYST::Hyst0
    }
    ///14 mV hysteresis
    #[inline(always)]
    pub fn is_hyst1(&self) -> bool {
        *self == HYST::Hyst1
    }
    ///25 mV hysteresis
    #[inline(always)]
    pub fn is_hyst2(&self) -> bool {
        *self == HYST::Hyst2
    }
    ///30 mV hysteresis
    #[inline(always)]
    pub fn is_hyst3(&self) -> bool {
        *self == HYST::Hyst3
    }
    ///35 mV hysteresis
    #[inline(always)]
    pub fn is_hyst4(&self) -> bool {
        *self == HYST::Hyst4
    }
    ///39 mV hysteresis
    #[inline(always)]
    pub fn is_hyst5(&self) -> bool {
        *self == HYST::Hyst5
    }
    ///42 mV hysteresis
    #[inline(always)]
    pub fn is_hyst6(&self) -> bool {
        *self == HYST::Hyst6
    }
    ///45 mV hysteresis
    #[inline(always)]
    pub fn is_hyst7(&self) -> bool {
        *self == HYST::Hyst7
    }
    ///No hysteresis
    #[inline(always)]
    pub fn is_hyst8(&self) -> bool {
        *self == HYST::Hyst8
    }
    ///-14 mV hysteresis
    #[inline(always)]
    pub fn is_hyst9(&self) -> bool {
        *self == HYST::Hyst9
    }
    ///-25 mV hysteresis
    #[inline(always)]
    pub fn is_hyst10(&self) -> bool {
        *self == HYST::Hyst10
    }
    ///-30 mV hysteresis
    #[inline(always)]
    pub fn is_hyst11(&self) -> bool {
        *self == HYST::Hyst11
    }
    ///-35 mV hysteresis
    #[inline(always)]
    pub fn is_hyst12(&self) -> bool {
        *self == HYST::Hyst12
    }
    ///-39 mV hysteresis
    #[inline(always)]
    pub fn is_hyst13(&self) -> bool {
        *self == HYST::Hyst13
    }
    ///-42 mV hysteresis
    #[inline(always)]
    pub fn is_hyst14(&self) -> bool {
        *self == HYST::Hyst14
    }
    ///-45 mV hysteresis
    #[inline(always)]
    pub fn is_hyst15(&self) -> bool {
        *self == HYST::Hyst15
    }
}
///Field `HYST` writer - Hysteresis Select When ACMPOUT=0
pub type HystW<'a, REG> = crate::FieldWriter<'a, REG, 4, HYST, crate::Safe>;
impl<'a, REG> HystW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No hysteresis
    #[inline(always)]
    pub fn hyst0(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::Hyst0)
    }
    ///14 mV hysteresis
    #[inline(always)]
    pub fn hyst1(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::Hyst1)
    }
    ///25 mV hysteresis
    #[inline(always)]
    pub fn hyst2(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::Hyst2)
    }
    ///30 mV hysteresis
    #[inline(always)]
    pub fn hyst3(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::Hyst3)
    }
    ///35 mV hysteresis
    #[inline(always)]
    pub fn hyst4(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::Hyst4)
    }
    ///39 mV hysteresis
    #[inline(always)]
    pub fn hyst5(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::Hyst5)
    }
    ///42 mV hysteresis
    #[inline(always)]
    pub fn hyst6(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::Hyst6)
    }
    ///45 mV hysteresis
    #[inline(always)]
    pub fn hyst7(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::Hyst7)
    }
    ///No hysteresis
    #[inline(always)]
    pub fn hyst8(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::Hyst8)
    }
    ///-14 mV hysteresis
    #[inline(always)]
    pub fn hyst9(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::Hyst9)
    }
    ///-25 mV hysteresis
    #[inline(always)]
    pub fn hyst10(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::Hyst10)
    }
    ///-30 mV hysteresis
    #[inline(always)]
    pub fn hyst11(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::Hyst11)
    }
    ///-35 mV hysteresis
    #[inline(always)]
    pub fn hyst12(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::Hyst12)
    }
    ///-39 mV hysteresis
    #[inline(always)]
    pub fn hyst13(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::Hyst13)
    }
    ///-42 mV hysteresis
    #[inline(always)]
    pub fn hyst14(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::Hyst14)
    }
    ///-45 mV hysteresis
    #[inline(always)]
    pub fn hyst15(self) -> &'a mut crate::W<REG> {
        self.variant(HYST::Hyst15)
    }
}
///Field `DIVVA` reader - Divider for VA Voltage When ACMPOUT=0
pub type DivvaR = crate::FieldReader;
///Field `DIVVA` writer - Divider for VA Voltage When ACMPOUT=0
pub type DivvaW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `DIVVB` reader - Divider for VB Voltage When ACMPOUT=0
pub type DivvbR = crate::FieldReader;
///Field `DIVVB` writer - Divider for VB Voltage When ACMPOUT=0
pub type DivvbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:3 - Hysteresis Select When ACMPOUT=0
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new((self.bits & 0x0f) as u8)
    }
    ///Bits 16:21 - Divider for VA Voltage When ACMPOUT=0
    #[inline(always)]
    pub fn divva(&self) -> DivvaR {
        DivvaR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:29 - Divider for VB Voltage When ACMPOUT=0
    #[inline(always)]
    pub fn divvb(&self) -> DivvbR {
        DivvbR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HYSTERESIS0")
            .field("hyst", &self.hyst())
            .field("divva", &self.divva())
            .field("divvb", &self.divvb())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Hysteresis Select When ACMPOUT=0
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HystW<HYSTERESIS0rs> {
        HystW::new(self, 0)
    }
    ///Bits 16:21 - Divider for VA Voltage When ACMPOUT=0
    #[inline(always)]
    #[must_use]
    pub fn divva(&mut self) -> DivvaW<HYSTERESIS0rs> {
        DivvaW::new(self, 16)
    }
    ///Bits 24:29 - Divider for VB Voltage When ACMPOUT=0
    #[inline(always)]
    #[must_use]
    pub fn divvb(&mut self) -> DivvbW<HYSTERESIS0rs> {
        DivvbW::new(self, 24)
    }
}
///Hysteresis 0 Register
///
///You can [`read`](crate::Reg::read) this register and get [`hysteresis0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hysteresis0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct HYSTERESIS0rs;
impl crate::RegisterSpec for HYSTERESIS0rs {
    type Ux = u32;
}
///`read()` method returns [`hysteresis0::R`](R) reader structure
impl crate::Readable for HYSTERESIS0rs {}
///`write(|w| ..)` method takes [`hysteresis0::W`](W) writer structure
impl crate::Writable for HYSTERESIS0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HYSTERESIS0 to value 0
impl crate::Resettable for HYSTERESIS0rs {
    const RESET_VALUE: u32 = 0;
}
