import type {ReturnNumber} from "@727-ventures/typechain-types";
import type * as ReturnTypes from '../types-returns/service';

export interface Deposited {
	asset: ReturnTypes.AccountId;
	account: ReturnTypes.AccountId;
	from: ReturnTypes.AccountId;
	amount: ReturnNumber;
}

export interface Withdrew {
	asset: ReturnTypes.AccountId;
	account: ReturnTypes.AccountId;
	to: ReturnTypes.AccountId;
	amount: ReturnNumber;
}

export interface Borrowed {
	asset: ReturnTypes.AccountId;
	account: ReturnTypes.AccountId;
	to: ReturnTypes.AccountId;
	amount: ReturnNumber;
}

export interface Repaid {
	asset: ReturnTypes.AccountId;
	account: ReturnTypes.AccountId;
	from: ReturnTypes.AccountId;
	amount: ReturnNumber;
}

export interface Liquidated {
	liquidator: ReturnTypes.AccountId;
	liquidatee: ReturnTypes.AccountId;
	collateralAsset: ReturnTypes.AccountId;
	collateralAmount: ReturnNumber;
	debtAsset: ReturnTypes.AccountId;
	debtAmount: ReturnNumber;
}

