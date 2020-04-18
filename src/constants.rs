// Copyright (c) 2017 Clark Moody
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! Human-readable constants for various cryptocurrencies
//!
//! The authoratative list of Human-readable parts for Bech32 addresses is
//! maintained in [SLIP-0173](https://github.com/satoshilabs/slips/blob/master/slip-0173.md).

/// The cryptocurrency to act on
#[derive(PartialEq, Eq, Debug, Clone, Copy, PartialOrd, Ord, Hash)]
pub enum Network {
    /// Bitcoin mainnet
    Bitcoin,
    /// Bitcoin testnet
    Testnet,
    /// Bitcoin signet,
    Signet,
    /// Bitcoin regtest,
    Regtest,
    /// Bellcoin mainnet
    Bellcoin,
    /// Bellcoin testnet
    BellcoinTestnet,
    /// BitZeny mainnet
    BitZeny,
    /// BitZeny testnet
    BitZenyTestnet,
    /// CranePay mainnet
    CranePay,
    /// CranePay testnet
    CranePayTestnet,
    /// Crypto.com Chain mainnet
    CryptoComChain,
    /// Crypto.com Chain testnet
    CryptoComChainTestnet,
    /// DigiByte mainnet
    DigiByte,
    /// DigiByte testnet
    DigiByteTestnet,
    /// FujiCoin mainnet
    FujiCoin,
    /// FujiCoin testnet
    FujiCoinTestnet,
    /// Groestlcoin mainnet
    Groestlcoin,
    /// Groestlcoin testnet
    GroestlcoinTestnet,
    /// Handshake mainnet
    Handshake,
    /// Handshake testnet
    HandshakeTestnet,
    /// Litecoin mainnet
    Litecoin,
    /// Litecoin testnet
    LitecoinTestnet,
    /// Monacoin mainnet
    Monacoin,
    /// Monacoin testnet
    MonacoinTestnet,
    /// Myriad mainnet
    Myriad,
    /// Myriad testnet
    MyriadTestnet,
    /// Namecoin mainnet
    Namecoin,
    /// Namecoin testnet
    NamecoinTestnet,
    /// Peercoin mainnet
    Peercoin,
    /// Peercoin testnet
    PeercoinTestnet,
    /// PKT mainnet
    PKT,
    /// PKT testnet
    PKTTestnet,
    /// Quantum Resistant Ledger mainnet
    QuantumResistantLedger,
    /// Quantum Resistant Ledger testnet
    QuantumResistantLedgerTestnet,
    /// Ravencoin mainnet
    Ravencoin,
    /// Ravencoin testnet
    RavencoinTestnet,
    /// Susucoin mainnet
    Susucoin,
    /// Susucoin testnet
    SusucoinTestnet,
    /// Unit-e mainnet
    Unite,
    /// Unit-e testnet
    UniteTestnet,
    /// Vertcoin mainnet
    Vertcoin,
    /// Vertcoin testnet
    VertcoinTestnet,
    /// Viacoin mainnet
    Viacoin,
    /// Viacoin testnet
    ViacoinTestnet,
    /// VIPSTARCOIN mainnet
    VIPSTARCOIN,
    /// VIPSTARCOIN testnet
    VIPSTARCOINTestnet,
    /// Zen Protocol mainnet
    ZenProtocol,
    /// Zen Protocol testnet
    ZenProtocolTestnet,
    /// Zilliqa mainnet
    Zilliqa,
    /// Zilliqa testnet
    ZilliqaTestnet,
}

