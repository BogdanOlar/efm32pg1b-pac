///Register `LINK` reader
pub type R = crate::R<LINKrs>;
///Register `LINK` writer
pub type W = crate::W<LINKrs>;
///Field `LINKMODE` reader - Link Structure Addressing Mode
pub type LinkmodeR = crate::BitReader;
///Field `LINK` reader - Link Next Structure
pub type LinkR = crate::BitReader;
///Field `LINK` writer - Link Next Structure
pub type LinkW<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINKADDR` reader - Link Structure Address
pub type LinkaddrR = crate::FieldReader<u32>;
///Field `LINKADDR` writer - Link Structure Address
pub type LinkaddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bit 0 - Link Structure Addressing Mode
    #[inline(always)]
    pub fn linkmode(&self) -> LinkmodeR {
        LinkmodeR::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Link Next Structure
    #[inline(always)]
    pub fn link(&self) -> LinkR {
        LinkR::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:31 - Link Structure Address
    #[inline(always)]
    pub fn linkaddr(&self) -> LinkaddrR {
        LinkaddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LINK")
            .field("linkmode", &self.linkmode())
            .field("link", &self.link())
            .field("linkaddr", &self.linkaddr())
            .finish()
    }
}
impl W {
    ///Bit 1 - Link Next Structure
    #[inline(always)]
    pub fn link(&mut self) -> LinkW<'_, LINKrs> {
        LinkW::new(self, 1)
    }
    ///Bits 2:31 - Link Structure Address
    #[inline(always)]
    pub fn linkaddr(&mut self) -> LinkaddrW<'_, LINKrs> {
        LinkaddrW::new(self, 2)
    }
}
///Channel Descriptor Link Structure Address Register
///
///You can [`read`](crate::Reg::read) this register and get [`link::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).
pub struct LINKrs;
impl crate::RegisterSpec for LINKrs {
    type Ux = u32;
}
///`read()` method returns [`link::R`](R) reader structure
impl crate::Readable for LINKrs {}
///`write(|w| ..)` method takes [`link::W`](W) writer structure
impl crate::Writable for LINKrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LINK to value 0
impl crate::Resettable for LINKrs {}
