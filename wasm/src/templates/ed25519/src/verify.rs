use ed25519_dalek::{PublicKey, Signature};
use sha2::Sha512;

#[derive(Debug)]
pub struct Error(pub &'static str);

impl From<&'static str> for self::Error {
    fn from(val: &'static str) -> Self {
        self::Error(val)
    }
}

pub fn verify(input: &[u8; 128]) -> Result<bool, self::Error> {
    let message = &input[..32];

    let public_key =
        PublicKey::from_bytes(&input[32..64]).expect("public key should be correctly formed");
    let sig = Signature::from_bytes(&input[64..128]).expect("signature should be correctly formed");

    Ok(public_key.verify::<Sha512>(&message, &sig).is_ok())
}

#[cfg(test)]
mod tests {
    extern crate hex;
    use super::*;

    // TODO : add test case for message 0x0000....000

    #[test]
    fn valid() {
        let inputs = [
      "7557f01a255ac98a325a1bfaae484ba11b01368e299c3b8d9370180750823bead94430b019f29bc78bdd2a9cbe796c9b633ad5cbe4ac804ac118563790e4c657a978c89e1b83c4a2f337a77b10351d0e184089fecf0ea8870614570306edd4a9ece7cdbc3378978bf4a4f85c0252ebb51cf00b2a4fc23833c6a1f6875503ae08",
      "00cd4bfe82f3de702d6c7bdb03fb5e198b2c919d511da05340d9e9a8d19f23a36f584bdc06e318b65ebc19e9770f82688686c3301a46cda717a6f5d9799708fd66849b045d01b50976f25e092d82fd85c932ef28294e8270a3abdc98f4c8836aa3929002a0c7a33b4c8d9bb422cce5cd52954ade1228f1552694bb6f9f02fd0a",
      "6eab91e7e0497708974c04cd7a92ac5984c751ff314b8ffdedf9e6557891dd9ba8f3e79b5fd9518562af18fceab88dd334a2217a21a473d67f1c8f2ff0d6aa3fbf75a63eb07cb71af0dad5498cb85dee04ee09b52f08810e5438cf504d366e8d79ff7713a37b4aad2d71b5919e009f29dbed7000216d5c30a332d034bd6d5306",
      "09168da69e98a320abaf4d6800f818c3ac184a23dea24eea9bf9b562ab5cad1a62e2dd9d14b23ad80213b4ffd2bd11b9778464b95e5fb80edddecefe98ab01759916665e5211dae56d0c56cf55325785eb0415a2024a819a8503a1f272b4870df143f644ffb6709e68cd2a4899c638f24c7d5ab0a97a5a609b0013f8e1a8e80c",
      "328dc5e413547ab2a880cc31799d56fbbd74e3cd189b03c23b522f3ca6d0c4caa8348e1940ca6909ccd24c94ba3fdff06ce00be8bdf803c6b8e55894204260b0c3ca89cc9d010e455df065173ee9c2a46ac009da91bc47fd57f0b52fa83af58dab8d2159ddadc940b3f05f7f9cc3773336bda31b886130ef0b835b9a7e4df504",
      "3f86ab0c079174566336ecdc9dee65122cab4509f214023394cf20935a2f652e5a087ae6914888b6477a1abfc7a2a882c94c4be232cf74c7ea17a3536425cddad455185ec6f66f81437fe231018f7e2f45e053c44a6faebb7c66c12028e4bd1d24e8657bbc5c7472a0122eec7e5621cd60d6e8a4b546297fd98895df0bd11b0a",
      "35bf580db70fef11c8f207035a862c24e9190105f88c4337157d71dbd6ddcf6d3f8ef5ec4351210fb645adb3527ab354770fcc6ad1256ea383125c4f74fd254dd64d94983335d9aacae84b2803a44c28d92e2a119c31d62e8cab8fd453d712b50044be7e9c540330cd4a164729b49b50d1f80084568e4637afbd4a66e2090e00",
      "6506e556d2decaa0e79ecc7c4418942f5e53f0724479a5d2de7e8745e28f51b5e4485ef638bf72054adec057c18a7caa92739d83cb8b80a2486d65fe2573162a94400812e2840600f98ee6026ff492f3130f189c080073ec553599e1916084ef7257ad11c3685cfcc0c172b265947cac4c786877fc57ec7ebcc387a042b3200e",
      "815266a579c22b95366b9346f22b221d549e61b8480a2f6dbb093b3ab395758e09101ce0c831a172538acf0b3ade4f2e1468e31d71c7c959e8bb13acde91fddd9a17a8adbb390c51366b089efcfedb35b6a711ce890c86e25a99e59fd0590af82facfede4c321877004ee35c5069c1f54fea8561fdf159c6362d55eb8c0f3c0a",
      "1110f6e4c25cbc9453a02d8de442496f8fab9e9f5a989e648720ec715f43a14690708b09728172402cf857d3545ecf4882d3c0828d79ebf20360b7371da795af7a0df44510016c03ce137524349f422f399434e7365e5c6b904d466ddedd0639a5348413c758b8a5fa4a9c4adaf25e1c01f480fa9105c6820307df7336563a0b",
      "6b8567b18b4dfa882b92970c38d6568c104e0e69bba71cb45a1ab620bccf37bb9409cb3e4cf0983a40cd2ce8b98f4fe8f80b0a6bbbf9a711ff407aea383b29f5447fc9d754f8af11971771e2870431e956af319548618148bcfb918d4a030582ada2cfd9a051bac4d56c8c880333fdcf1071562abe05c5c33ed06ec2745d010c",
    ];
        for i in 0..inputs.len() {
            let input = hex::decode(inputs[i]).unwrap();
            let mut tmp = [0u8; 128];
            tmp.copy_from_slice(&input[..128]);
            assert_eq!(verify(&tmp).unwrap(), true);
        }
    }

