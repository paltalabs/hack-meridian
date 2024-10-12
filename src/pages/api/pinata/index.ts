import { NextApiRequest, NextApiResponse } from "next";

async function handler(req: NextApiRequest, res: NextApiResponse) {
  res.status(200).json({ text: 'Hello' });
}

export default handler;