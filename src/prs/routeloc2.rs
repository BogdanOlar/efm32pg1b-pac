///Register `ROUTELOC2` reader
pub type R = crate::R<ROUTELOC2rs>;
///Register `ROUTELOC2` writer
pub type W = crate::W<ROUTELOC2rs>;
///I/O Location
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH8LOC {
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
    ///8: Location 8
    Loc8 = 8,
    ///9: Location 9
    Loc9 = 9,
    ///10: Location 10
    Loc10 = 10,
}
impl From<CH8LOC> for u8 {
    #[inline(always)]
    fn from(variant: CH8LOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH8LOC {
    type Ux = u8;
}
impl crate::IsEnum for CH8LOC {}
///Field `CH8LOC` reader - I/O Location
pub type Ch8locR = crate::FieldReader<CH8LOC>;
impl Ch8locR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH8LOC> {
        match self.bits {
            0 => Some(CH8LOC::Loc0),
            1 => Some(CH8LOC::Loc1),
            2 => Some(CH8LOC::Loc2),
            3 => Some(CH8LOC::Loc3),
            4 => Some(CH8LOC::Loc4),
            5 => Some(CH8LOC::Loc5),
            6 => Some(CH8LOC::Loc6),
            7 => Some(CH8LOC::Loc7),
            8 => Some(CH8LOC::Loc8),
            9 => Some(CH8LOC::Loc9),
            10 => Some(CH8LOC::Loc10),
            _ => None,
        }
    }
    ///Location 0
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH8LOC::Loc0
    }
    ///Location 1
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH8LOC::Loc1
    }
    ///Location 2
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH8LOC::Loc2
    }
    ///Location 3
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CH8LOC::Loc3
    }
    ///Location 4
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CH8LOC::Loc4
    }
    ///Location 5
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CH8LOC::Loc5
    }
    ///Location 6
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CH8LOC::Loc6
    }
    ///Location 7
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CH8LOC::Loc7
    }
    ///Location 8
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == CH8LOC::Loc8
    }
    ///Location 9
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == CH8LOC::Loc9
    }
    ///Location 10
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == CH8LOC::Loc10
    }
}
///Field `CH8LOC` writer - I/O Location
pub type Ch8locW<'a, REG> = crate::FieldWriter<'a, REG, 6, CH8LOC>;
impl<'a, REG> Ch8locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Location 0
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC::Loc0)
    }
    ///Location 1
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC::Loc1)
    }
    ///Location 2
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC::Loc2)
    }
    ///Location 3
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC::Loc3)
    }
    ///Location 4
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC::Loc4)
    }
    ///Location 5
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC::Loc5)
    }
    ///Location 6
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC::Loc6)
    }
    ///Location 7
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC::Loc7)
    }
    ///Location 8
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC::Loc8)
    }
    ///Location 9
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC::Loc9)
    }
    ///Location 10
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(CH8LOC::Loc10)
    }
}
///I/O Location
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH9LOC {
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
    ///8: Location 8
    Loc8 = 8,
    ///9: Location 9
    Loc9 = 9,
    ///10: Location 10
    Loc10 = 10,
    ///11: Location 11
    Loc11 = 11,
    ///12: Location 12
    Loc12 = 12,
    ///13: Location 13
    Loc13 = 13,
    ///14: Location 14
    Loc14 = 14,
    ///15: Location 15
    Loc15 = 15,
    ///16: Location 16
    Loc16 = 16,
}
impl From<CH9LOC> for u8 {
    #[inline(always)]
    fn from(variant: CH9LOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH9LOC {
    type Ux = u8;
}
impl crate::IsEnum for CH9LOC {}
///Field `CH9LOC` reader - I/O Location
pub type Ch9locR = crate::FieldReader<CH9LOC>;
impl Ch9locR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH9LOC> {
        match self.bits {
            0 => Some(CH9LOC::Loc0),
            1 => Some(CH9LOC::Loc1),
            2 => Some(CH9LOC::Loc2),
            3 => Some(CH9LOC::Loc3),
            4 => Some(CH9LOC::Loc4),
            5 => Some(CH9LOC::Loc5),
            6 => Some(CH9LOC::Loc6),
            7 => Some(CH9LOC::Loc7),
            8 => Some(CH9LOC::Loc8),
            9 => Some(CH9LOC::Loc9),
            10 => Some(CH9LOC::Loc10),
            11 => Some(CH9LOC::Loc11),
            12 => Some(CH9LOC::Loc12),
            13 => Some(CH9LOC::Loc13),
            14 => Some(CH9LOC::Loc14),
            15 => Some(CH9LOC::Loc15),
            16 => Some(CH9LOC::Loc16),
            _ => None,
        }
    }
    ///Location 0
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH9LOC::Loc0
    }
    ///Location 1
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH9LOC::Loc1
    }
    ///Location 2
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH9LOC::Loc2
    }
    ///Location 3
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CH9LOC::Loc3
    }
    ///Location 4
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CH9LOC::Loc4
    }
    ///Location 5
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CH9LOC::Loc5
    }
    ///Location 6
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CH9LOC::Loc6
    }
    ///Location 7
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CH9LOC::Loc7
    }
    ///Location 8
    #[inline(always)]
    pub fn is_loc8(&self) -> bool {
        *self == CH9LOC::Loc8
    }
    ///Location 9
    #[inline(always)]
    pub fn is_loc9(&self) -> bool {
        *self == CH9LOC::Loc9
    }
    ///Location 10
    #[inline(always)]
    pub fn is_loc10(&self) -> bool {
        *self == CH9LOC::Loc10
    }
    ///Location 11
    #[inline(always)]
    pub fn is_loc11(&self) -> bool {
        *self == CH9LOC::Loc11
    }
    ///Location 12
    #[inline(always)]
    pub fn is_loc12(&self) -> bool {
        *self == CH9LOC::Loc12
    }
    ///Location 13
    #[inline(always)]
    pub fn is_loc13(&self) -> bool {
        *self == CH9LOC::Loc13
    }
    ///Location 14
    #[inline(always)]
    pub fn is_loc14(&self) -> bool {
        *self == CH9LOC::Loc14
    }
    ///Location 15
    #[inline(always)]
    pub fn is_loc15(&self) -> bool {
        *self == CH9LOC::Loc15
    }
    ///Location 16
    #[inline(always)]
    pub fn is_loc16(&self) -> bool {
        *self == CH9LOC::Loc16
    }
}
///Field `CH9LOC` writer - I/O Location
pub type Ch9locW<'a, REG> = crate::FieldWriter<'a, REG, 6, CH9LOC>;
impl<'a, REG> Ch9locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Location 0
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC::Loc0)
    }
    ///Location 1
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC::Loc1)
    }
    ///Location 2
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC::Loc2)
    }
    ///Location 3
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC::Loc3)
    }
    ///Location 4
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC::Loc4)
    }
    ///Location 5
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC::Loc5)
    }
    ///Location 6
    #[inline(always)]
    pub fn loc6(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC::Loc6)
    }
    ///Location 7
    #[inline(always)]
    pub fn loc7(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC::Loc7)
    }
    ///Location 8
    #[inline(always)]
    pub fn loc8(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC::Loc8)
    }
    ///Location 9
    #[inline(always)]
    pub fn loc9(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC::Loc9)
    }
    ///Location 10
    #[inline(always)]
    pub fn loc10(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC::Loc10)
    }
    ///Location 11
    #[inline(always)]
    pub fn loc11(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC::Loc11)
    }
    ///Location 12
    #[inline(always)]
    pub fn loc12(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC::Loc12)
    }
    ///Location 13
    #[inline(always)]
    pub fn loc13(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC::Loc13)
    }
    ///Location 14
    #[inline(always)]
    pub fn loc14(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC::Loc14)
    }
    ///Location 15
    #[inline(always)]
    pub fn loc15(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC::Loc15)
    }
    ///Location 16
    #[inline(always)]
    pub fn loc16(self) -> &'a mut crate::W<REG> {
        self.variant(CH9LOC::Loc16)
    }
}
///I/O Location
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH10LOC {
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
}
impl From<CH10LOC> for u8 {
    #[inline(always)]
    fn from(variant: CH10LOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH10LOC {
    type Ux = u8;
}
impl crate::IsEnum for CH10LOC {}
///Field `CH10LOC` reader - I/O Location
pub type Ch10locR = crate::FieldReader<CH10LOC>;
impl Ch10locR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH10LOC> {
        match self.bits {
            0 => Some(CH10LOC::Loc0),
            1 => Some(CH10LOC::Loc1),
            2 => Some(CH10LOC::Loc2),
            3 => Some(CH10LOC::Loc3),
            4 => Some(CH10LOC::Loc4),
            5 => Some(CH10LOC::Loc5),
            _ => None,
        }
    }
    ///Location 0
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH10LOC::Loc0
    }
    ///Location 1
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH10LOC::Loc1
    }
    ///Location 2
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH10LOC::Loc2
    }
    ///Location 3
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CH10LOC::Loc3
    }
    ///Location 4
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CH10LOC::Loc4
    }
    ///Location 5
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CH10LOC::Loc5
    }
}
///Field `CH10LOC` writer - I/O Location
pub type Ch10locW<'a, REG> = crate::FieldWriter<'a, REG, 6, CH10LOC>;
impl<'a, REG> Ch10locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Location 0
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH10LOC::Loc0)
    }
    ///Location 1
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH10LOC::Loc1)
    }
    ///Location 2
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH10LOC::Loc2)
    }
    ///Location 3
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CH10LOC::Loc3)
    }
    ///Location 4
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(CH10LOC::Loc4)
    }
    ///Location 5
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(CH10LOC::Loc5)
    }
}
///I/O Location
///
///Value on reset: 0
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CH11LOC {
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
}
impl From<CH11LOC> for u8 {
    #[inline(always)]
    fn from(variant: CH11LOC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CH11LOC {
    type Ux = u8;
}
impl crate::IsEnum for CH11LOC {}
///Field `CH11LOC` reader - I/O Location
pub type Ch11locR = crate::FieldReader<CH11LOC>;
impl Ch11locR {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CH11LOC> {
        match self.bits {
            0 => Some(CH11LOC::Loc0),
            1 => Some(CH11LOC::Loc1),
            2 => Some(CH11LOC::Loc2),
            3 => Some(CH11LOC::Loc3),
            4 => Some(CH11LOC::Loc4),
            5 => Some(CH11LOC::Loc5),
            _ => None,
        }
    }
    ///Location 0
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH11LOC::Loc0
    }
    ///Location 1
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH11LOC::Loc1
    }
    ///Location 2
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH11LOC::Loc2
    }
    ///Location 3
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CH11LOC::Loc3
    }
    ///Location 4
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CH11LOC::Loc4
    }
    ///Location 5
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CH11LOC::Loc5
    }
}
///Field `CH11LOC` writer - I/O Location
pub type Ch11locW<'a, REG> = crate::FieldWriter<'a, REG, 6, CH11LOC>;
impl<'a, REG> Ch11locW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Location 0
    #[inline(always)]
    pub fn loc0(self) -> &'a mut crate::W<REG> {
        self.variant(CH11LOC::Loc0)
    }
    ///Location 1
    #[inline(always)]
    pub fn loc1(self) -> &'a mut crate::W<REG> {
        self.variant(CH11LOC::Loc1)
    }
    ///Location 2
    #[inline(always)]
    pub fn loc2(self) -> &'a mut crate::W<REG> {
        self.variant(CH11LOC::Loc2)
    }
    ///Location 3
    #[inline(always)]
    pub fn loc3(self) -> &'a mut crate::W<REG> {
        self.variant(CH11LOC::Loc3)
    }
    ///Location 4
    #[inline(always)]
    pub fn loc4(self) -> &'a mut crate::W<REG> {
        self.variant(CH11LOC::Loc4)
    }
    ///Location 5
    #[inline(always)]
    pub fn loc5(self) -> &'a mut crate::W<REG> {
        self.variant(CH11LOC::Loc5)
    }
}
impl R {
    ///Bits 0:5 - I/O Location
    #[inline(always)]
    pub fn ch8loc(&self) -> Ch8locR {
        Ch8locR::new((self.bits & 0x3f) as u8)
    }
    ///Bits 8:13 - I/O Location
    #[inline(always)]
    pub fn ch9loc(&self) -> Ch9locR {
        Ch9locR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:21 - I/O Location
    #[inline(always)]
    pub fn ch10loc(&self) -> Ch10locR {
        Ch10locR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:29 - I/O Location
    #[inline(always)]
    pub fn ch11loc(&self) -> Ch11locR {
        Ch11locR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROUTELOC2")
            .field("ch8loc", &self.ch8loc())
            .field("ch9loc", &self.ch9loc())
            .field("ch10loc", &self.ch10loc())
            .field("ch11loc", &self.ch11loc())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - I/O Location
    #[inline(always)]
    #[must_use]
    pub fn ch8loc(&mut self) -> Ch8locW<ROUTELOC2rs> {
        Ch8locW::new(self, 0)
    }
    ///Bits 8:13 - I/O Location
    #[inline(always)]
    #[must_use]
    pub fn ch9loc(&mut self) -> Ch9locW<ROUTELOC2rs> {
        Ch9locW::new(self, 8)
    }
    ///Bits 16:21 - I/O Location
    #[inline(always)]
    #[must_use]
    pub fn ch10loc(&mut self) -> Ch10locW<ROUTELOC2rs> {
        Ch10locW::new(self, 16)
    }
    ///Bits 24:29 - I/O Location
    #[inline(always)]
    #[must_use]
    pub fn ch11loc(&mut self) -> Ch11locW<ROUTELOC2rs> {
        Ch11locW::new(self, 24)
    }
}
///I/O Routing Location Register
///
///You can [`read`](crate::Reg::read) this register and get [`routeloc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`routeloc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct ROUTELOC2rs;
impl crate::RegisterSpec for ROUTELOC2rs {
    type Ux = u32;
}
///`read()` method returns [`routeloc2::R`](R) reader structure
impl crate::Readable for ROUTELOC2rs {}
///`write(|w| ..)` method takes [`routeloc2::W`](W) writer structure
impl crate::Writable for ROUTELOC2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ROUTELOC2 to value 0
impl crate::Resettable for ROUTELOC2rs {
    const RESET_VALUE: u32 = 0;
}