    #[test]
    fn invalid() {
        let inputs = [
      "10cd4bfe82f3de702d6c7bdb03fb5e198b2c919d511da05340d9e9a8d19f23a36f584bdc06e318b65ebc19e9770f82688686c3301a46cda717a6f5d9799708fd66849b045d01b50976f25e092d82fd85c932ef28294e8270a3abdc98f4c8836aa3929002a0c7a33b4c8d9bb422cce5cd52954ade1228f1552694bb6f9f02fd0a",
      "6eab91e7e0497708974c04cd7a92ac5984c751ff314b8ffdedf9e6557891dd9ba8f3e79b5fd9518562af18fceab88dd334a2217a21a473d77f1c8f2ff0d6aa3fbf75a63eb07cb71af0dad5498cb85dee04ee09b52f08810e5438cf504d366e8d79ff7713a37b4aad2d71b5919e009f29dbed7000216d5c30a332d034bd6d5306",
      "09168da69e98a320abaf4d6800f818c3ac184a23dea24eea9bf9b562ab5cad1a62e2dd9d14b23ad80213b4ffd2bd11b9778464b95e5fb801dddecefe98ab01759916665e5211dae56d0c56cf55325785eb0415a2024a819a8503a1f272b4870df143f644ffb6709e68cd2a4899c638f24c7d5ab0a97a5a609b0013f8e1a8e80c",
      "328dc5e413547ab2a880cc31799d56fbbd74e3cd189b03c23b522f3ca6d0c4caa8348e1940ca6909ccd24c94ba3fdff06ce00be8bdf803c3b8e55894204260b0c3ca89cc9d010e455df065173ee9c2a46ac009da91bc47fd57f0b52fa83af58dab8d2159ddadc940b3f05f7f9cc3773336bda31b886130ef0b835b9a7e4df504",
      "3f86ab0c079174566336ecdc9dee65122cab4509f214023394cf20935a2f652e5a087ae6914888b6477a1abfc7a2a882c94c4be232cf71c7ea17a3536425cddad455185ec6f66f81437fe231018f7e2f45e053c44a6faebb7c66c12028e4bd1d24e8657bbc5c7472a0122eec7e5621cd60d6e8a4b546297fd98895df0bd11b0a",
      "35bf580db70fef11c8f207035a862c24e9190105f88c4337157d71dbd6ddcf6d3f8ef5ec4351210fb645adb3527ab354770fcc6ad1256ea283125c4f74fd254dd64d94983335d9aacae84b2803a44c28d92e2a119c31d62e8cab8fd453d712b50044be7e9c540330cd4a164729b49b50d1f80084568e4637afbd4a66e2090e00",
      "6506e556d2decaa0e79ecc7c4418942f5e53f0724479a5d2de7e8745e28f51b5e4485ef638bf72054adec057c18a7caa92739d83cb8b81a2486d65fe2573162a94400812e2840600f98ee6026ff492f3130f189c080073ec553599e1916084ef7257ad11c3685cfcc0c172b265947cac4c786877fc57ec7ebcc387a042b3200e",
      "815266a579c22b95366b9346f22b221d549e61b8480a2f6dbb093b3ab395758e09101ce0c831a172538acf0b3ade4f2e1468e31d71c7c159e8bb13acde91fddd9a17a8adbb390c51366b089efcfedb35b6a711ce890c86e25a99e59fd0590af82facfede4c321877004ee35c5069c1f54fea8561fdf159c6362d55eb8c0f3c0a",
      "1110f6e4c25cbc9453a02d8de442496f8fab9e9f5a989e648720ec715f43a14690708b09728172402cf857d3545ecf4882d3c0828d79e1f20360b7371da795af7a0df44510016c03ce137524349f422f399434e7365e5c6b904d466ddedd0639a5348413c758b8a5fa4a9c4adaf25e1c01f480fa9105c6820307df7336563a0b",
      "6b8567b18b4dfa882b92970c38d6568c104e0e69bba71cb45a1ab620bccf37bb9409cb3e4cf0983a40cd2ce8b98f4fe8f80b0a6bbbf9a111ff407aea383b29f5447fc9d754f8af11971771e2870431e956af319548618148bcfb918d4a030582ada2cfd9a051bac4d56c8c880333fdcf1071562abe05c5c33ed06ec2745d010c",
    ];
        for i in 0..inputs.len() {
            let input = hex::decode(inputs[i]).unwrap();
            let mut tmp = [0u8; 128];
            tmp.copy_from_slice(&input[..128]);
            assert_eq!(verify(&tmp).unwrap(), false);
        }
    }
}