/// Returns the Human-readable part for the given network
pub fn hrp(network: &Network) -> String {
    match *network {
        Network::Bitcoin => "bc".to_string(),
        Network::Testnet => "tb".to_string(),
        Network::Signet => "sb".to_string(),
        Network::Bellcoin => "bm".to_string(),
        Network::BellcoinTestnet => "bt".to_string(),
        Network::BitZeny => "bz".to_string(),
        Network::BitZenyTestnet => "tz".to_string(),
        Network::CranePay => "cp".to_string(),
        Network::CranePayTestnet => "cpt".to_string(),
        Network::CryptoComChain => "cro".to_string(),
        Network::CryptoComChainTestnet => "tcro".to_string(),
        Network::DigiByte => "dgb".to_string(),
        Network::DigiByteTestnet => "dgbt".to_string(),
        Network::FujiCoin => "fc".to_string(),
        Network::FujiCoinTestnet => "tf".to_string(),
        Network::Groestlcoin => "grs".to_string(),
        Network::GroestlcoinTestnet => "tgrs".to_string(),
        Network::Handshake => "hs".to_string(),
        Network::HandshakeTestnet => "ts".to_string(),
        Network::Litecoin => "ltc".to_string(),
        Network::LitecoinTestnet => "tltc".to_string(),
        Network::Monacoin => "mona".to_string(),
        Network::MonacoinTestnet => "tmona".to_string(),
        Network::MonacoinRegtest => "rmona".to_string(),
        Network::Myriad => "my".to_string(),
        Network::MyriadTestnet => "tm".to_string(),
        Network::Namecoin => "nc".to_string(),
        Network::NamecoinTestnet => "tn".to_string(),
        Network::Peercoin => "xpc".to_string(),
        Network::PeercoinTestnet => "tpc".to_string(),
        Network::PKT => "pkt".to_string(),
        Network::PKTTestnet => "tpk".to_string(),
        Network::QuantumResistantLedger => "qrl".to_string(),
        Network::QuantumResistantLedgerTestnet => "tqrl".to_string(),
        Network::Ravencoin => "rc".to_string(),
        Network::RavencoinTestnet => "tr".to_string(),
        Network::Susucoin => "susu".to_string(),
        Network::SusucoinTestnet => "tutu".to_string(),
        Network::Unite => "ue".to_string(),
        Network::UniteTestnet => "tue".to_string(),
        Network::Vertcoin => "vtc".to_string(),
        Network::VertcoinTestnet => "tvtc".to_string(),
        Network::Viacoin => "via".to_string(),
        Network::ViacoinTestnet => "tvia".to_string(),
        Network::VIPSTARCOIN => "vips".to_string(),
        Network::VIPSTARCOINTestnet => "tvips".to_string(),
        Network::ZenProtocol => "zen".to_string(),
        Network::ZenProtocolTestnet => "tzn".to_string(),
        Network::Zilliqa => "zil".to_string(),
        Network::ZilliqaTestnet => "tzil".to_string(),
        Network::Regtest => "bcrt".to_string(),
    }
}

/// Classify a Human-readable part as its cryptocurrency
pub fn classify(hrp: &str) -> Option<Network> {
    match hrp {
        "bc" => Some(Network::Bitcoin),
        "tb" => Some(Network::Testnet),
        "sb" => Some(Network::Signet),
        "bm" => Some(Network::Bellcoin),
        "bt" => Some(Network::BellcoinTestnet),
        "bz" => Some(Network::BitZeny),
        "tz" => Some(Network::BitZenyTestnet),
        "cp" => Some(Network::CranePay),
        "cpt" => Some(Network::CranePayTestnet),
        "cro" => Some(Network::CryptoComChain),
        "tcro" => Some(Network::CryptoComChainTestnet),
        "dgb" => Some(Network::DigiByte),
        "dgbt" => Some(Network::DigiByteTestnet),
        "fc" => Some(Network::FujiCoin),
        "tf" => Some(Network::FujiCoinTestnet),
        "grs" => Some(Network::Groestlcoin),
        "tgrs" => Some(Network::GroestlcoinTestnet),
        "hs" => Some(Network::Handshake),
        "ts" => Some(Network::HandshakeTestnet),
        "ltc" => Some(Network::Litecoin),
        "tltc" => Some(Network::LitecoinTestnet),
        "mona" => Some(Network::Monacoin),
        "tmona" => Some(Network::MonacoinTestnet),
        "rmona" => Some(Network::MonacoinRegtest),
        "my" => Some(Network::Myriad),
        "tm" => Some(Network::MyriadTestnet),
        "nc" => Some(Network::Namecoin),
        "tn" => Some(Network::NamecoinTestnet),
        "xpc" => Some(Network::Peercoin),
        "tpc" => Some(Network::PeercoinTestnet),
        "pkt" => Some(Network::PKT),
        "tpk" => Some(Network::PKTTestnet),
        "qrl" => Some(Network::QuantumResistantLedger),
        "tqrl" => Some(Network::QuantumResistantLedgerTestnet),
        "rc" => Some(Network::Ravencoin),
        "tr" => Some(Network::RavencoinTestnet),
        "susu" => Some(Network::Susucoin),
        "tutu" => Some(Network::SusucoinTestnet),
        "ue" => Some(Network::Unite),
        "tue" => Some(Network::UniteTestnet),
        "vtc" => Some(Network::Vertcoin),
        "tvtc" => Some(Network::VertcoinTestnet),
        "via" => Some(Network::Viacoin),
        "tvia" => Some(Network::ViacoinTestnet),
        "vips" => Some(Network::VIPSTARCOIN),
        "tvips" => Some(Network::VIPSTARCOINTestnet),
        "zen" => Some(Network::ZenProtocol),
        "tzn" => Some(Network::ZenProtocolTestnet),
        "zil" => Some(Network::Zilliqa),
        "tzil" => Some(Network::ZilliqaTestnet),
        "bcrt" => Some(Network::Regtest),
        _ => None,
    }
}
