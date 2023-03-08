import type BN from 'bn.js';
import type {ReturnNumber} from '@727-ventures/typechain-types';

export type AccountId = string | number[]

export enum LangError {
	couldNotReadInput = 'CouldNotReadInput'
}

export type PoolData = {
	registry: AccountId,
	asset: AccountId,
	collateralToken: AccountId,
	debtToken: AccountId,
	liquidityShare: ReturnNumber,
	liquidityIndex: ReturnNumber,
	liquidityRate: ReturnNumber,
	debtShare: ReturnNumber,
	debtIndex: ReturnNumber,
	debtRate: ReturnNumber,
	lastUpdateTimestamp: number
}

export type MarketData = {
	asset: AccountId,
	price: ReturnNumber,
	liquidityShare: ReturnNumber,
	liquidityIndex: ReturnNumber,
	liquidityInterestRate: ReturnNumber,
	debtShare: ReturnNumber,
	debtIndex: ReturnNumber,
	debtInterestRate: ReturnNumber,
	poolLastUpdateTimestamp: number,
	loanToValue: number,
	liquidationThreshold: number
}

