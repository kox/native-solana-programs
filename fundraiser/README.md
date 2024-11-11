# Fundraiser Solana Program

## Thanks Giving To Turbin3 Team
@deanlittle @bergabman @ASCorreia @jeff

## Overview

This Solana program facilitates a decentralized fundraising mechanism. It allows users to create, contribute to, and manage fundraising campaigns, while ensuring trustless interactions using Program Derived Addresses (PDAs) for secure fund handling. The program consists of four main instructions:

1. **Initialize**: Sets up a new fundraiser campaign.
2. **Contribute**: Allows users to contribute to the fundraiser.
3. **Checker**: Enables the fundraiser creator to withdraw funds if the goal is met.
4. **Refund**: Allows contributors to retrieve their funds if the campaign fails.


## Table of Contents
- [Program Structure](#program-structure)
- [Instructions](#instructions)
  - [Initialize](#initialize)
  - [Contribute](#contribute)
  - [Checker](#checker)
  - [Refund](#refund)
- [State Accounts](#state-accounts)
  - [Fundraiser](#fundraiser)
  - [Contributor](#contributor)
- [Constants](#constants)
- [License](#license)

## Program Structure

### Modules
- **`instructions`**: Contains the logic for each instruction (`initialize`, `contribute`, `checker`, `refund`).
- **`state`**: Defines the data structures (`Fundraiser` and `Contributor`) stored on-chain.
- **`constants`**: Includes global constants used across the program.

### Entrypoint
```rust
entrypoint!(process_instruction);
```

The `process_instruction` function serves as the program's entrypoint, dispatching instructions based on a discriminator byte.

## Instructions

### Initialize
**Purpose**: Sets up a new fundraiser campaign with a target amount and deadline.

#### Accounts:
- `maker`: Creator of the fundraiser.
- `fundraiser`: PDA storing campaign details.
- `system_program`: Required for account creation.

#### Data:
- Fundraiser details (target amount, end slot, etc.).

### Contribute
**Purpose**: Allows a user to contribute tokens to the fundraiser.

#### Accounts:
- `contributor`: User contributing to the fundraiser.
- `contributor_ta`: Token account of the contributor.
- `fundraiser`: PDA with campaign details.
- `vault`: PDA that securely holds contributed funds.
- `token_program`: Token program for CPI transfers.

#### Data:
- `amount`: The amount of tokens to contribute.

#### Checks:
- Contribution must meet a minimum amount.
- Campaign must not be expired.
- Contribution must not exceed the remaining fundraising target.

### Checker
**Purpose**: Allows the fundraiser creator to claim the raised funds if the goal is met.

#### Accounts:
- `maker`: Creator of the fundraiser.
- `maker_ta`: Token account where funds will be transferred.
- `fundraiser`: PDA storing campaign details.
- `vault`: PDA holding raised funds.
- `authority`: PDA signer.
- `token_program`: Token program for CPI transfers.

#### Checks:
- Campaign must be expired.
- Fundraising goal must be met.

### Refund
**Purpose**: Enables contributors to reclaim their funds if the campaign fails.

#### Accounts:
- `contributor`: User reclaiming their contribution.
- `contributor_ta`: Token account to receive the refund.
- `contributor_account`: PDA tracking the contributor’s contribution.
- `fundraiser`: PDA storing campaign details.
- `vault`: PDA holding contributed funds.
- `authority`: PDA signer.
- `token_program`: Token program for CPI transfers.

#### Checks:
- Campaign must be expired.
- Fundraising goal not met.

## State Accounts

### Fundraiser
Holds the details of a fundraising campaign.

**Data Schema**:
- `maker`: `Pubkey` of the fundraiser creator.
- `mint`: `Pubkey` of the token used for contributions.
- `remaining_amount`: `u64` indicating the remaining amount to reach the target.
- `slot`: `u64` deadline for the fundraiser.
- `bump`: `u8` bump seed for PDA generation.

### Contributor
Tracks an individual contributor's participation.

**Data Schema**:
- `amount`: `u64` total contributed amount.
- `bump`: `u8` bump seed for PDA generation.

## Constants

The program uses several constants, including:
- `ID`: Unique program identifier.
- `PDA_MARKER`: Marker for generating PDAs.
- `MIN_AMOUNT_TO_RAISE`: Minimum contribution amount.

## CUs
- Initialize        -> 184
- Contribute        -> 6471
- Checker           -> 10769
- Refund            -> 6629

## License

This program is licensed under the [MIT License](https://opensource.org/licenses/MIT).