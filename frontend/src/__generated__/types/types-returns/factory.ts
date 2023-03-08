import type BN from 'bn.js';
import type {ReturnNumber} from '@727-ventures/typechain-types';

export type AccountId = string | number[]

export type Hash = string | number[]

export enum LangError {
	couldNotReadInput = 'CouldNotReadInput'
}

export interface factory_Error {
	poolImplementationMissing ? : null,
	callerIsNotManager ? : null,
	registry ? : registry_Error,
	custom ? : number
}

export class factory_ErrorBuilder {
	static PoolImplementationMissing(): factory_Error {
		return {
			poolImplementationMissing: null,
		};
	}
	static CallerIsNotManager(): factory_Error {
		return {
			callerIsNotManager: null,
		};
	}
	static Registry(value: registry_Error): factory_Error {
		return {
			registry: value,
		};
	}
	static Custom(value: number): factory_Error {
		return {
			custom: value,
		};
	}
}

export enum registry_Error {
	poolAlreadyExists = 'PoolAlreadyExists',
	callerIsNotManager = 'CallerIsNotManager'
}

