use std::io::{Error, Write};
use crate::codec::Encodable;
use crate::codec::error::ParseError;
use crate::codec::sections::campaign::Campaign;
use crate::codec::stream::Stream;

#[derive(Debug)]
pub struct Cam<'a> {
    pub campaign: Campaign<'a>
}

impl<'a> Encodable<'a> for Cam<'a> {
    fn parse(data: &mut Stream<'a>) -> Result<Self, ParseError> {
        Ok(Self {
            campaign: Campaign::parse(data)?,
        })
    }

    fn write<T: Write>(&self, _stream: T) -> Result<(), Error> {
        self.campaign.write(_stream)
    }
}