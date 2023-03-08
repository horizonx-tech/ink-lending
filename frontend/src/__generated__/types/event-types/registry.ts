import type {ReturnNumber} from "@727-ventures/typechain-types";
import type * as ReturnTypes from '../types-returns/registry';

export interface PoolRegistered {
	asset: ReturnTypes.AccountId;
	pool: ReturnTypes.AccountId;
}

export interface FactoryChanged {
	factory: ReturnTypes.AccountId;
}

export interface ManagerChanged {
	manager: ReturnTypes.AccountId;
}

export interface RateStrategyChanged {
	asset: ReturnTypes.AccountId | null;
	strategy: ReturnTypes.AccountId;
}

export interface RiskStrategyChanged {
	asset: ReturnTypes.AccountId | null;
	strategy: ReturnTypes.AccountId;
}

export interface PriceOracleChanged {
	priceOracle: ReturnTypes.AccountId;
}

