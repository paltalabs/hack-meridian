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

      const fileName = (fields.fileName as unknown as string) || 'DefaultNFTName';

      try {
        const fileStream = fs.createReadStream(uploadedFile.filepath);
        const nftUri = await pinFileToIPFS(fileStream, fileName);
        return res.status(200).json({ nftUri });
      } catch (error) {
        console.error('Error uploading to Pinata:', error);
        return res.status(500).json({ message: 'Error uploading to Pinata' });
      }
    });
  } else {
    res.status(405).json({ message: 'Method not allowed' });
  }
}