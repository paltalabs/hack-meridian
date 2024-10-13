import pinataSDK from '@pinata/sdk';

const PINATA_API_KEY = process.env.PINATA_API_KEY;
const PINATA_API_SECRET = process.env.PINATA_API_SECRET;

export async function pinFileToIPFS(fileStream: NodeJS.ReadableStream, fileName: string): Promise<string> {
  if (!PINATA_API_KEY || !PINATA_API_SECRET) {
    console.error('Pinata API Key or Secret is undefined.');
    throw new Error('Pinata API Key or Secret is undefined');
  }

  const pinata = new pinataSDK(PINATA_API_KEY, PINATA_API_SECRET);

  const options = {
    pinataMetadata: {
      name: fileName,
    },
  };
  
  const result = await pinata.pinFileToIPFS(fileStream, options);
  console.log('File pinned:', result);
  const ipfsHash = result.IpfsHash;

  return ipfsHash;
}