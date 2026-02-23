//! # StellarLend Protocol – Event Logging
//!
//! Defines a **consistent, structured event schema** for every state-changing
//! action in the StellarLend protocol, including governance operations.
//!
//! ## Design principles
//! - Each event is its own `#[contractevent]` struct. The macro auto-derives
//!   the lowercase snake_case struct name as the leading topic, generates XDR
//!   spec entries, and exposes a `.publish(&env)` method.
//! - Fields annotated with `#[topic]` become additional Soroban event topics.
//!   All other fields are packed into the event data payload (default format: map).
//! - `emit_*` helper functions wrap struct construction and call `.publish`,
//!   providing a single call-site per action.
//! - **No sensitive data**: all fields are publicly observable state only
//!   (`Address`, `Symbol`, `i128`, `u32`, `u64`, `bool`, `Option<Address>`).
#[allow(unused_variables)]
use soroban_sdk::{contractevent, Address, Env, String, Symbol, Vec};

use crate::types::{AssetStatus, ProposalStatus, ProposalType, VoteType};

// ============================================================================
// Core Lending Events (Existing)
// ============================================================================

/// Emitted when a user deposits collateral into the protocol.
#[contractevent]
#[derive(Clone, Debug)]
pub struct DepositEvent {
    pub user: Address,
    pub asset: Option<Address>,
    pub amount: i128,
    pub timestamp: u64,
}

/// Emitted when a user withdraws collateral from the protocol.
#[contractevent]
#[derive(Clone, Debug)]
pub struct WithdrawalEvent {
    pub user: Address,
    pub asset: Option<Address>,
    pub amount: i128,
    pub timestamp: u64,
}

/// Emitted when a user borrows assets from the protocol.
#[contractevent]
#[derive(Clone, Debug)]
pub struct BorrowEvent {
    pub user: Address,
    pub asset: Option<Address>,
    pub amount: i128,
    pub timestamp: u64,
}

/// Emitted when a user repays debt to the protocol.
#[contractevent]
#[derive(Clone, Debug)]
pub struct RepayEvent {
    pub user: Address,
    pub asset: Option<Address>,
    pub amount: i128,
    pub timestamp: u64,
}

/// Emitted when a liquidator liquidates an undercollateralised position.
#[contractevent]
#[derive(Clone, Debug)]
pub struct LiquidationEvent {
    pub liquidator: Address,
    pub borrower: Address,
    pub debt_asset: Option<Address>,
    pub collateral_asset: Option<Address>,
    pub debt_liquidated: i128,
    pub collateral_seized: i128,
    pub incentive_amount: i128,
    pub timestamp: u64,
}

/// Emitted when a flash loan is initiated.
#[contractevent]
#[derive(Clone, Debug)]
pub struct FlashLoanInitiatedEvent {
    pub user: Address,
    pub asset: Address,
    pub amount: i128,
    pub fee: i128,
    pub callback: Address,
    pub timestamp: u64,
}

/// Emitted when a flash loan is successfully repaid.
#[contractevent]
#[derive(Clone, Debug)]
pub struct FlashLoanRepaidEvent {
    pub user: Address,
    pub asset: Address,
    pub amount: i128,
    pub fee: i128,
    pub timestamp: u64,
}

/// Emitted for generic admin-initiated state-changing actions.
#[contractevent]
#[derive(Clone, Debug)]
pub struct AdminActionEvent {
    pub actor: Address,
    pub action: Symbol,
    pub timestamp: u64,
}

/// Emitted when an oracle price is updated.
#[contractevent]
#[derive(Clone, Debug)]
pub struct PriceUpdatedEvent {
    pub actor: Address,
    pub asset: Address,
    pub price: i128,
    pub decimals: u32,
    pub oracle: Address,
    pub timestamp: u64,
}

/// Emitted when risk parameters are updated by an admin.
#[contractevent]
#[derive(Clone, Debug)]
pub struct RiskParamsUpdatedEvent {
    pub actor: Address,
    pub timestamp: u64,
}

