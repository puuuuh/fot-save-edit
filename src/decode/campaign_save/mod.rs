use std::borrow::Cow;
use std::io::{BufRead, Read};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::{assert_section, dbg_str, read_primitive_vec, skip};
use crate::decode::error::ParseError;
use crate::decode::primitive::FOTString;
use crate::decode::saveh::Saveh;
use crate::decode::sections::world::World;
use crate::decode::stream::Stream;

const HEADER: &str = "<campaign_save>";

#[derive(Debug)]
pub struct CampaignSave {
    pub worlds: Vec<CampaignWorld>,
    pub save: Campaign
}

#[derive(Debug)]
pub struct CampaignWorld {
    pub file_name: FOTString,
    pub saveh: Saveh
}

#[derive(Debug)]
pub struct Campaign {
    pub file_name: FOTString,
    pub world_file: FOTString,
}

impl CampaignSave {
    pub fn read(mut data: &mut Stream) -> Result<Self, ParseError> {
        assert_section!(data, HEADER);
        data.skip(3);
        let cnt = data.read_u32()?;
        let mut res = vec![];
        let mut campaign = None;
        for _ in 0..cnt {
            let s = FOTString::read(&mut data)?;
            let len = data.read_u32()?;
            let name = (data.clone()).read_cstr()?; // kinda lookahead
            match &*name.to_bytes() {
                b"<saveh>" => {
                    let mut substream = data.clone();
                    let saveh = Saveh::read(&mut substream)?;
                    let world = World::read(&mut substream)?;
                    // TODO: Parse tail too
                    res.push(CampaignWorld {
                        file_name: s,
                        saveh,
                    });

                    data.skip(len as usize)?;
                }
                b"<campaign>" => {
                    data.skip(0x22C5)?;
                    let world_file = data.read_string()?;

                    data.skip(len as usize - 0x22C5 - world_file.serialized_length())?;

                    campaign = Some(Campaign {
                        file_name: s,
                        world_file
                    });
                }
                _ => {}
            }
        }
        Ok(Self {
            worlds: res,
            save: campaign.unwrap(),
        })
    }
}