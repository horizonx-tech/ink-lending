import type BN from 'bn.js';

export type AccountId = string | number[]

export enum LangError {
	couldNotReadInput = 'CouldNotReadInput'
}

export type PoolData = {
	registry: AccountId,
	asset: AccountId,
	collateralToken: AccountId,
	debtToken: AccountId,
	liquidityShare: (string | number | BN),
	liquidityIndex: (string | number | BN),
	liquidityRate: (string | number | BN),
	debtShare: (string | number | BN),
	debtIndex: (string | number | BN),
	debtRate: (string | number | BN),
	lastUpdateTimestamp: (number | string | BN)
}

export type MarketData = {
	asset: AccountId,
	price: (string | number | BN),
	liquidityShare: (string | number | BN),
	liquidityIndex: (string | number | BN),
	liquidityInterestRate: (string | number | BN),
	debtShare: (string | number | BN),
	debtIndex: (string | number | BN),
	debtInterestRate: (string | number | BN),
	poolLastUpdateTimestamp: (number | string | BN),
	loanToValue: (number | string | BN),
	liquidationThreshold: (number | string | BN)
}