/// Emitted when the pause state of any protocol operation changes.
#[contractevent]
#[derive(Clone, Debug)]
pub struct PauseStateChangedEvent {
    pub actor: Address,
    pub operation: Symbol,
    pub paused: bool,
    pub timestamp: u64,
}

/// Emitted when a user's position is updated.
#[contractevent]
#[derive(Clone, Debug)]
pub struct PositionUpdatedEvent {
    pub user: Address,
    pub collateral: i128,
    pub debt: i128,
}

/// Emitted when analytics data is updated.
#[contractevent]
#[derive(Clone, Debug)]
pub struct AnalyticsUpdatedEvent {
    pub user: Address,
    pub activity_type: String,
    pub amount: i128,
    pub timestamp: u64,
}

/// Emitted when user activity is tracked.
#[contractevent]
#[derive(Clone, Debug)]
pub struct UserActivityTrackedEvent {
    pub user: Address,
    pub operation: Symbol,
    pub amount: i128,
    pub timestamp: u64,
}

// ============================================================================
// Asset-Specific Events (Carbon Asset Style)
// ============================================================================

/// Emitted when an asset is minted.
#[contractevent]
#[derive(Clone, Debug)]
pub struct MintEvent {
    pub token_id: u32,
    pub owner: Address,
    pub project_id: String,
    pub vintage_year: u64,
    pub methodology_id: u32,
}

/// Emitted when an asset is transferred.
#[contractevent]
#[derive(Clone, Debug)]
pub struct TransferEvent {
    pub token_id: u32,
    pub from: Address,
    pub to: Address,
}

/// Emitted when asset status changes.
#[contractevent]
#[derive(Clone, Debug)]
pub struct StatusChangeEvent {
    pub token_id: u32,
    pub old_status: Option<AssetStatus>,
    pub new_status: AssetStatus,
    pub changed_by: Address,
}

/// Emitted when quality score is updated.
#[contractevent]
#[derive(Clone, Debug)]
pub struct QualityScoreUpdatedEvent {
    pub token_id: u32,
    pub old_score: i128,
    pub new_score: i128,
    pub updated_by: Address,
}

/// SEP-41 style approve event.
#[contractevent]
#[derive(Clone, Debug)]
pub struct ApproveEvent {
    pub from: Address,
    pub spender: Address,
    pub amount: i128,
    pub live_until_ledger: u32,
}

/// SEP-41 style transfer event.
#[contractevent]
#[derive(Clone, Debug)]
pub struct Sep41TransferEvent {
    pub from: Address,
    pub to: Address,
    pub amount: i128,
}

/// SEP-41 style burn event.
#[contractevent]
#[derive(Clone, Debug)]
pub struct Sep41BurnEvent {
    pub from: Address,
    pub amount: i128,
}

// ============================================================================
// Governance Events
// ============================================================================

/// Emitted when governance system is initialized.
#[contractevent]
#[derive(Clone, Debug)]
pub struct GovernanceInitializedEvent {
    pub admin: Address,
    pub vote_token: Address,
    pub voting_period: u64,
    pub quorum_bps: u32,
    pub timestamp: u64,
}

/// Emitted when a new proposal is created.
#[contractevent]
#[derive(Clone, Debug)]
pub struct ProposalCreatedEvent {
    pub proposal_id: u64,
    pub proposer: Address,
    pub proposal_type: ProposalType,
    pub description: String,
    pub start_time: u64,
    pub end_time: u64,
    pub created_at: u64,
}

/// Emitted when a vote is cast on a proposal.
#[contractevent]
#[derive(Clone, Debug)]
pub struct VoteCastEvent {
    pub proposal_id: u64,
    pub voter: Address,
    pub vote_type: VoteType,
    pub voting_power: i128,
    pub timestamp: u64,
}

/// Emitted when a proposal is queued for execution.
#[contractevent]
#[derive(Clone, Debug)]
pub struct ProposalQueuedEvent {
    pub proposal_id: u64,
    pub execution_time: u64,
    pub for_votes: i128,
    pub against_votes: i128,
    pub quorum_reached: bool,
    pub threshold_met: bool,
}

