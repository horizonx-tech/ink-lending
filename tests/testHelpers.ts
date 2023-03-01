import { ReturnNumber, Result } from '@727-ventures/typechain-types';
import Token from '../types/contracts/psp22_token';
import { expect } from '@jest/globals';
import { ReplacedType } from './utilityTypes';

export function parseUnits(amount: bigint | number, decimals = 18): bigint {
  return BigInt(amount) * 10n ** BigInt(decimals);
}

export const hexToUtf8 = (hexArray: number[]): string =>
  Buffer.from(hexArray.toString().replace('0x', ''), 'hex').toString('utf-8');

export const expectToEmit = <T = unknown>(
  event: { name: string; args: T },
  name: string,
  args: ReplacedType<T, ReturnNumber, number>,
): void => {
  expect(event.name).toBe(name);
  Object.keys(event.args).forEach((key) => {
    if (event.args[key] instanceof ReturnNumber)
      expect(event.args[key].toNumber()).toBe(args[key]);
    else expect(event.args[key]).toBe(args[key]);
  });
};

export function revertedWith(
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  result: { value: { err?: any } },
  // eslint-disable-next-line @typescript-eslint/no-explicit-any,@typescript-eslint/explicit-module-boundary-types
  errorTitle: any,
): void {
  if (result.value instanceof Result) {
    result.value = result.value.ok;
  }
  if (typeof errorTitle === 'object') {
    expect(result.value).toHaveProperty('err', errorTitle);
  } else {
    expect(result.value.err).toHaveProperty(errorTitle);
  }
}

export async function changeTokenBalances<T>(
  txThunk: () => Promise<T>,
  token: Token,
  actors: { address: string }[],
  expectedChanges: string[],
): Promise<T> {
  const accounts = actors.map((actor) => actor.address);
  const beforeBalances = await Promise.all(
    accounts.map(
      async (account) =>
        (
          await token.query.balanceOf(account)
        ).value.ok.rawNumber,
    ),
  );
  const result = await txThunk();
  const afterBalances = await Promise.all(
    accounts.map(
      async (account) =>
        (
          await token.query.balanceOf(account)
        ).value.ok.rawNumber,
    ),
  );
  const changes = afterBalances.map((afterBalance, i) =>
    afterBalance.sub(beforeBalances[i]).toString(),
  );
  expect(changes).toEqual(expectedChanges);
  return result;
}
