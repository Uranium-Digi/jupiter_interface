use anchor_lang::prelude::*;

#[derive(Clone, Debug, PartialEq, AnchorSerialize, AnchorDeserialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddLiquidity {
    pub token_amount_in: u64,
    pub min_lp_amount_out: u64,
    pub token_amount_pre_swap: Option<u64>,
}
#[derive(Clone, Debug, PartialEq, AnchorSerialize, AnchorDeserialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoveLiquidity {
    pub lp_amount_in: u64,
    pub min_amount_out: u64,
}
#[derive(Clone, Debug, PartialEq, AnchorSerialize, AnchorDeserialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AmountWithSlippage {
    pub amount: u64,
    pub slippage_bps: u16,
}
#[derive(Clone, Debug, PartialEq, AnchorSerialize, AnchorDeserialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RoutePlanStep {
    pub swap: Swap,
    pub percent: u8,
    pub input_index: u8,
    pub output_index: u8,
}
#[derive(Clone, Debug, PartialEq, AnchorSerialize, AnchorDeserialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Side {
    Bid,
    Ask,
}
#[derive(Clone, Debug, PartialEq, AnchorSerialize, AnchorDeserialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Swap {
    Saber,
    SaberAddDecimalsDeposit,
    SaberAddDecimalsWithdraw,
    TokenSwap,
    Sencha,
    Step,
    Cropper,
    Raydium,
    Crema { a_to_b: bool },
    Lifinity,
    Mercurial,
    Cykura,
    Serum { side: Side },
    MarinadeDeposit,
    MarinadeUnstake,
    Aldrin { side: Side },
    AldrinV2 { side: Side },
    Whirlpool { a_to_b: bool },
    Invariant { x_to_y: bool },
    Meteora,
    GooseFx,
    DeltaFi { stable: bool },
    Balansol,
    MarcoPolo { x_to_y: bool },
    Dradex { side: Side },
    LifinityV2,
    RaydiumClmm,
    Openbook { side: Side },
    Phoenix { side: Side },
    Symmetry { from_token_id: u64, to_token_id: u64 },
    TokenSwapV2,
    HeliumTreasuryManagementRedeemV0,
    StakeDexStakeWrappedSol,
    StakeDexSwapViaStake { bridge_stake_seed: u32 },
    GooseFxv2,
    Perps,
    PerpsAddLiquidity,
    PerpsRemoveLiquidity,
    MeteoraDlmm,
}
