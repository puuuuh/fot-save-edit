use crate::codec::error::ParseError;
use crate::codec::sections::saveh::Saveh;
use crate::codec::sections::world::World;
use crate::codec::stream::Stream;
use crate::codec::Encodable;
use std::io::{Error, Write};

#[derive(Debug)]
pub struct Sav<'a> {
    pub saveh: Saveh<'a>,
    pub world: World<'a>,
}

impl<'a> Encodable<'a> for Sav<'a> {
    fn parse(data: &mut Stream<'a>) -> Result<Self, ParseError> {
        let saveh = Saveh::parse(data)?;
        let world = World::parse(data)?;

        Ok(Sav { saveh, world })
    }

    fn write<T: Write>(&self, mut stream: T) -> Result<(), Error> {
        self.saveh.write(&mut stream)?;
        self.world.write(&mut stream)?;
        Ok(())
    }
}
