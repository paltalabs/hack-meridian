import { NextApiRequest, NextApiResponse } from 'next';
import formidable from 'formidable';
import fs from 'fs';
import { pinFileToIPFS } from '@/utils/pinataHelper';

export const config = {
  api: {
    bodyParser: false, // Disallow body parsing, consume as stream
  },
};

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
  if (req.method === 'POST') {
    const form = formidable({ multiples: false });

    form.parse(req, async (err, fields, files) => {
      if (err) {
        console.error('Error parsing the files', err);
        return res.status(400).json({ message: 'Error parsing the files' });
      }

      const file = files.file;
      if (!file) {
        return res.status(400).json({ message: 'No file uploaded' });
      }

      let uploadedFile: formidable.File;

      // Check if 'file' is an array
      if (Array.isArray(file)) {
        uploadedFile = file[0]; // Use the first file if multiple were uploaded
      } else {
        uploadedFile = file;
      }

      const hash = Array.isArray(fields.hash) ? fields.hash[0] : fields.hash || 'DefaultHash';

      try {
        const fileStream = fs.createReadStream(uploadedFile.filepath);
        const fileHash = await pinFileToIPFS(fileStream, hash);

        // Construct the base URL dynamically
        const protocol = req.headers['x-forwarded-proto']
          ? Array.isArray(req.headers['x-forwarded-proto'])
            ? req.headers['x-forwarded-proto'][0]
            : req.headers['x-forwarded-proto']
          : 'http';

        const host = req.headers.host || 'localhost:3000'; // Default to localhost if host header is missing

        const baseUrl = `${protocol}://${host}`;

        // Create URLSearchParams to handle query parameters
        const params = new URLSearchParams({
          hash: hash,
          fileIpfsHash: fileHash,
        });

        const sign_url = `${baseUrl}/sign/?${params.toString()}`;

        const jsonContent = {
          hash: hash,
          file_url: `https://gateway.pinata.cloud/ipfs/${fileHash}`,
          sign_url: sign_url,
        };

        console.log('FILE URI:', jsonContent);

        return res.status(200).json({ jsonContent });
      } catch (error) {
        console.error('Error uploading to Pinata:', error);
        return res.status(500).json({ message: 'Error uploading to Pinata' });
      }
    });
  } else {
    res.status(405).json({ message: 'Method not allowed' });
  }
}