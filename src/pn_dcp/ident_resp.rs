use crate::comm::{to_u16, BytesWrap};
use crate::consts::PROFINET_ETHER_TYPE;
use crate::dcp_block::{BlockCommon, BlockCommonWithoutInfo, BlockIp, BlockPadding};
use crate::options::{OptionAndSub, OptionAndSubValue};
use crate::pn_dcp::{DcgHead, PnDcg, PnDcpTy};
use anyhow::{bail, Result};
use bytes::Bytes;
use pnet::util::MacAddr;

#[derive(Debug)]
pub enum IdentRespBlock {
    Block(BlockCommon),
    BlockIp(BlockIp),
    Padding(BlockPadding),
}
#[derive(Debug)]
pub struct IdentRespBlocks(pub Vec<IdentRespBlock>);

impl TryFrom<BytesWrap> for IdentRespBlocks {
    type Error = anyhow::Error;

    fn try_from(value: BytesWrap) -> Result<Self, Self::Error> {
        let mut index = 0usize;
        let mut blocks = Vec::<IdentRespBlock>::new();
        println!("{:?}", value);
        while let Ok(tmp) = value.slice(index..) {
            println!("{:?}", tmp.as_ref());
            let option = OptionAndSub::try_from(tmp.clone())?;
            let len = match option {
                OptionAndSub::IpAddr => {
                    let block = BlockIp::try_from_bytes(tmp)?;
                    println!("{:?}", block);
                    let len = block.len();
                    blocks.push(block.into());
                    len
                }
                OptionAndSub::Response => {
                    bail!("todo OptionAndSub::Response")
                }
                option => {
                    let block = BlockCommon::try_from_bytes(option, tmp)?;
                    println!("{:?}", block);
                    let len = block.len();
                    blocks.push(block.into());
                    len
                }
            };
            if len % 2 == 1 {
                blocks.push(BlockPadding.into());
                index += 1;
            }
            index += len;
        }
        Ok(blocks.into())
    }
}

pub struct PacketIdentResp {
    pub head: DcgHead,
    pub blocks: IdentRespBlocks,
}

impl TryFrom<PnDcg> for PacketIdentResp {
    type Error = anyhow::Error;

    fn try_from(dcg: PnDcg) -> Result<Self, Self::Error> {
        let PnDcg { head, blocks } = dcg;
        if head.ty != PnDcpTy::IdentRespSuc {
            bail!("todo");
        }
        let blocks = IdentRespBlocks::try_from(blocks)?;
        Ok(Self { blocks, head })
    }
}

impl TryFrom<&[u8]> for PacketIdentResp {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let dcg = PnDcg::try_from(value)?;
        PacketIdentResp::try_from(dcg)
    }
}
