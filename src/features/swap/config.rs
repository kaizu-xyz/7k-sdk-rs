use anyhow::Result;

use crate::types::aggregators::{
    AftermathConfig, BluefinConfig, BluemoveConfig, CetusConfig, Config, DeepbookV3Config,
    DexConfig, FlowxConfig, FlowxV3Config, KriyaV3Config, ObricConfig, TurbosConfig,
};

// static mut CONFIG: Option<Config> = None;
// static mut CONFIG_TS: u64 = 0;

// Seconds
const TTL: u64 = 60;

pub struct ConfigManager {
    config: Config,
    ts: u64,
}

impl ConfigManager {
    pub async fn new() -> Result<Self> {
        Ok(ConfigManager {
            config: get_refreshed_config().await?,
            ts: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
        })
    }

    pub async fn get_config(&mut self) -> Result<&Config> {
        if std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - self.ts
            > TTL
        {
            self.config = get_refreshed_config().await?;
            self.ts = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
        Ok(&self.config)
    }

    pub fn get_ts(&self) -> u64 {
        self.ts
    }
}

pub async fn get_refreshed_config() -> Result<Config> {
    let client = reqwest::Client::new();
    match client.get("https://api.7k.ag/config").send().await {
        Ok(response) => match response.json::<Config>().await {
            Ok(config) => Ok(config),
            Err(_) => Ok(get_default_config()),
        },
        Err(_) => Ok(get_default_config()),
    }
}

fn get_default_config() -> Config {
    Config {
        aftermath: AftermathConfig {
            base: DexConfig {
                name: String::from("Aftermath"),
                package: String::from(
                    "0xc4049b2d1cc0f6e017fda8260e4377cecd236bd7f56a54fee120816e72e2e0dd",
                ),
                url: None,
                image: None,
            },
            pool_registry: String::from(
                "0xfcc774493db2c45c79f688f88d28023a3e7d98e4ee9f48bbf5c7990f651577ae",
            ),
            protocol_fee_vault: String::from(
                "0xf194d9b1bcad972e45a7dd67dd49b3ee1e3357a00a50850c52cd51bb450e13b4",
            ),
            treasury: String::from(
                "0x28e499dff5e864a2eafe476269a4f5035f1c16f338da7be18b103499abf271ce",
            ),
            insurance_fund: String::from(
                "0xf0c40d67b078000e18032334c3325c47b9ec9f3d9ae4128be820d54663d14e3b",
            ),
            referral_vault: String::from(
                "0x35d35b0e5b177593d8c3a801462485572fc30861e6ce96a55af6dc4730709278",
            ),
        },
        bluefin: BluefinConfig {
            base: DexConfig {
                name: String::from("Bluefin"),
                package: String::from(
                    "0x6c796c3ab3421a68158e0df18e4657b2827b1f8fed5ed4b82dba9c935988711b",
                ),
                url: None,
                image: None,
            },
            global_config: String::from(
                "0x03db251ba509a8d5d8777b6338836082335d93eecbdd09a11e190a1cff51c352",
            ),
        },
        bluemove: BluemoveConfig {
            base: DexConfig {
                name: String::from("Bluemove"),
                package: String::from(
                    "0x08cd33481587d4c4612865b164796d937df13747d8c763b8a178c87e3244498f",
                ),
                url: None,
                image: None,
            },
            dex_info: String::from(
                "0x3f2d9f724f4a1ce5e71676448dc452be9a6243dac9c5b975a588c8c867066e92",
            ),
        },
        cetus: CetusConfig {
            base: DexConfig {
                name: String::from("Cetus"),
                package: String::from(
                    "0x6f5e582ede61fe5395b50c4a449ec11479a54d7ff8e0158247adfda60d98970b",
                ),
                url: None,
                image: None,
            },
            global_config: String::from(
                "0xdaa46292632c3c4d8f31f23ea0f9b36a28ff3677e9684980e4438403a67a3d8f",
            ),
        },
        deepbook: DexConfig {
            name: String::from("Deepbook"),
            package: String::from("0xdee9"),
            url: None,
            image: None,
        },
        deepbook_v3: DeepbookV3Config {
            base: DexConfig {
                name: String::from("Deepbook V3"),
                package: String::from(""),
                url: None,
                image: None,
            },
            sponsor: String::from(
                "0x951a01360d85b06722edf896852bf8005b81cdb26375235c935138987f629502",
            ),
            sponsor_fund: String::from(
                "0xf245e7a4b83ed9a26622f5818a158c2ba7a03b91e62717b557a7df1d4dab38df",
            ),
        },
        flowx: FlowxConfig {
            base: DexConfig {
                name: String::from("Flowx Finance"),
                package: String::from(
                    "0xba153169476e8c3114962261d1edc70de5ad9781b83cc617ecc8c1923191cae0",
                ),
                url: None,
                image: None,
            },
            container: String::from(
                "0xb65dcbf63fd3ad5d0ebfbf334780dc9f785eff38a4459e37ab08fa79576ee511",
            ),
        },
        flowx_v3: FlowxV3Config {
            base: DexConfig {
                name: String::from("Flowx Finance V3"),
                package: String::from(
                    "0x25929e7f29e0a30eb4e692952ba1b5b65a3a4d65ab5f2a32e1ba3edcb587f26d",
                ),
                url: None,
                image: None,
            },
            registry: String::from(
                "0x27565d24a4cd51127ac90e4074a841bbe356cca7bf5759ddc14a975be1632abc",
            ),
            version: String::from(
                "0x67624a1533b5aff5d0dfcf5e598684350efd38134d2d245f475524c03a64e656",
            ),
        },
        kriya: DexConfig {
            name: String::from("Kriya"),
            package: String::from(
                "0xa0eba10b173538c8fecca1dff298e488402cc9ff374f8a12ca7758eebe830b66",
            ),
            url: None,
            image: None,
        },
        kriya_v3: KriyaV3Config {
            base: DexConfig {
                name: String::from("Kriya V3"),
                package: String::from(
                    "0xbd8d4489782042c6fafad4de4bc6a5e0b84a43c6c00647ffd7062d1e2bb7549e",
                ),
                url: None,
                image: None,
            },
            version: String::from(
                "0xf5145a7ac345ca8736cf8c76047d00d6d378f30e81be6f6eb557184d9de93c78",
            ),
        },
        obric: ObricConfig {
            base: DexConfig {
                name: String::from("Obric"),
                package: String::from(
                    "0xb84e63d22ea4822a0a333c250e790f69bf5c2ef0c63f4e120e05a6415991368f",
                ),
                url: None,
                image: None,
            },
            pyth_state: String::from(
                "0x1f9310238ee9298fb703c3419030b35b22bb1cc37113e3bb5007c99aec79e5b8",
            ),
        },
        springsui: DexConfig {
            name: String::from("SpringSui"),
            package: String::from(
                "0x82e6f4f75441eae97d2d5850f41a09d28c7b64a05b067d37748d471f43aaf3f7",
            ),
            url: None,
            image: None,
        },
        stsui: DexConfig {
            name: String::from("AlphaFi stSUI"),
            package: String::from(
                "0x059f94b85c07eb74d2847f8255d8cc0a67c9a8dcc039eabf9f8b9e23a0de2700",
            ),
            url: None,
            image: None,
        },
        suiswap: DexConfig {
            name: String::from("SuiSwap"),
            package: String::from(
                "0xd075d51486df71e750872b4edf82ea3409fda397ceecc0b6aedf573d923c54a0",
            ),
            url: None,
            image: None,
        },
        turbos: TurbosConfig {
            base: DexConfig {
                name: String::from("Turbos Finance"),
                package: String::from(
                    "0x1a3c42ded7b75cdf4ebc7c7b7da9d1e1db49f16fcdca934fac003f35f39ecad9",
                ),
                url: None,
                image: None,
            },
            version: String::from(
                "0xf1cf0e81048df168ebeb1b8030fad24b3e0b53ae827c25053fff0779c1445b6f",
            ),
        },
    }
}
