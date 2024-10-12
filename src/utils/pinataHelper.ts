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

  // Pin the file to IPFS
  const result = await pinata.pinFileToIPFS(fileStream, options);
  console.log('File pinned:', result);
  const ipfsHash = result.IpfsHash;

  const jsonOptions = {
    pinataMetadata: {
      name: `${fileName}.json`,
    },
  };

  const jsonContent = {
    name: fileName,
    img_url: `https://gateway.pinata.cloud/ipfs/${ipfsHash}`,
  };

  // Pin the JSON content to IPFS
  const jsonResult = await pinata.pinJSONToIPFS(jsonContent, jsonOptions);
  const fileUri = `https://gateway.pinata.cloud/ipfs/${jsonResult.IpfsHash}`;
  console.log('FILE URI:', fileUri);

  return fileUri;
}