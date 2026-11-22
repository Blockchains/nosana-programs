use anchor_lang::declare_id;

/***
 * IDs
 */

pub use system_program::ID as SYSTEM_PROGRAM;
mod system_program {
    use super::*;
    declare_id!("11111111111111111111111111111111");
}

pub use jobs_program::ID as JOBS_PROGRAM;
mod jobs_program {
    use super::*;
    #[cfg(feature = "mainnet")]
    declare_id!("nosJhNRqr2bc9g1nfGDcXXTXvYUmxD4cVwy2pMWhrYM");
    #[cfg(not(feature = "mainnet"))]
    declare_id!("4nFPjT5MgCtYheCHGACin9S8N56Cg1RTBAhFQv43Ybxb"); // Sol.College devnet lab key (official devnet id not reused)
}

pub use rewards_program::ID as REWARDS_PROGRAM;
mod rewards_program {
    use super::*;
    declare_id!("EWdwdb7Jbjd2hSYAUKz5NTTCJSRgkZNH6tbXprAv3LEY"); // Sol.College devnet lab key (official id not reused)
}

pub use staking_program::ID as STAKING_PROGRAM;
mod staking_program {
    use super::*;
    declare_id!("qqA8dv9jjMYZJgVKBdiCjSL9MYQviiEN4R93dVuYeJR"); // Sol.College devnet lab key (official id not reused)
}

pub use pools_program::ID as POOLS_PROGRAM;
mod pools_program {
    use super::*;
    declare_id!("nosPdZrfDzND1LAR28FLMDEATUPK53K8xbRBXAirevD");
}

pub use nodes_program::ID as NODES_PROGRAM;
mod nodes_program {
    use super::*;
    declare_id!("nosNeZR64wiEhQc5j251bsP4WqDabT6hmz4PHyoHLGD");
}

pub use nos_token::ID as NOS_TOKEN;
mod nos_token {
    use super::*;
    #[cfg(feature = "mainnet")]
    declare_id!("nosXBVoaCTtYdLvKY6Csb4AC8JCdQKKAaWYtx2ZMoo7");
    #[cfg(not(feature = "mainnet"))]
    declare_id!("GKkroxnqiJvxyVUXKu9BAEttwGkZH5zSn8zvZ5b5fPt7"); // Sol.College devnet lab key (official devnet id not reused)
}

pub use authority::ID as AUTHORITY;
mod authority {
    use super::*;
    #[cfg(feature = "mainnet")]
    declare_id!("NosanarMxfrZbyCx5CotBVrzxiPcrnhj6ickpX9vRkB");
    #[cfg(not(feature = "mainnet"))]
    declare_id!("sZWwGfLUsuZSYW2vkF4RFBgQRQeq1HqXzzWPDPPFWQD"); // Sol.College devnet lab key (official devnet id not reused)
}

pub use market_admin::ID as MARKET_ADMIN;
mod market_admin {
    use super::*;
    #[cfg(feature = "mainnet")]
    declare_id!("AdmMVMMU3po5BP3bVWsF9GgN6NNEfpfxpTDGTzjgb72S");
    #[cfg(not(feature = "mainnet"))]
    declare_id!("sZWwGfLUsuZSYW2vkF4RFBgQRQeq1HqXzzWPDPPFWQD"); // Sol.College devnet lab key (official devnet id not reused)
}

pub use token_account::ID as TOKEN_ACCOUNT;
mod token_account {
    use super::*;
    #[cfg(feature = "mainnet")]
    declare_id!("A9V8JkR5HihvFpHq1ZbwrpPAGBhsGfeWw5TVcUdGf2dg");
    #[cfg(not(feature = "mainnet"))]
    declare_id!("Hz8amGzuMqXU9tQbEak2SHXShMahnKhx3LprtwS8Semn"); // Sol.College devnet lab key (official devnet id not reused)
}

pub use nft_collection::ID as NFT_COLLECTION;
mod nft_collection {
    use super::*;
    declare_id!("nftNgYSG5pbwL7kHeJ5NeDrX8c4KrG1CzWhEXT8RMJ3");
}

pub use metaplex_metadata::ID as METAPLEX_METADATA;
mod metaplex_metadata {
    use super::*;
    declare_id!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
}
