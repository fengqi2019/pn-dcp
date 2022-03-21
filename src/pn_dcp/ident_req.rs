use crate::comm::{to_u16, BytesWrap};
use crate::consts::PROFINET_ETHER_TYPE;
use crate::dcp_block::{BlockCommonWithoutInfo, BlockPadding};
use crate::options::OptionAndSubValue;
use crate::pn_dcp::{DcgHead, PnDcg, PnDcpTy};
use anyhow::{bail, Result};
use bytes::Bytes;
use pnet::util::MacAddr;

#[derive(Debug)]
pub enum IdentReqBlock {
    Block(BlockCommonWithoutInfo),
    Padding(BlockPadding),
}
#[derive(Debug)]
pub struct IdentReqBlocks(pub Vec<IdentReqBlock>);

impl TryFrom<BytesWrap> for IdentReqBlocks {
    type Error = anyhow::Error;

    fn try_from(value: BytesWrap) -> Result<Self, Self::Error> {
        let mut index = 0usize;
        let mut blocks = Vec::<IdentReqBlock>::new();
        println!("{:?}", value);
        while let Ok(tmp) = value.slice(index..) {
            let one = BlockCommonWithoutInfo::try_from(tmp)?;
            let len = one.len();
            if one.len() % 2 == 1 {
                blocks.push(one.into());
                blocks.push(BlockPadding.into());
                index += 1;
            } else {
                blocks.push(one.into());
            }
            index += len;
        }
        Ok(blocks.into())
    }
}

pub struct PacketIdentReq {
    pub head: DcgHead,
    pub blocks: IdentReqBlocks,
}

impl PacketIdentReq {
    pub fn get_manufacturer_pecific_block(&self) -> Result<BytesWrap> {
        for block in self.blocks.0.as_slice() {
            if let IdentReqBlock::Block(BlockCommonWithoutInfo(
                OptionAndSubValue::ManufacturerSpecific(data),
            )) = block
            {
                return Ok(data.clone());
            }
        }
        bail!("todo 未包含manufacturer_pecific信息")
    }
}

impl TryFrom<PnDcg> for PacketIdentReq {
    type Error = anyhow::Error;

    fn try_from(dcg: PnDcg) -> Result<Self, Self::Error> {
        let PnDcg { head, blocks } = dcg;
        if head.ty != PnDcpTy::IdentReq {
            bail!("todo");
        }
        let blocks = IdentReqBlocks::try_from(blocks)?;
        Ok(Self { blocks, head })
    }
}

impl TryFrom<&[u8]> for PacketIdentReq {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let dcg = PnDcg::try_from(value)?;
        PacketIdentReq::try_from(dcg)
    }
}
