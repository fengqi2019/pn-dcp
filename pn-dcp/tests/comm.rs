#![allow(dead_code)]
use anyhow::{anyhow, bail, Result};
use pnet::datalink;
use pnet::datalink::interfaces;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{Config, MacAddr, NetworkInterface};

pub fn get_ident_req() -> Vec<u8> {
    vec![
        0x01, 0x0e, 0xcf, 0x00, 0x00, 0x00, 0xec, 0x63, 0xd7, 0x5d, 0x5d, 0x22, 0x88, 0x92, 0xfe,
        0xfe, 0x05, 0x00, 0x0f, 0x01, 0x00, 0x0c, 0x00, 0x80, 0x00, 0x10, 0x02, 0x01, 0x00, 0x0c,
        0x53, 0x37, 0x2d, 0x32, 0x30, 0x30, 0x20, 0x53, 0x4d, 0x41, 0x52, 0x54,
    ]
}
pub fn get_ident_resp() -> Vec<u8> {
    vec![
        0xec, 0x63, 0xd7, 0x5d, 0x5d, 0x22, 0x00, 0x1c, 0x06, 0x11, 0x42, 0x02, 0x88, 0x92, 0xfe,
        0xff, 0x05, 0x01, 0x0f, 0x01, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x60, 0x02, 0x01, 0x00, 0x0e,
        0x00, 0x00, 0x53, 0x37, 0x2d, 0x32, 0x30, 0x30, 0x20, 0x53, 0x4d, 0x41, 0x52, 0x54, 0x02,
        0x05, 0x00, 0x14, 0x00, 0x00, 0x02, 0x01, 0x02, 0x02, 0x02, 0x03, 0x02, 0x04, 0x02, 0x05,
        0x02, 0x06, 0x01, 0x01, 0x01, 0x02, 0x03, 0x3d, 0x02, 0x02, 0x00, 0x0d, 0x00, 0x00, 0x62,
        0x62, 0x2d, 0x61, 0x62, 0x63, 0x69, 0x2e, 0x31, 0x31, 0x31, 0x00, 0x02, 0x03, 0x00, 0x06,
        0x00, 0x00, 0x00, 0x2a, 0x00, 0x00, 0x02, 0x04, 0x00, 0x04, 0x00, 0x00, 0x02, 0x00, 0x01,
        0x02, 0x00, 0x0e, 0x00, 0x01, 0xc0, 0xa8, 0xc7, 0xf5, 0xff, 0xff, 0xff, 0x00, 0xc0, 0xa8,
        0xc7, 0xfe,
    ]
}

pub fn get_get_req() -> Vec<u8> {
    vec![
        0x00, 0x1c, 0x06, 0x11, 0x42, 0x02, 0xec, 0x63, 0xd7, 0x5d, 0x5d, 0x22, 0x88, 0x92, 0xfe,
        0xfd, 0x03, 0x00, 0x0f, 0x02, 0x00, 0x14, 0x00, 0x00, 0x00, 0x04, 0x01, 0x02, 0x03, 0x3d,
    ]
}
pub fn get_get_resp() -> Vec<u8> {
    vec![
        0xec, 0x63, 0xd7, 0x5d, 0x5d, 0x22, 0x00, 0x1c, 0x06, 0x11, 0x42, 0x02, 0x88, 0x92, 0xfe,
        0xfd, 0x03, 0x01, 0x0f, 0x02, 0x00, 0x14, 0x00, 0x00, 0x00, 0x1a, 0x01, 0x02, 0x00, 0x0e,
        0x00, 0x01, 0xc0, 0xa8, 0xc7, 0xf5, 0xff, 0xff, 0xff, 0x00, 0xc0, 0xa8, 0xc7, 0xfe, 0x05,
        0x04, 0x00, 0x03, 0x03, 0x3d, 0x02, 0x00,
    ]
}
pub fn get_set_req() -> Vec<u8> {
    vec![
        0x00, 0x1c, 0x06, 0x11, 0x42, 0x02, 0xec, 0x63, 0xd7, 0x5d, 0x5d, 0x22, 0x88, 0x92, 0xfe,
        0xfd, 0x04, 0x00, 0x0f, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x12, 0x01, 0x02, 0x00, 0x0e,
        0x00, 0x01, 0xc0, 0xa8, 0xc7, 0xf5, 0xff, 0xff, 0xff, 0x00, 0xc0, 0xa8, 0xc7, 0xfe,
    ]
}
pub fn get_set_resp() -> Vec<u8> {
    vec![
        0xec, 0x63, 0xd7, 0x5d, 0x5d, 0x22, 0x00, 0x1c, 0x06, 0x11, 0x42, 0x02, 0x88, 0x92, 0xfe,
        0xfd, 0x04, 0x01, 0x0f, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x08, 0x05, 0x04, 0x00, 0x03,
        0x01, 0x02, 0x00, 0x00,
    ]
}

