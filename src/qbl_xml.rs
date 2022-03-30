use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use hex_literal::hex;

pub fn qbl_to_xml(qbl: &[u8]) -> Vec<u8> {
    let crypt = qbl_to_crypt(qbl);
    let lenxml = crypt_to_lenxml(&crypt);
    lenxml_to_xml(&lenxml)
}

pub fn xml_to_qbl(xml: &[u8]) -> Vec<u8> {
    let lenxml = xml_to_lenxml(xml);
    let crypt = lenxml_to_crypt(&lenxml);
    crypt_to_qbl(&crypt)
}

// conversion between .qbl format and plain encrypted data
// qbl format (is actually a more general format, but is only used with specific bytes)
// 7e 00 {len+7 as u32} 7f 01 {len as u32} {crypt as [u8;len]} 7b
fn qbl_to_crypt(qbl: &[u8]) -> &[u8] {
    // TODO validate first 12 and final byte against magic
    // eg qbl[8..12] as u32 == qbl.len()-13
    &qbl[12..qbl.len() - 1]
}

fn crypt_to_qbl(crypt: &[u8]) -> Vec<u8> {
    let mut qbl = Vec::with_capacity(13 + crypt.len());
    qbl.push(0x7e);
    qbl.push(0);
    // TODO verify crypt.len() + 7 <= U32::MAX
    push_u32(&mut qbl, (crypt.len() + 7) as u32);
    qbl.push(0x7f);
    qbl.push(1);
    push_u32(&mut qbl, crypt.len() as u32);
    qbl.extend_from_slice(crypt);
    qbl.push(0x7b);
    qbl
}

// conversion between plain enctrypted data and len tagged xml

// I dont realy know why Liquid Flower is using encryption (and not compression), but whatever...
const KEY: [u8; 16] = hex!("30 85 c1 24 9a 56 b6 30 79 67 5c 88 c8 8a dc ba");
const IV: [u8; 16] = hex!("df 86 4a 53 c4 68 c9 8f b4 a5 61 dc 14 ff 53 57");

fn crypt_to_lenxml(crypt: &[u8]) -> Vec<u8> {
    cbc::Decryptor::<aes::Aes128Dec>::new(&KEY.into(), &IV.into())
        .decrypt_padded_vec_mut::<aes::cipher::block_padding::Pkcs7>(crypt)
        .unwrap()
}

fn lenxml_to_crypt(lenxml: &[u8]) -> Vec<u8> {
    cbc::Encryptor::<aes::Aes128Enc>::new(&KEY.into(), &IV.into())
        .encrypt_padded_vec_mut::<aes::cipher::block_padding::Pkcs7>(lenxml)
}

// conversion between len tagged xml and plain xml

fn lenxml_to_xml(lenxml: &[u8]) -> Vec<u8> {
    // first byte with a 0 8th bit is the last byte of the length
    // TODO check the length is correct
    let mut i = 0;
    while lenxml[i] & 0x80 == 0x80 {
        i += 1;
    }
    lenxml[i + 1..].to_vec()
}

fn xml_to_lenxml(xml: &[u8]) -> Vec<u8> {
    let mut lenxml = Vec::with_capacity(5 + xml.len());
    push_usize(&mut lenxml, xml.len());
    lenxml.extend_from_slice(xml);
    lenxml
}

fn push_u32(vec: &mut Vec<u8>, v: u32) {
    vec.extend_from_slice(&v.to_le_bytes());
}

fn push_usize(vec: &mut Vec<u8>, mut len: usize) {
    while len > 0x7F {
        vec.push((0x80 | (len & 0x7F)) as u8);
        len >>= 7;
    }
    vec.push(len as u8);
}
