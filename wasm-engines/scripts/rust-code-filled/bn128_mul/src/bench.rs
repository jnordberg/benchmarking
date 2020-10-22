extern crate ethereum_bn128;

pub fn bench() {

    // let input: [u8; 96] = [ 3u8, 151u8, 48u8, 234u8, 141u8, 255u8, 18u8, 84u8, 192u8, 254u8, 233u8, 192u8, 234u8, 119u8, 125u8, 41u8, 169u8, 199u8, 16u8, 183u8, 230u8, 22u8, 104u8, 63u8, 25u8, 79u8, 24u8, 196u8, 59u8, 67u8, 184u8, 105u8, 7u8, 58u8, 95u8, 252u8, 198u8, 252u8, 122u8, 40u8, 195u8, 7u8, 35u8, 214u8, 229u8, 140u8, 229u8, 119u8, 53u8, 105u8, 130u8, 214u8, 91u8, 131u8, 58u8, 90u8, 92u8, 21u8, 191u8, 144u8, 36u8, 180u8, 61u8, 152u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8 ];

    let input: [u8; 96] = [ 3u8, 151u8, 48u8, 234u8, 141u8, 255u8, 18u8, 84u8, 192u8, 254u8, 233u8, 192u8, 234u8, 119u8, 125u8, 41u8, 169u8, 199u8, 16u8, 183u8, 230u8, 22u8, 104u8, 63u8, 25u8, 79u8, 24u8, 196u8, 59u8, 67u8, 184u8, 105u8, 7u8, 58u8, 95u8, 252u8, 198u8, 252u8, 122u8, 40u8, 195u8, 7u8, 35u8, 214u8, 229u8, 140u8, 229u8, 119u8, 53u8, 105u8, 130u8, 214u8, 91u8, 131u8, 58u8, 90u8, 92u8, 21u8, 191u8, 144u8, 36u8, 180u8, 61u8, 152u8, 48u8, 100u8, 78u8, 114u8, 225u8, 49u8, 160u8, 41u8, 184u8, 80u8, 69u8, 182u8, 129u8, 129u8, 88u8, 93u8, 40u8, 51u8, 232u8, 72u8, 121u8, 185u8, 112u8, 145u8, 67u8, 225u8, 245u8, 147u8, 240u8, 0u8, 0u8, 0u8 ];

    // let expected: [u8; 64] = [ 0u8, 161u8, 162u8, 52u8, 208u8, 142u8, 250u8, 162u8, 97u8, 102u8, 7u8, 227u8, 30u8, 202u8, 25u8, 128u8, 18u8, 139u8, 0u8, 180u8, 21u8, 200u8, 69u8, 255u8, 37u8, 187u8, 163u8, 175u8, 203u8, 129u8, 220u8, 0u8, 36u8, 32u8, 119u8, 41u8, 14u8, 211u8, 57u8, 6u8, 174u8, 184u8, 228u8, 47u8, 217u8, 140u8, 65u8, 188u8, 185u8, 5u8, 123u8, 160u8, 52u8, 33u8, 175u8, 63u8, 45u8, 8u8, 207u8, 196u8, 65u8, 24u8, 96u8, 36u8 ];

    let expected: [u8; 64] = [ 3u8, 151u8, 48u8, 234u8, 141u8, 255u8, 18u8, 84u8, 192u8, 254u8, 233u8, 192u8, 234u8, 119u8, 125u8, 41u8, 169u8, 199u8, 16u8, 183u8, 230u8, 22u8, 104u8, 63u8, 25u8, 79u8, 24u8, 196u8, 59u8, 67u8, 184u8, 105u8, 41u8, 41u8, 238u8, 118u8, 26u8, 53u8, 38u8, 0u8, 245u8, 73u8, 33u8, 223u8, 155u8, 244u8, 114u8, 230u8, 98u8, 23u8, 231u8, 187u8, 12u8, 238u8, 144u8, 50u8, 224u8, 10u8, 204u8, 134u8, 179u8, 200u8, 191u8, 175u8 ];

    let mut output = [0u8; 64];
    match ethereum_bn128::bn128_mul(&input[..], &mut output) {
        Ok(_) => {
            if !(output).eq(expected.as_ref()) {
                panic!("crash and burn");
            }
        },
        Err(_) => panic!(),
    }

}