/// Emitted when a proposal is executed.
#[contractevent]
#[derive(Clone, Debug)]
pub struct ProposalExecutedEvent {
    pub proposal_id: u64,
    pub executor: Address,
    pub timestamp: u64,
}

/// Emitted when a proposal fails.
#[contractevent]
#[derive(Clone, Debug)]
pub struct ProposalFailedEvent {
    pub proposal_id: u64,
    pub for_votes: i128,
    pub against_votes: i128,
    pub quorum_reached: bool,
    pub threshold_met: bool,
}

/// Emitted when a proposal is cancelled.
#[contractevent]
#[derive(Clone, Debug)]
pub struct ProposalCancelledEvent {
    pub proposal_id: u64,
    pub caller: Address,
    pub timestamp: u64,
}

/// Emitted when a multisig admin approves a proposal.
#[contractevent]
#[derive(Clone, Debug)]
pub struct ProposalApprovedEvent {
    pub proposal_id: u64,
    pub approver: Address,
    pub timestamp: u64,
}

/// Emitted when governance configuration is updated.
#[contractevent]
#[derive(Clone, Debug)]
pub struct GovernanceConfigUpdatedEvent {
    pub admin: Address,
    pub voting_period: Option<u64>,
    pub execution_delay: Option<u64>,
    pub quorum_bps: Option<u32>,
    pub proposal_threshold: Option<i128>,
    pub timestamp: u64,
}

// ============================================================================
// Multisig Events
// ============================================================================

/// Emitted when multisig configuration is updated.
#[contractevent]
#[derive(Clone, Debug)]
pub struct MultisigConfigUpdatedEvent {
    pub admin: Address,
    pub admins: Vec<Address>,
    pub threshold: u32,
    pub timestamp: u64,
}

// ============================================================================
// Guardian & Recovery Events
// ============================================================================

/// Emitted when a guardian is added.
#[contractevent]
#[derive(Clone, Debug)]
pub struct GuardianAddedEvent {
    pub guardian: Address,
    pub added_by: Address,
    pub timestamp: u64,
}

/// Emitted when a guardian is removed.
#[contractevent]
#[derive(Clone, Debug)]
pub struct GuardianRemovedEvent {
    pub guardian: Address,
    pub removed_by: Address,
    pub timestamp: u64,
}

/// Emitted when guardian threshold is updated.
#[contractevent]
#[derive(Clone, Debug)]
pub struct GuardianThresholdUpdatedEvent {
    pub admin: Address,
    pub old_threshold: u32,
    pub new_threshold: u32,
    pub timestamp: u64,
}

/// Emitted when a recovery process is started.
#[contractevent]
#[derive(Clone, Debug)]
pub struct RecoveryStartedEvent {
    pub old_admin: Address,
    pub new_admin: Address,
    pub initiator: Address,
    pub expires_at: u64,
    pub timestamp: u64,
}

/// Emitted when a recovery request is approved by a guardian.
#[contractevent]
#[derive(Clone, Debug)]
pub struct RecoveryApprovedEvent {
    pub approver: Address,
    pub current_approvals: u32,
    pub threshold: u32,
    pub timestamp: u64,
}

/// Emitted when recovery is executed and admin is changed.
#[contractevent]
#[derive(Clone, Debug)]
pub struct RecoveryExecutedEvent {
    pub old_admin: Address,
    pub new_admin: Address,
    pub executor: Address,
    pub timestamp: u64,
}

// ============================================================================
// Core Lending Emitter Helpers
// ============================================================================

pub fn emit_deposit(e: &Env, event: DepositEvent) {
    event.publish(e);
}

pub fn emit_withdrawal(e: &Env, event: WithdrawalEvent) {
    event.publish(e);
}

pub fn emit_borrow(e: &Env, event: BorrowEvent) {
    event.publish(e);
}

pub fn emit_repay(e: &Env, event: RepayEvent) {
    event.publish(e);
}

pub fn emit_liquidation(e: &Env, event: LiquidationEvent) {
    event.publish(e);
}

