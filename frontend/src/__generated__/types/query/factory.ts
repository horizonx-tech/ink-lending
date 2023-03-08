/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryJSON, queryOkJSON, handleReturnType } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/factory';
import type * as ReturnTypes from '../types-returns/factory';
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
	* registry
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"registry" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "registry", [], __options , (result) => { return handleReturnType(result, getTypeDescription(7, 'factory')); });
	}

	/**
	* poolCodeHash
	*
	* @returns { Result<ReturnTypes.Hash, ReturnTypes.LangError> }
	*/
	"poolCodeHash" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Hash, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "poolCodeHash", [], __options , (result) => { return handleReturnType(result, getTypeDescription(8, 'factory')); });
	}

	/**
	* sharesCodeHash
	*
	* @returns { Result<ReturnTypes.Hash, ReturnTypes.LangError> }
	*/
	"sharesCodeHash" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Hash, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "sharesCodeHash", [], __options , (result) => { return handleReturnType(result, getTypeDescription(8, 'factory')); });
	}

	/**
	* create
	*
	* @param { ArgumentTypes.AccountId } asset,
	* @param { Array<(number | string | BN)> } data,
	* @returns { Result<Result<ReturnTypes.AccountId, ReturnTypes.factory_Error>, ReturnTypes.LangError> }
	*/
	"create" (
		asset: ArgumentTypes.AccountId,
		data: Array<(number | string | BN)>,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<ReturnTypes.AccountId, ReturnTypes.factory_Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "factory::create", [asset, data], __options , (result) => { return handleReturnType(result, getTypeDescription(10, 'factory')); });
	}

}