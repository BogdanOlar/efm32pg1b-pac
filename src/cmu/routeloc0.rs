///Register `ROUTELOC0` reader
pub type R = crate::R<ROUTELOC0rs>;
///Register `ROUTELOC0` writer
pub type W = crate::W<ROUTELOC0rs>;
///I/O Location
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKOUT0LOC {
    ///0: Location 0
    Loc0 = 0,
    ///1: Location 1
    Loc1 = 1,
    ///2: Location 2
    Loc2 = 2,
    ///3: Location 3
    Loc3 = 3,
    ///4: Location 4
    Loc4 = 4,
    ///5: Location 5
    Loc5 = 5,
    ///6: Location 6
    Loc6 = 6,
    ///7: Location 7
    Loc7 = 7,
}
impl From<CLKOUT0LOC> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUT0LOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKOUT0LOC {
    type Ux = u8;
}
impl crate::IsEnum for CLKOUT0LOC {}
///Field `CLKOUT0LOC` reader - I/O Location
pub type Clkout0locR = crate::FieldReader<CLKOUT0LOC>;
impl Clkout0locR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKOUT0LOC> {
        match self.bits {
            0 => Some(CLKOUT0LOC::Loc0),
            1 => Some(CLKOUT0LOC::Loc1),
            2 => Some(CLKOUT0LOC::Loc2),
            3 => Some(CLKOUT0LOC::Loc3),
            4 => Some(CLKOUT0LOC::Loc4),
            5 => Some(CLKOUT0LOC::Loc5),
            6 => Some(CLKOUT0LOC::Loc6),
            7 => Some(CLKOUT0LOC::Loc7),
            _ => None,
        }
    }
    ///Location 0
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CLKOUT0LOC::Loc0
    }
    ///Location 1
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CLKOUT0LOC::Loc1
    }
    ///Location 2
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CLKOUT0LOC::Loc2
    }
    ///Location 3
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CLKOUT0LOC::Loc3
    }
    ///Location 4
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CLKOUT0LOC::Loc4
    }
    ///Location 5
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CLKOUT0LOC::Loc5
    }
    ///Location 6
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CLKOUT0LOC::Loc6
    }
    ///Location 7
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CLKOUT0LOC::Loc7
    }
}
///Field `CLKOUT0LOC` writer - I/O Location
pub type Clkout0locW<'a, REG> = crate::FieldWriter<'a, REG, 6, CLKOUT0LOC>;
impl<'a, REG> Clkout0locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Location 0
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT0LOC::Loc0)
    }
    ///Location 1
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT0LOC::Loc1)
    }
    ///Location 2
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT0LOC::Loc2)
    }
    ///Location 3
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT0LOC::Loc3)
    }
    ///Location 4
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT0LOC::Loc4)
    }
    ///Location 5
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT0LOC::Loc5)
    }
    ///Location 6
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT0LOC::Loc6)
    }
    ///Location 7
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT0LOC::Loc7)
    }
}
///I/O Location
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKOUT1LOC {
    ///0: Location 0
    Loc0 = 0,
    ///1: Location 1
    Loc1 = 1,
    ///2: Location 2
    Loc2 = 2,
    ///3: Location 3
    Loc3 = 3,
    ///4: Location 4
    Loc4 = 4,
    ///5: Location 5
    Loc5 = 5,
    ///6: Location 6
    Loc6 = 6,
    ///7: Location 7
    Loc7 = 7,
}
impl From<CLKOUT1LOC> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUT1LOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKOUT1LOC {
    type Ux = u8;
}
impl crate::IsEnum for CLKOUT1LOC {}
///Field `CLKOUT1LOC` reader - I/O Location
pub type Clkout1locR = crate::FieldReader<CLKOUT1LOC>;
impl Clkout1locR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKOUT1LOC> {
        match self.bits {
            0 => Some(CLKOUT1LOC::Loc0),
            1 => Some(CLKOUT1LOC::Loc1),
            2 => Some(CLKOUT1LOC::Loc2),
            3 => Some(CLKOUT1LOC::Loc3),
            4 => Some(CLKOUT1LOC::Loc4),
            5 => Some(CLKOUT1LOC::Loc5),
            6 => Some(CLKOUT1LOC::Loc6),
            7 => Some(CLKOUT1LOC::Loc7),
            _ => None,
        }
    }
    ///Location 0
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CLKOUT1LOC::Loc0
    }
    ///Location 1
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CLKOUT1LOC::Loc1
    }
    ///Location 2
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CLKOUT1LOC::Loc2
    }
    ///Location 3
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CLKOUT1LOC::Loc3
    }
    ///Location 4
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CLKOUT1LOC::Loc4
    }
    ///Location 5
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CLKOUT1LOC::Loc5
    }
    ///Location 6
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CLKOUT1LOC::Loc6
    }
    ///Location 7
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CLKOUT1LOC::Loc7
    }
}
///Field `CLKOUT1LOC` writer - I/O Location
pub type Clkout1locW<'a, REG> = crate::FieldWriter<'a, REG, 6, CLKOUT1LOC>;
impl<'a, REG> Clkout1locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Location 0
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1LOC::Loc0)
    }
    ///Location 1
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1LOC::Loc1)
    }
    ///Location 2
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1LOC::Loc2)
    }
    ///Location 3
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1LOC::Loc3)
    }
    ///Location 4
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1LOC::Loc4)
    }
    ///Location 5
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1LOC::Loc5)
    }
    ///Location 6
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1LOC::Loc6)
    }
    ///Location 7
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1LOC::Loc7)
    }
}
impl R {
    ///Bits 0:5 - I/O Location
    #[inline(always)]
    pub fn clkout0loc(&self) -> Clkout0locR {
        Clkout0locR::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:13 - I/O Location
    #[inline(always)]
    pub fn clkout1loc(&self) -> Clkout1locR {
        Clkout1locR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROUTELOC0")
            .field("clkout0loc", &self.clkout0loc())
            .field("clkout1loc", &self.clkout1loc())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - I/O Location
    #[inline(always)]
    #[must_use]
    pub fn clkout0loc(&mut self) -> Clkout0locW<ROUTELOC0rs> {
        Clkout0locW::new(self, 0)
    }
    ///Bits 8:13 - I/O Location
    #[inline(always)]
    #[must_use]
    pub fn clkout1loc(&mut self) -> Clkout1locW<ROUTELOC0rs> {
        Clkout1locW::new(self, 8)
    }
}
///I/O Routing Location Register
///
///You can [`read`](crate::Reg::read) this register and get [`routeloc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ROUTELOC0rs;
impl crate::RegisterSpec for ROUTELOC0rs {
    type Ux = u32;
}
///`read()` method returns [`routeloc0::R`](R) reader structure
impl crate::Readable for ROUTELOC0rs {}
///`write(|w| ..)` method takes [`routeloc0::W`](W) writer structure
impl crate::Writable for ROUTELOC0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ROUTELOC0 to value 0
impl crate::Resettable for ROUTELOC0rs {
    const RESET_VALUE: u32 = 0;
}
