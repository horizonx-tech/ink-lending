/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { GasLimit, GasLimitAndRequiredValue } from '@727-ventures/typechain-types';
import { buildSubmittableExtrinsic } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/rate_strategies';
import type BN from 'bn.js';
import type { ApiPromise } from '@polkadot/api';



export default class Methods {
	private __nativeContract : ContractPromise;
	private __apiPromise: ApiPromise;

	constructor(
		nativeContract : ContractPromise,
		apiPromise: ApiPromise,
	) {
		this.__nativeContract = nativeContract;
		this.__apiPromise = apiPromise;
	}
	/**
	 * calculateRate
	 *
	 * @param { ArgumentTypes.AccountId } asset,
	 * @param { (string | number | BN) } liquidityAdded,
	 * @param { (string | number | BN) } liquidityTaken,
	*/
	"calculateRate" (
		asset: ArgumentTypes.AccountId,
		liquidityAdded: (string | number | BN),
		liquidityTaken: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "rateStrategy::calculateRate", [asset, liquidityAdded, liquidityTaken], __options);
	}

}