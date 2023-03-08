/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryJSON, queryOkJSON, handleReturnType } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/registry';
import type * as ReturnTypes from '../types-returns/registry';
import type BN from 'bn.js';
//@ts-ignore
import {ReturnNumber} from '@727-ventures/typechain-types';
import {getTypeDescription} from './../shared/utils';


export default class Methods {
	private __nativeContract : ContractPromise;
	private __apiPromise: ApiPromise;
	private __callerAddress : string;

	constructor(
		nativeContract : ContractPromise,
		nativeApi : ApiPromise,
		callerAddress : string,
	) {
		this.__nativeContract = nativeContract;
		this.__callerAddress = callerAddress;
		this.__apiPromise = nativeApi;
	}

	/**
	* manager
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"manager" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::manager", [], __options , (result) => { return handleReturnType(result, getTypeDescription(8, 'registry')); });
	}

	/**
	* rateStrategy
	*
	* @param { ArgumentTypes.AccountId } asset,
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"rateStrategy" (
		asset: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::rateStrategy", [asset], __options , (result) => { return handleReturnType(result, getTypeDescription(8, 'registry')); });
	}

	/**
	* pool
	*
	* @param { ArgumentTypes.AccountId } asset,
	* @returns { Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> }
	*/
	"pool" (
		asset: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::pool", [asset], __options , (result) => { return handleReturnType(result, getTypeDescription(9, 'registry')); });
	}

	/**
	* assetsCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"assetsCount" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::assetsCount", [], __options , (result) => { return handleReturnType(result, getTypeDescription(10, 'registry')); });
	}

	/**
	* setFactory
	*
	* @param { ArgumentTypes.AccountId } address,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"setFactory" (
		address: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::setFactory", [address], __options , (result) => { return handleReturnType(result, getTypeDescription(12, 'registry')); });
	}

	/**
	* priceOracle
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"priceOracle" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::priceOracle", [], __options , (result) => { return handleReturnType(result, getTypeDescription(8, 'registry')); });
	}

	/**
	* factory
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"factory" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::factory", [], __options , (result) => { return handleReturnType(result, getTypeDescription(8, 'registry')); });
	}

	/**
	* setRiskStrategy
	*
	* @param { ArgumentTypes.AccountId } address,
	* @param { ArgumentTypes.AccountId | null } asset,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"setRiskStrategy" (
		address: ArgumentTypes.AccountId,
		asset: ArgumentTypes.AccountId | null,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::setRiskStrategy", [address, asset], __options , (result) => { return handleReturnType(result, getTypeDescription(12, 'registry')); });
	}

	/**
	* defaultRateStrategy
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"defaultRateStrategy" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::defaultRateStrategy", [], __options , (result) => { return handleReturnType(result, getTypeDescription(8, 'registry')); });
	}

	/**
	* registerPool
	*
	* @param { ArgumentTypes.AccountId } asset,
	* @param { ArgumentTypes.AccountId } pool,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"registerPool" (
		asset: ArgumentTypes.AccountId,
		pool: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::registerPool", [asset, pool], __options , (result) => { return handleReturnType(result, getTypeDescription(12, 'registry')); });
	}

	/**
	* assetList
	*
	* @returns { Result<Array<ReturnTypes.AccountId>, ReturnTypes.LangError> }
	*/
	"assetList" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Array<ReturnTypes.AccountId>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::assetList", [], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'registry')); });
	}

	/**
	* setPriceOracle
	*
	* @param { ArgumentTypes.AccountId } address,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"setPriceOracle" (
		address: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::setPriceOracle", [address], __options , (result) => { return handleReturnType(result, getTypeDescription(12, 'registry')); });
	}

	/**
	* riskStrategy
	*
	* @param { ArgumentTypes.AccountId } asset,
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"riskStrategy" (
		asset: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::riskStrategy", [asset], __options , (result) => { return handleReturnType(result, getTypeDescription(8, 'registry')); });
	}

	/**
	* defaultRiskStrategy
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"defaultRiskStrategy" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::defaultRiskStrategy", [], __options , (result) => { return handleReturnType(result, getTypeDescription(8, 'registry')); });
	}

	/**
	* asset
	*
	* @param { (number | string | BN) } index,
	* @returns { Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> }
	*/
	"asset" (
		index: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::asset", [index], __options , (result) => { return handleReturnType(result, getTypeDescription(9, 'registry')); });
	}

	/**
	* setManager
	*
	* @param { ArgumentTypes.AccountId } address,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"setManager" (
		address: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::setManager", [address], __options , (result) => { return handleReturnType(result, getTypeDescription(12, 'registry')); });
	}

	/**
	* setRateStrategy
	*
	* @param { ArgumentTypes.AccountId } address,
	* @param { ArgumentTypes.AccountId | null } asset,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"setRateStrategy" (
		address: ArgumentTypes.AccountId,
		asset: ArgumentTypes.AccountId | null,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry::setRateStrategy", [address, asset], __options , (result) => { return handleReturnType(result, getTypeDescription(12, 'registry')); });
	}

}