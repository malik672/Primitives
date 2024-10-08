
#[inline]
pub const fn to_eip155_v(v: u8, chain_id: u64) -> u64 {
    (v as u64) + 35 + chain_id * 2
}

/// Normalizes a `v` value, respecting raw, legacy, and EIP-155 values.
///
/// This function covers the entire u64 range, producing v-values as follows:
/// - 0-26 - raw/bare. 0-3 are legal. In order to ensure that all values are covered, we also handle
///   4-26 here by returning v % 4.
/// - 27-34 - legacy. 27-30 are legal. By legacy bitcoin convention range 27-30 signals uncompressed
///   pubkeys, while 31-34 signals compressed pubkeys. We do not respect the compression convention.
///   All Ethereum keys are uncompressed.
/// - 35+ - EIP-155. By EIP-155 convention, `v = 35 + CHAIN_ID * 2 + 0/1` We return (v-1 % 2) here.
///
/// NB: raw and legacy support values 2, and 3, while EIP-155 does not.
/// Recovery values of 2 and 3 are unlikely to occur in practice. In the
/// vanishingly unlikely event  that you encounter an EIP-155 signature with a
/// recovery value of 2 or 3, you should normalize out of band.
#[cfg(feature = "k256")]
#[inline]
pub(crate) const fn normalize_v(v: u64) -> k256::ecdsa::RecoveryId {
    let byte = normalize_v_to_byte(v);
    assert!(byte <= k256::ecdsa::RecoveryId::MAX);
    match k256::ecdsa::RecoveryId::from_byte(byte) {
        Some(recid) => recid,
        None => unsafe { core::hint::unreachable_unchecked() },
    }
}

#[inline]
pub const fn normalize_v_to_byte(v: u64) -> u8 {
    match v {
        0..=26 => (v & 3) as u8,
        27..=34 => ((v - 27) & 3) as u8,
        35.. => ((v - 1) & 1) as u8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_normalize_v_to_byte() {
      
   
        assert_eq!(normalize_v_to_byte(0), 0);
        assert_eq!(normalize_v_to_byte(1), 1);
        assert_eq!(normalize_v_to_byte(2), 2);
        assert_eq!(normalize_v_to_byte(3), 3);
        assert_eq!(normalize_v_to_byte(4), 0);
        assert_eq!(normalize_v_to_byte(27), 0);
        assert_eq!(normalize_v_to_byte(28), 1);
        assert_eq!(normalize_v_to_byte(35), 0);
        assert_eq!(normalize_v_to_byte(36), 1);

    }


}
