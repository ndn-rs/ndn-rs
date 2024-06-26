use tlv::Tlv;
use tokio_util::codec::Decoder;

use super::*;

const P1: &[u8] = &[
    6, 253, 1, 27, 7, 46, 8, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 8, 3, 110, 102, 100, 8,
    6, 115, 116, 97, 116, 117, 115, 8, 7, 103, 101, 110, 101, 114, 97, 108, 54, 8, 0, 0, 1, 140,
    67, 46, 112, 8, 50, 1, 0, 20, 9, 25, 2, 3, 232, 26, 3, 50, 1, 0, 21, 85, 128, 18, 50, 50, 46,
    49, 50, 45, 51, 51, 45, 103, 101, 50, 55, 55, 102, 56, 98, 57, 129, 8, 0, 0, 1, 140, 51, 165,
    241, 48, 130, 8, 0, 0, 1, 140, 67, 46, 112, 8, 131, 1, 11, 132, 1, 2, 133, 1, 2, 134, 1, 0,
    135, 1, 2, 144, 2, 20, 191, 145, 2, 3, 230, 151, 1, 0, 146, 2, 20, 191, 147, 2, 3, 211, 152, 1,
    0, 153, 2, 3, 211, 154, 2, 16, 230, 22, 63, 27, 1, 3, 28, 58, 7, 56, 8, 9, 108, 111, 99, 97,
    108, 104, 111, 115, 116, 8, 7, 100, 97, 101, 109, 111, 110, 115, 8, 3, 110, 102, 100, 8, 3, 75,
    69, 89, 8, 8, 236, 225, 196, 116, 178, 135, 206, 56, 8, 4, 115, 101, 108, 102, 54, 8, 0, 0, 1,
    140, 16, 9, 111, 157, 23, 70, 48, 68, 2, 32, 27, 43, 29, 209, 53, 118, 16, 115, 224, 250, 31,
    15, 92, 109, 138, 64, 162, 142, 57, 116, 130, 238, 247, 33, 230, 126, 27, 122, 198, 27, 212,
    30, 2, 32, 49, 242, 60, 6, 201, 70, 78, 10, 105, 155, 243, 234, 81, 99, 45, 95, 155, 148, 108,
    107, 150, 54, 206, 64, 36, 21, 71, 250, 100, 63, 254, 121,
];

#[test]
fn decode() {
    use tlv::TlvCodec;
    let mut src = BytesMut::from(P1);
    let data = tlv::Data::decode(&mut src).unwrap();
    assert!(src.is_empty());
    assert_eq!(
        data.name(),
        "/localhost/nfd/status/general/v=1701934166024/seg=0"
    );
    assert!(data.metainfo.is_some());
    assert!(data.content.is_some());
}

#[test]
fn decode_value() {
    let mut src = P1.iter().collect();
    let mut codec = TlvCodec::new();
    let packet = codec.decode(&mut src).unwrap().unwrap();
    println!("{packet:?}");
    let tlv::Generic {
        r#type,
        length,
        mut value,
    } = packet;
    let length = length.to_usize();
    assert!(src.is_empty());
    assert_eq!(r#type, tlv::Type::Data);
    assert_eq!(length, 283);
    let data = tlv::Data::decode_value(r#type, length, &mut value).unwrap();
    // let data = tlv::Data::try_from(packet).unwrap();
    assert_eq!(
        data.name(),
        "/localhost/nfd/status/general/v=1701934166024/seg=0"
    );
    assert!(data.metainfo.is_some());
    assert!(data.content.is_some());
}
