export function shortenAddress(address: string, size: number): string {
  if (address.length !== 56) {
    throw new Error('Invalid address length');
  }

  const firstThree = address.slice(0, size);
  const lastThree = address.slice(-size);

  return `${firstThree}...${lastThree}`;
}