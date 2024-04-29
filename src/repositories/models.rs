use crate::util::deserialize_str_to_number;
use serde::{Deserialize, Serialize};
use serde_json::Number;

pub use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Debug)]
pub enum TokenType {
    Id(String),
    Name(String),
    Pairs(Vec<EnhancedTransaction>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenFinal {
    token_descrip: TokenDescription,
    is_dexscreener: bool,
}

impl From<TokenFinal> for String {
    fn from(value: TokenFinal) -> Self {
        value.token_descrip.name.to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenDescription {
    pub id: u64,
    name: String,
    liquidity: u64, //?
}

impl From<TokenDescription> for String {
    fn from(value: TokenDescription) -> Self {
        value.name.to_string()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pairs {
    pub pairs: Vec<Pair>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pair {
    pub chain_id: String,
    pub dex_id: String,
    pub url: String,
    #[serde(default)]
    pub labels: Vec<String>,

    pub pair_address: String,
    pub base_token: WebhookToken,
    pub quote_token: WebhookToken,

    pub price_native: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_usd: Option<String>,
    pub txns: Timed<Transactions>,

    pub volume: Timed<f64>,
    pub price_change: Timed<f64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<Liquidity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fdv: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pair_created_at: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "Token")]
pub struct WebhookToken {
    pub address: Option<String>,
    pub name: Option<String>,
    pub symbol: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transactions {
    pub buys: u64,
    pub sells: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Liquidity {
    pub usd: f64,
    pub base: f64,
    pub quote: f64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Timed<T> {
    pub m5: T,
    pub h1: T,
    pub h6: T,
    pub h24: T,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnhancedTransaction {
    pub account_data: Vec<AccountData>,
    pub description: String,
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    pub source: Source,
    pub fee: i32,
    pub fee_payer: String,
    pub signature: String,
    pub slot: i32,
    pub native_transfers: Option<Vec<NativeTransfer>>,
    pub token_transfers: Option<Vec<TokenTransfer>>,
    pub transaction_error: Option<TransactionError>,
    pub instructions: Vec<Instruction>,
    pub events: TransactionEvent,
    pub timestamp: u64,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParseTransactionsRequest {
    pub transactions: Vec<String>,
}

impl ParseTransactionsRequest {
    /// Split the signatures into 100 vec sized chunks.
    /// Helius has a limit of 100 transactions per call
    pub fn from_slice(signatures: &[String]) -> Vec<Self> {
        signatures
            .chunks(100)
            .map(|chunk| Self {
                transactions: chunk.to_vec(),
            })
            .collect()
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEvent {
    pub nft: Option<NFTEvent>,
    pub swap: Option<SwapEvent>,
    pub compressed: Option<Vec<CompressedNftEvent>>,
    pub set_authority: Option<Vec<Authority>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompressedNftEvent {
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    pub tree_id: String,
    pub leaf_index: Option<i32>,
    pub seq: Option<i32>,
    pub asset_id: Option<String>,
    pub instruction_index: Option<i32>,
    pub inner_instruction_index: Option<i32>,
    pub new_leaf_owner: Option<String>,
    pub old_leaf_owner: Option<String>,
    pub new_leaf_delegate: Option<String>,
    pub old_leaf_delegate: Option<serde_json::Value>,
    pub tree_delegate: Option<String>,
    pub metadata: Option<Metadata>,
    pub update_args: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwapEvent {
    pub native_input: Option<NativeBalanceChange>,
    pub native_output: Option<NativeBalanceChange>,
    pub token_inputs: Vec<TokenBalanceChange>,
    pub token_outputs: Vec<TokenBalanceChange>,
    pub token_fees: Vec<TokenBalanceChange>,
    pub native_fees: Vec<NativeBalanceChange>,
    pub inner_swaps: Vec<TokenSwap>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenSwap {
    pub native_input: Option<NativeTransfer>,
    pub native_output: Option<NativeTransfer>,
    pub token_inputs: Vec<TokenTransfer>,
    pub token_outputs: Vec<TokenTransfer>,
    pub token_fees: Vec<TokenTransfer>,
    pub native_fees: Vec<NativeTransfer>,
    pub program_info: ProgramInfo,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProgramInfo {
    pub source: Source,
    pub account: String,
    pub program_name: ProgramName,
    pub instruction_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NFTEvent {
    pub seller: String,
    pub buyer: String,
    pub timestamp: Number,
    pub amount: Number,
    pub fee: Number,
    pub signature: String,
    pub source: Source,
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    pub sale_type: TransactionContext,
    pub nfts: Vec<Token>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub mint: String,
    pub token_standard: TokenStandard,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionError {
    #[serde(rename = "InstructionError")]
    pub instruciton_error: serde_json::Value,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NativeBalanceChange {
    pub account: String,
    #[serde(deserialize_with = "deserialize_str_to_number")]
    pub amount: Number,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccountData {
    pub account: String,
    pub native_balance_change: Number,
    pub token_balance_changes: Option<Vec<TokenBalanceChange>>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalanceChange {
    pub user_account: String,
    pub token_account: String,
    pub raw_token_amount: RawTokenAmount,
    pub mint: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RawTokenAmount {
    pub token_amount: String,
    pub decimals: Number,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TokenTransfer {
    #[serde(flatten)]
    pub user_accounts: TransferUserAccounts,
    pub from_token_account: Option<String>,
    pub to_token_account: Option<String>,
    pub token_amount: Number,
    pub token_standard: TokenStandard,
    pub mint: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransferUserAccounts {
    pub from_user_account: Option<String>,
    pub to_user_account: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NativeTransfer {
    #[serde(flatten)]
    pub user_accounts: TransferUserAccounts,
    pub amount: Number,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Instruction {
    pub accounts: Vec<String>,
    pub data: String,
    pub program_id: String,
    pub inner_instructions: Vec<InnerInstruction>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InnerInstruction {
    pub accounts: Vec<String>,
    pub data: String,
    pub program_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Collection {
    pub key: String,
    pub verified: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: i32,
    pub primary_sale_happened: bool,
    #[serde(rename = "isMutable")]
    pub mutable: bool,
    pub edition_nonce: Option<i32>,
    pub token_standard: Option<String>,
    pub collection: Option<Collection>,
    pub token_program_version: String,
    pub creators: Option<Vec<serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Authority {
    pub account: String,
    pub from: String,
    pub to: String,
    pub instruction_index: Option<i32>,
    pub inner_instruction_index: Option<i32>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize_enum_str, Serialize_enum_str)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Source {
    FormFunction,
    ExchangeArt,
    CandyMachineV3,
    CandyMachineV2,
    CandyMachineV1,
    Unknown,
    Solanart,
    Solsea,
    MagicEden,
    Holaplex,
    Metaplex,
    Opensea,
    SolanaProgramLibrary,
    Anchor,
    Phantom,
    SystemProgram,
    StakeProgram,
    Coinbase,
    CoralCube,
    Hedge,
    LaunchMyNft,
    GemBank,
    GemFarm,
    Degods,
    Bsl,
    Yawww,
    Atadia,
    DigitalEyes,
    Hyperspace,
    Tensor,
    Bifrost,
    Jupiter,
    Mecurial,
    Saber,
    Serum,
    StepFinance,
    Cropper,
    Raydium,
    Aldrin,
    Crema,
    Lifinity,
    Cykura,
    Orca,
    Marinade,
    Stepn,
    Sencha,
    Saros,
    EnglishAuction,
    Foxy,
    Hadeswap,
    FoxyStaking,
    FoxyRaffle,
    FoxyTokenMarket,
    FoxyMissions,
    FoxyMarmalade,
    FoxyCoinflip,
    FoxyAuction,
    Citrus,
    Zeta,
    Elixir,
    ElixirLaunchpad,
    CardinalRent,
    CardinalStaking,
    BpfLoader,
    BpfUpgradeableLoader,
    Squads,
    SharkyFi,
    OpenCreatorProtocol,
    Bubblegum,
    // Mints
    W_SOL,
    DUST,
    SOLI,
    USDC,
    FLWR,
    HDG,
    MEAN,
    UXD,
    SHDW,
    POLIS,
    ATLAS,
    USH,
    TRTLS,
    RUNNER,
    INVICTUS,
    #[serde(other)]
    Other(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize_enum_str, Serialize_enum_str)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    AcceptEscrowArtist,
    AcceptEscrowUser,
    AcceptRequestArtist,
    ActivateTransaction,
    ActivateVault,
    AddInstruction,
    AddItem,
    AddRaritiesToBank,
    AddTokenToVault,
    AddToPool,
    AddToWhitelist,
    Any,
    ApproveTransaction,
    AttachMetadata,
    AuctionHouseCreate,
    AuctionManagerClaimBid,
    AuthorizeFunder,
    BorrowFox,
    BorrowSolForNft,
    Burn,
    BurnNft,
    BuyItem,
    BuySubscription,
    BuyTickets,
    CancelEscrow,
    CancelLoanRequest,
    CancelOffer,
    CancelOrder,
    CancelReward,
    CancelSwap,
    CancelTransaction,
    CandyMachineRoute,
    CandyMachineUnwrap,
    CandyMachineUpdate,
    CandyMachineWrap,
    ChangeComicState,
    ClaimNft,
    ClaimRewards,
    CloseAccount,
    CloseEscrowAccount,
    CloseItem,
    CloseOrder,
    ClosePosition,
    CompressedNftBurn,
    CompressedNftCancelRedeem,
    CompressedNftDelegate,
    CompressedNftMint,
    CompressedNftRedeem,
    CompressedNftSetVerifyCollection,
    CompressedNftTransfer,
    CompressedNftUnverifyCollection,
    CompressedNftUnverifyCreator,
    CompressedNftVerifyCollection,
    CompressedNftVerifyCreator,
    CompressNft,
    CreateAppraisal,
    CreateBet,
    CreateEscrow,
    CreateMasterEdition,
    CreateMerkleTree,
    CreateOrder,
    CreatePool,
    CreateRaffle,
    CreateStore,
    CreateTransaction,
    DeauthorizeFunder,
    DecompressNft,
    DelegateMerkleTree,
    DelistItem,
    Deposit,
    DepositFractionalPool,
    DepositGem,
    DistributeCompressionRewards,
    EmptyPaymentAccount,
    ExecuteTransaction,
    FillOrder,
    FinalizeProgramInstruction,
    ForecloseLoan,
    Fractionalize,
    FundReward,
    Fuse,
    InitAuctionManagerV2,
    InitBank,
    InitFarm,
    InitFarmer,
    InitializeAccount,
    InitRent,
    InitStake,
    InitSwap,
    InitVault,
    KickItem,
    LendForNft,
    ListItem,
    Loan,
    LoanFox,
    LockReward,
    MergeStake,
    MigrateToPnft,
    NftAuctionCancelled,
    NftAuctionCreated,
    NftAuctionUpdated,
    NftBid,
    NftBidCancelled,
    NftCancelListing,
    NftGlobalBid,
    NftGlobalBidCancelled,
    NftListing,
    NftMint,
    NftMintRejected,
    NftParticipationReward,
    NftRentActivate,
    NftRentCancelListing,
    NftRentEnd,
    NftRentListing,
    NftRentUpdateListing,
    NftSale,
    OfferLoan,
    Payout,
    PlaceBet,
    PlaceSolBet,
    PlatformFee,
    ReborrowSolForNft,
    RecordRarityPoints,
    RefreshFarmer,
    RejectSwap,
    RejectTransaction,
    RemoveFromPool,
    RemoveFromWhitelist,
    RepayLoan,
    RequestLoan,
    RequestPnftMigration,
    RescindLoan,
    SetAuthority,
    SetBankFlags,
    SetVaultLock,
    SplitStake,
    StakeSol,
    StakeToken,
    StartPnftMigration,
    Swap,
    SwitchFox,
    SwitchFoxRequest,
    TakeLoan,
    TokenMint,
    Transfer,
    Unknown,
    Unlabeled,
    UnstakeSol,
    UnstakeToken,
    UpdateBankManager,
    UpdateExternalPriceAccount,
    UpdateFarm,
    UpdateItem,
    UpdateOffer,
    UpdateOrder,
    UpdatePrimarySaleMetadata,
    UpdateRaffle,
    UpdateRecordAuthorityData,
    UpdateVaultOwner,
    UpgradeFox,
    UpgradeFoxRequest,
    UpgradeProgramInstruction,
    ValidateSafetyDepositBoxV2,
    WhitelistCreator,
    Withdraw,
    WithdrawGem,
    #[serde(other)]
    Other(String),
}

#[allow(clippy::too_many_lines)]
impl TransactionType {
    pub fn all() -> Vec<Self> {
        vec![
            Self::AcceptEscrowArtist,
            Self::AcceptEscrowUser,
            Self::AcceptRequestArtist,
            Self::ActivateTransaction,
            Self::ActivateVault,
            Self::AddInstruction,
            Self::AddItem,
            Self::AddRaritiesToBank,
            Self::AddTokenToVault,
            Self::AddToPool,
            Self::AddToWhitelist,
            Self::Any,
            Self::ApproveTransaction,
            Self::AttachMetadata,
            Self::AuctionHouseCreate,
            Self::AuctionManagerClaimBid,
            Self::AuthorizeFunder,
            Self::BorrowFox,
            Self::BorrowSolForNft,
            Self::Burn,
            Self::BurnNft,
            Self::BuyItem,
            Self::BuySubscription,
            Self::BuyTickets,
            Self::CancelEscrow,
            Self::CancelLoanRequest,
            Self::CancelOffer,
            Self::CancelOrder,
            Self::CancelReward,
            Self::CancelSwap,
            Self::CancelTransaction,
            Self::CandyMachineRoute,
            Self::CandyMachineUnwrap,
            Self::CandyMachineUpdate,
            Self::CandyMachineWrap,
            Self::ChangeComicState,
            Self::ClaimNft,
            Self::ClaimRewards,
            Self::CloseAccount,
            Self::CloseEscrowAccount,
            Self::CloseItem,
            Self::CloseOrder,
            Self::ClosePosition,
            Self::CompressedNftBurn,
            Self::CompressedNftCancelRedeem,
            Self::CompressedNftDelegate,
            Self::CompressedNftMint,
            Self::CompressedNftRedeem,
            Self::CompressedNftSetVerifyCollection,
            Self::CompressedNftTransfer,
            Self::CompressedNftUnverifyCollection,
            Self::CompressedNftUnverifyCreator,
            Self::CompressedNftVerifyCollection,
            Self::CompressedNftVerifyCreator,
            Self::CompressNft,
            Self::CreateAppraisal,
            Self::CreateBet,
            Self::CreateEscrow,
            Self::CreateMasterEdition,
            Self::CreateMerkleTree,
            Self::CreateOrder,
            Self::CreatePool,
            Self::CreateRaffle,
            Self::CreateStore,
            Self::CreateTransaction,
            Self::DeauthorizeFunder,
            Self::DecompressNft,
            Self::DelegateMerkleTree,
            Self::DelistItem,
            Self::Deposit,
            Self::DepositFractionalPool,
            Self::DepositGem,
            Self::DistributeCompressionRewards,
            Self::EmptyPaymentAccount,
            Self::ExecuteTransaction,
            Self::FillOrder,
            Self::FinalizeProgramInstruction,
            Self::ForecloseLoan,
            Self::Fractionalize,
            Self::FundReward,
            Self::Fuse,
            Self::InitAuctionManagerV2,
            Self::InitBank,
            Self::InitFarm,
            Self::InitFarmer,
            Self::InitializeAccount,
            Self::InitRent,
            Self::InitStake,
            Self::InitSwap,
            Self::InitVault,
            Self::KickItem,
            Self::LendForNft,
            Self::ListItem,
            Self::Loan,
            Self::LoanFox,
            Self::LockReward,
            Self::MergeStake,
            Self::MigrateToPnft,
            Self::NftAuctionCancelled,
            Self::NftAuctionCreated,
            Self::NftAuctionUpdated,
            Self::NftBid,
            Self::NftBidCancelled,
            Self::NftCancelListing,
            Self::NftGlobalBid,
            Self::NftGlobalBidCancelled,
            Self::NftListing,
            Self::NftMint,
            Self::NftMintRejected,
            Self::NftParticipationReward,
            Self::NftRentActivate,
            Self::NftRentCancelListing,
            Self::NftRentEnd,
            Self::NftRentListing,
            Self::NftRentUpdateListing,
            Self::NftSale,
            Self::OfferLoan,
            Self::Payout,
            Self::PlaceBet,
            Self::PlaceSolBet,
            Self::PlatformFee,
            Self::ReborrowSolForNft,
            Self::RecordRarityPoints,
            Self::RefreshFarmer,
            Self::RejectSwap,
            Self::RejectTransaction,
            Self::RemoveFromPool,
            Self::RemoveFromWhitelist,
            Self::RepayLoan,
            Self::RequestLoan,
            Self::RequestPnftMigration,
            Self::RescindLoan,
            Self::SetAuthority,
            Self::SetBankFlags,
            Self::SetVaultLock,
            Self::SplitStake,
            Self::StakeSol,
            Self::StakeToken,
            Self::StartPnftMigration,
            Self::Swap,
            Self::SwitchFox,
            Self::SwitchFoxRequest,
            Self::TakeLoan,
            Self::TokenMint,
            Self::Transfer,
            Self::Unknown,
            Self::Unlabeled,
            Self::UnstakeSol,
            Self::UnstakeToken,
            Self::UpdateBankManager,
            Self::UpdateExternalPriceAccount,
            Self::UpdateFarm,
            Self::UpdateItem,
            Self::UpdateOffer,
            Self::UpdateOrder,
            Self::UpdatePrimarySaleMetadata,
            Self::UpdateRaffle,
            Self::UpdateRecordAuthorityData,
            Self::UpdateVaultOwner,
            Self::UpgradeFox,
            Self::UpgradeFoxRequest,
            Self::UpgradeProgramInstruction,
            Self::ValidateSafetyDepositBoxV2,
            Self::WhitelistCreator,
            Self::Withdraw,
            Self::WithdrawGem,
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize_enum_str, Serialize_enum_str)]
pub enum TokenStandard {
    ProgrammableNonFungible,
    NonFungible,
    Fungible,
    FungibleAsset,
    NonFungibleEdition,
    UnknownStandard,
    #[serde(other)]
    Other(String),
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize_enum_str, Serialize_enum_str)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionContext {
    Auction,
    InstantSale,
    Offer,
    GlobalOffer,
    Mint,
    Unknown,
    #[serde(other)]
    Other(String),
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize_enum_str, Serialize_enum_str)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProgramName {
    Unkown,
    JupiterV1,
    JupiterV2,
    JupiterV3,
    JupiterV4,
    MercurialStableSwap,
    SaberStableSwap,
    SaberExchange,
    SerumDexV1,
    SerumDexV2,
    SerumDexV3,
    SerumSwap,
    StepFinance,
    Cropper,
    RaydiumLiquidityPoolV2,
    RaydiumLiquidityPoolV3,
    RaydiumLiquidityPoolV4,
    AldrinAmmV1,
    AldrinAmmV2,
    Crema,
    Lifinity,
    LifinityV2,
    Cykura,
    OrcaTokenSwapV1,
    OrcaTokenSwapV2,
    OrcaWhirlpools,
    Marinade,
    Stepn,
    SenchaExchange,
    SarosAmm,
    FoxyStake,
    FoxySwap,
    FoxyRaffle,
    FoxyTokenMarket,
    FoxyMissions,
    FoxyMarmalade,
    FoxyCoinflip,
    FoxyAuction,
    Citrus,
    HadeSwap,
    Zeta,
    CardinalRent,
    CardinalStaking,
    SharkyFi,
    OpenCreatorProtocol,
    Bubblegum,
    CoralCube,
    #[serde(other)]
    Other(String),
}
