import type {ReturnNumber} from "@727-ventures/typechain-types";
import type * as ReturnTypes from '../types-returns/shares_token';

export interface Transfer {
	from: ReturnTypes.AccountId | null;
	to: ReturnTypes.AccountId | null;
	value: ReturnNumber;
}

export interface Approval {
	owner: ReturnTypes.AccountId;
	spender: ReturnTypes.AccountId;
	value: ReturnNumber;
}

export interface OwnershipTransferred {
	previous: ReturnTypes.AccountId | null;
	new: ReturnTypes.AccountId | null;
}