pub fn emit_flash_loan_initiated(e: &Env, event: FlashLoanInitiatedEvent) {
    event.publish(e);
}

pub fn emit_flash_loan_repaid(e: &Env, event: FlashLoanRepaidEvent) {
    event.publish(e);
}

pub fn emit_admin_action(e: &Env, event: AdminActionEvent) {
    event.publish(e);
}

pub fn emit_price_updated(e: &Env, event: PriceUpdatedEvent) {
    event.publish(e);
}

pub fn emit_risk_params_updated(e: &Env, event: RiskParamsUpdatedEvent) {
    event.publish(e);
}

pub fn emit_pause_state_changed(e: &Env, event: PauseStateChangedEvent) {
    event.publish(e);
}

pub fn emit_position_updated(e: &Env, event: PositionUpdatedEvent) {
    event.publish(e);
}

pub fn emit_analytics_updated(e: &Env, event: AnalyticsUpdatedEvent) {
    event.publish(e);
}

pub fn emit_user_activity_tracked(e: &Env, event: UserActivityTrackedEvent) {
    event.publish(e);
}

// ============================================================================
// Asset-Specific Emitter Helpers
// ============================================================================

pub fn emit_mint(e: &Env, event: MintEvent) {
    event.publish(e);
}

pub fn emit_transfer(e: &Env, event: TransferEvent) {
    event.publish(e);
}

pub fn emit_status_change(e: &Env, event: StatusChangeEvent) {
    event.publish(e);
}

pub fn emit_quality_score_updated(e: &Env, event: QualityScoreUpdatedEvent) {
    event.publish(e);
}

pub fn emit_approve(e: &Env, event: ApproveEvent) {
    event.publish(e);
}

pub fn emit_sep41_transfer(e: &Env, event: Sep41TransferEvent) {
    event.publish(e);
}

pub fn emit_sep41_burn(e: &Env, event: Sep41BurnEvent) {
    event.publish(e);
}

// ============================================================================
// Governance Emitter Helpers
// ============================================================================

pub fn emit_governance_initialized(e: &Env, event: GovernanceInitializedEvent) {
    event.publish(e);
}

pub fn emit_proposal_created(e: &Env, event: ProposalCreatedEvent) {
    event.publish(e);
}

pub fn emit_vote_cast(e: &Env, event: VoteCastEvent) {
    event.publish(e);
}

pub fn emit_proposal_queued(e: &Env, event: ProposalQueuedEvent) {
    event.publish(e);
}

pub fn emit_proposal_executed(e: &Env, event: ProposalExecutedEvent) {
    event.publish(e);
}

pub fn emit_proposal_failed(e: &Env, event: ProposalFailedEvent) {
    event.publish(e);
}

pub fn emit_proposal_cancelled(e: &Env, event: ProposalCancelledEvent) {
    event.publish(e);
}

pub fn emit_proposal_approved(e: &Env, event: ProposalApprovedEvent) {
    event.publish(e);
}

pub fn emit_governance_config_updated(e: &Env, event: GovernanceConfigUpdatedEvent) {
    event.publish(e);
}

// ============================================================================
// Multisig Emitter Helpers
// ============================================================================

pub fn emit_multisig_config_updated(e: &Env, event: MultisigConfigUpdatedEvent) {
    event.publish(e);
}

// ============================================================================
// Guardian & Recovery Emitter Helpers
// ============================================================================

pub fn emit_guardian_added(e: &Env, event: GuardianAddedEvent) {
    event.publish(e);
}

pub fn emit_guardian_removed(e: &Env, event: GuardianRemovedEvent) {
    event.publish(e);
}

pub fn emit_guardian_threshold_updated(e: &Env, event: GuardianThresholdUpdatedEvent) {
    event.publish(e);
}

pub fn emit_recovery_started(e: &Env, event: RecoveryStartedEvent) {
    event.publish(e);
}

pub fn emit_recovery_approved(e: &Env, event: RecoveryApprovedEvent) {
    event.publish(e);
}

pub fn emit_recovery_executed(e: &Env, event: RecoveryExecutedEvent) {
    event.publish(e);
}
