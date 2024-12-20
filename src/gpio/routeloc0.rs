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
pub enum SWVLOC {
    ///0: Location 0
    Loc0 = 0,
    ///1: Location 1
    Loc1 = 1,
    ///2: Location 2
    Loc2 = 2,
    ///3: Location 3
    Loc3 = 3,
}
impl From<SWVLOC> for u8 {
    #[inline(always)]
    fn from(variant: SWVLOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SWVLOC {
    type Ux = u8;
}
impl crate::IsEnum for SWVLOC {}
///Field `SWVLOC` reader - I/O Location
pub type SwvlocR = crate::FieldReader<SWVLOC>;
impl SwvlocR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SWVLOC> {
        match self.bits {
            0 => Some(SWVLOC::Loc0),
            1 => Some(SWVLOC::Loc1),
            2 => Some(SWVLOC::Loc2),
            3 => Some(SWVLOC::Loc3),
            _ => None,
        }
    }
    ///Location 0
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == SWVLOC::Loc0
    }
    ///Location 1
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == SWVLOC::Loc1
    }
    ///Location 2
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == SWVLOC::Loc2
    }
    ///Location 3
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == SWVLOC::Loc3
    }
}
///Field `SWVLOC` writer - I/O Location
pub type SwvlocW<'a, REG> = crate::FieldWriter<'a, REG, 6, SWVLOC>;
impl<'a, REG> SwvlocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Location 0
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(SWVLOC::Loc0)
    }
    ///Location 1
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(SWVLOC::Loc1)
    }
    ///Location 2
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(SWVLOC::Loc2)
    }
    ///Location 3
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(SWVLOC::Loc3)
    }
}
impl R {
    ///Bits 0:5 - I/O Location
    #[inline(always)]
    pub fn swvloc(&self) -> SwvlocR {
        SwvlocR::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROUTELOC0")
            .field("swvloc", &self.swvloc())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - I/O Location
    #[inline(always)]
    #[must_use]
    pub fn swvloc(&mut self) -> SwvlocW<ROUTELOC0rs> {
        SwvlocW::new(self, 0)
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
