use solana_program::program_error::ProgramError;
use solana_program::pubkey;
use solana_program::pubkey::Pubkey;

#[derive(Debug, Clone, Copy)]
pub enum SupportedToken {
    USDC,
    USDT,
    Sol,
    Fida,
    FWC,
}


//mainnet
// const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
// const USDT_MINT: Pubkey = pubkey!("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB");
// const SOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
// const FIDA_MINT: Pubkey = pubkey!("FLEYqPkSSUoZXywYaKoN7eRPDFWDM6THLz2kuW9zKwHE");

//test devnet
const USDC_MINT: Pubkey = pubkey!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
const USDT_MINT: Pubkey = pubkey!("EJwZgeZrdC8TXTQbQBoL6bfuAnFUUy1PVCMB4DYPzVaS");
const SOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
const FIDA_MINT: Pubkey = pubkey!("fidaWCioBQjieRrUQDxxS5Uxmq1CLi2VuVRyv4dEBey");
const FWC_MINT: Pubkey = pubkey!("FLEYqPkSSUoZXywYaKoN7eRPDFWDM6THLz2kuW9zKwHE");

impl SupportedToken {
    pub const fn mint(self) -> Pubkey {
        match self {
            SupportedToken::USDC => USDC_MINT,
            SupportedToken::USDT => USDT_MINT,
            SupportedToken::Sol => SOL_MINT,
            SupportedToken::Fida => FIDA_MINT,
            SupportedToken::FWC => FWC_MINT,
        }
    }

    pub const fn from_mint(mint: &Pubkey) -> Result<Self, ProgramError> {
        Ok(match *mint {
            USDC_MINT => SupportedToken::USDC,
            USDT_MINT => SupportedToken::USDT,
            SOL_MINT => SupportedToken::Sol,
            FIDA_MINT => SupportedToken::Fida,
            FWC_MINT => SupportedToken::FWC,
            _ => return Err(ProgramError::InvalidArgument),
        })
    }

    pub const fn decimals(self) -> u8 {
        match self {
            SupportedToken::Sol => 9,
            SupportedToken::USDC
            | SupportedToken::USDT
            | SupportedToken::Fida
            | SupportedToken::FWC => 6,
        }
    }

    //price feed account key on the pyth
    pub const fn price_feed_account_key(self) -> Pubkey {
        match self {
            SupportedToken::USDC => pubkey!("Dpw1EAVrSB1ibxiDQyTAW6Zip3J4Btk2x4SgApQCeFbX"),
            SupportedToken::USDT => pubkey!("HT2PLQBcG5EiCcNSaMHAjSgd9F98ecpATbk4Sk5oYuM"),
            SupportedToken::Sol => pubkey!("7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE"),
            SupportedToken::Fida => pubkey!("2cfmeuVBf7bvBJcjKBQgAwfvpUvdZV7K8NZxUEuccrub"),
            //test
            SupportedToken::FWC => pubkey!("Dpw1EAVrSB1ibxiDQyTAW6Zip3J4Btk2x4SgApQCeFbX"),
        }
    }

    pub const fn price_feed(self) -> [u8; 32] {
        match self {
            SupportedToken::USDC => [
                234, 160, 32, 198, 28, 196, 121, 113, 40, 19, 70, 28, 225, 83, 137, 74, 150, 166,
                192, 11, 33, 237, 12, 252, 39, 152, 209, 249, 169, 233, 201, 74,
            ],
            SupportedToken::USDT => [
                43, 137, 185, 220, 143, 223, 159, 52, 112, 154, 91, 16, 107, 71, 47, 15, 57, 187,
                108, 169, 206, 4, 176, 253, 127, 46, 151, 22, 136, 226, 229, 59,
            ],
            SupportedToken::Sol => [
                239, 13, 139, 111, 218, 44, 235, 164, 29, 161, 93, 64, 149, 209, 218, 57, 42, 13,
                47, 142, 208, 198, 199, 188, 15, 76, 250, 200, 194, 128, 181, 109,
            ],
            SupportedToken::Fida => [
                200, 6, 87, 183, 246, 243, 234, 194, 114, 24, 208, 157, 90, 78, 84, 228, 123, 37,
                118, 141, 159, 94, 16, 172, 21, 254, 44, 249, 0, 136, 20, 0,
            ],
            SupportedToken::FWC => [
                234, 160, 32, 198, 28, 196, 121, 113, 40, 19, 70, 28, 225, 83, 137, 74, 150, 166,
                192, 11, 33, 237, 12, 252, 39, 152, 209, 249, 169, 233, 201, 74,
            ]
        }
    }
}

#[cfg(test)]
mod test {
    use crate::pyth::get_pyth_feed_account_key;
    use crate::tokens::SupportedToken;

    #[test]
    fn test_price_feed_account_key() {
        assert_eq!(
            SupportedToken::USDC.price_feed_account_key(),
            get_pyth_feed_account_key(0, &SupportedToken::USDC.price_feed())
        );
        assert_eq!(
            SupportedToken::USDT.price_feed_account_key(),
            get_pyth_feed_account_key(0, &SupportedToken::USDT.price_feed())
        );
        assert_eq!(
            SupportedToken::Sol.price_feed_account_key(),
            get_pyth_feed_account_key(0, &SupportedToken::Sol.price_feed())
        );
        assert_eq!(
            SupportedToken::Fida.price_feed_account_key(),
            get_pyth_feed_account_key(0, &SupportedToken::Fida.price_feed())
        );
    }

    #[test]
    fn test_feed_id() {
        // https://pyth.network/developers/price-feed-ids
        assert_eq!(
            SupportedToken::USDC.price_feed(),
            hex::decode("eaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a")
                .unwrap()
                .as_slice()
        );
        assert_eq!(
            SupportedToken::USDT.price_feed(),
            hex::decode("2b89b9dc8fdf9f34709a5b106b472f0f39bb6ca9ce04b0fd7f2e971688e2e53b")
                .unwrap()
                .as_slice()
        );
        assert_eq!(
            SupportedToken::Sol.price_feed(),
            hex::decode("ef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d")
                .unwrap()
                .as_slice()
        );
        assert_eq!(
            SupportedToken::Fida.price_feed(),
            hex::decode("c80657b7f6f3eac27218d09d5a4e54e47b25768d9f5e10ac15fe2cf900881400")
                .unwrap()
                .as_slice()
        );
    }
}