pub fn get_block() -> Vec<u8> {
    vec![
        0x02u8, 0x01, 0x00, 0x0c, 0x53, 0x37, 0x2d, 0x32, 0x30, 0x30, 0x20, 0x53, 0x4d, 0x41, 0x52,
        0x54,
    ]
}
pub fn get_block_with_padding() -> Vec<u8> {
    vec![
        0x02u8, 0x01, 0x00, 0x0c, 0x53, 0x37, 0x2d, 0x32, 0x30, 0x30, 0x20, 0x53, 0x4d, 0x41, 0x52,
        0x54, 0x00,
    ]
}

pub fn get_destination_mac(data: &[u8]) -> Result<&[u8]> {
    data.get(0..=5).ok_or(anyhow!("数组越界"))
}
pub fn get_destination_array(data: &[u8]) -> Result<[u8; 6]> {
    if data.len() <= 5 {
        bail!("数组越界")
    } else {
        Ok([data[0], data[1], data[2], data[3], data[4], data[5]])
    }
}
pub fn get_src_mac(data: &[u8]) -> Result<&[u8]> {
    data.get(6..=11).ok_or(anyhow!("数组越界"))
}
pub fn get_src_array(data: &[u8]) -> Result<[u8; 6]> {
    if data.len() <= 11 {
        bail!("数组越界")
    } else {
        Ok([data[6], data[7], data[8], data[9], data[10], data[11]])
    }
}
pub fn get_ethernet_type(data: &[u8]) -> Result<&[u8]> {
    data.get(12..=13).ok_or(anyhow!("数组越界"))
}
pub fn get_frame_id(data: &[u8]) -> Result<[u8; 2]> {
    if data.len() <= 15 {
        bail!("数组越界")
    } else {
        Ok([data[14], data[15]])
    }
}
pub fn get_service_id(data: &[u8]) -> Result<&u8> {
    data.get(16).ok_or(anyhow!("数组越界"))
}
pub fn get_service_type(data: &[u8]) -> Result<&u8> {
    data.get(17).ok_or(anyhow!("数组越界"))
}

pub fn get_xid(data: &[u8]) -> Result<[u8; 4]> {
    if data.len() <= 21 {
        bail!("数组越界")
    } else {
        Ok([data[18], data[19], data[20], data[21]])
    }
    // data.get(18..=21)
    //     .and_then(|x| Ok([x[0], x[1], x[2], x[3]]))
    //     .ok_or()
}
pub fn get_response_delay(data: &[u8]) -> Result<u16> {
    if data.len() <= 23 {
        bail!("数组越界")
    } else {
        Ok(u16::from_be_bytes([data[22], data[23]]))
    }
    // data.get(22..=23).ok_or(anyhow!("数组越界"))
}
pub fn get_dcp_data_length(data: &[u8]) -> Result<&[u8]> {
    data.get(24..=25).ok_or(anyhow!("数组越界"))
}
pub fn get_blocks(data: &[u8]) -> Result<&[u8]> {
    if data.len() <= 27 {
        bail!("数组越界")
    } else {
        let mut size = u16::from_be_bytes([data[24], data[25]]) as usize;
        size += 25;
        Ok(&data[26..=size])
    }
}

pub fn init_mac_by_array(src: [u8; 6]) -> MacAddr {
    MacAddr::new(src[0], src[1], src[2], src[3], src[4], src[5])
}
#[test]
fn test() {
    let left = [0u8, 0u8, 1, 2, 3, 4, 5, 6];
    let right = vec![0u8, 1, 2, 3, 4, 5, 6];
    assert_eq!(get_destination_mac(right.as_slice()).unwrap(), &left[1..7]);
}

pub fn tx_data(data: Vec<u8>) -> Result<()> {
    let index: u32 = 21;
    let interface = get_interface(index)?;
    if let Some(_src) = interface.mac {
        let cf = Config::default();
        let (mut tx, _rx) = match datalink::channel(&interface, cf) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => bail!("Unhandled channel type"),
            Err(e) => bail!(
                "An error occurred when creating the datalink channel: {}",
                e
            ),
        };
        if let Some(Err(e)) = tx.send_to(data.as_slice(), Some(interface.clone())) {
            bail!("error: {:?}", e);
        }
    } else {
        panic!("");
    }
    Ok(())
}
pub fn get_interface(index: u32) -> Result<NetworkInterface> {
    for interface in interfaces() {
        println!("{:?} {}", interface.ips, interface.index);
    }
    for interface in interfaces() {
        if interface.index == index {
            return Ok(interface);
        }
    }
    bail!("不存在【index={}】的网络接口", index);
}
