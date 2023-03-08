/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryOkJSON, queryJSON, handleReturnType } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/ui_data_provider';
import type * as ReturnTypes from '../types-returns/ui_data_provider';
import type BN from 'bn.js';
//@ts-ignore
import {ReturnNumber} from '@727-ventures/typechain-types';
import {getTypeDescription} from './../shared/utils';
// @ts-ignore
import type {EventRecord} from "@polkadot/api/submittable";
import {decodeEvents} from "../shared/utils";


export default class Methods {
	private __nativeContract : ContractPromise;
	private __keyringPair : KeyringPair;
	private __callerAddress : string;
	private __apiPromise: ApiPromise;

	constructor(
		apiPromise : ApiPromise,
		nativeContract : ContractPromise,
		keyringPair : KeyringPair,
	) {
		this.__apiPromise = apiPromise;
		this.__nativeContract = nativeContract;
		this.__keyringPair = keyringPair;
		this.__callerAddress = keyringPair.address;
	}

	/**
	* assets
	*
	* @returns { Result<Array<ReturnTypes.AccountId>, ReturnTypes.LangError> }
	*/
	"assets" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<Array<ReturnTypes.AccountId>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "assets", [], __options, (result) => { return handleReturnType(result, getTypeDescription(6, 'ui_data_provider')); });
	}

	/**
	* pools
	*
	* @param { Array<ArgumentTypes.AccountId> | null } assets,
	* @returns { Result<Array<ReturnTypes.PoolData>, ReturnTypes.LangError> }
	*/
	"pools" (
		assets: Array<ArgumentTypes.AccountId> | null,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<Array<ReturnTypes.PoolData>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "pools", [assets], __options, (result) => { return handleReturnType(result, getTypeDescription(9, 'ui_data_provider')); });
	}

	/**
	* marketData
	*
	* @param { Array<ArgumentTypes.AccountId> | null } assets,
	* @returns { Result<Array<ReturnTypes.MarketData>, ReturnTypes.LangError> }
	*/
	"marketData" (
		assets: Array<ArgumentTypes.AccountId> | null,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<Array<ReturnTypes.MarketData>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "marketData", [assets], __options, (result) => { return handleReturnType(result, getTypeDescription(14, 'ui_data_provider')); });
	}

}