import type BN from 'bn.js';
import type {ReturnNumber} from '@727-ventures/typechain-types';

export type AccountId = string | number[]

export enum LangError {
	couldNotReadInput = 'CouldNotReadInput'
}

export interface service_Error {
	poolNotFound ? : null,
	assetPool ? : asset_pool_Error,
	risk ? : number
}

export class service_ErrorBuilder {
	static PoolNotFound(): service_Error {
		return {
			poolNotFound: null,
		};
	}
	static AssetPool(value: asset_pool_Error): service_Error {
		return {
			assetPool: value,
		};
	}
	static Risk(value: number): service_Error {
		return {
			risk: value,
		};
	}
}

export interface asset_pool_Error {
	psp22 ? : PSP22Error,
	risk ? : number
}

export class asset_pool_ErrorBuilder {
	static PSP22(value: PSP22Error): asset_pool_Error {
		return {
			psp22: value,
		};
	}
	static Risk(value: number): asset_pool_Error {
		return {
			risk: value,
		};
	}
}

export interface PSP22Error {
	custom ? : Array<number>,
	insufficientBalance ? : null,
	insufficientAllowance ? : null,
	zeroRecipientAddress ? : null,
	zeroSenderAddress ? : null,
	safeTransferCheckFailed ? : Array<number>
}

export class PSP22ErrorBuilder {
	static Custom(value: Array<number>): PSP22Error {
		return {
			custom: value,
		};
	}
	static InsufficientBalance(): PSP22Error {
		return {
			insufficientBalance: null,
		};
	}
	static InsufficientAllowance(): PSP22Error {
		return {
			insufficientAllowance: null,
		};
	}
	static ZeroRecipientAddress(): PSP22Error {
		return {
			zeroRecipientAddress: null,
		};
	}
	static ZeroSenderAddress(): PSP22Error {
		return {
			zeroSenderAddress: null,
		};
	}
	static SafeTransferCheckFailed(value: Array<number>): PSP22Error {
		return {
			safeTransferCheckFailed: value,
		};
	}
}

