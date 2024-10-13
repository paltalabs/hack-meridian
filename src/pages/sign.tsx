import { useRouter } from 'next/router';
import { useEffect, useState } from 'react';
import { Button, Box, Spinner, Text } from '@chakra-ui/react';

const SignPage = () => {
  const router = useRouter();
  const { hash, fileIpfsHash } = router.query;
  const [fileUrl, setFileUrl] = useState<string | null>(null);
  const [fileType, setFileType] = useState<string | null>(null);
  const [hashMatch, setHashMatch] = useState<boolean | null>(null);

  useEffect(() => {
    const fetchFileData = async () => {
      if (fileIpfsHash && hash) {
        const url = `https://gateway.pinata.cloud/ipfs/${fileIpfsHash}`;
        setFileUrl(url);

        try {
          // Fetch the file's content type
          const headResponse = await fetch(url, { method: 'HEAD' });
          const contentType = headResponse.headers.get('Content-Type');
          if (contentType) {
            setFileType(contentType);
          }

          // Fetch the file content
          const response = await fetch(url);
          const arrayBuffer = await response.arrayBuffer();

          // Compute the SHA-256 hash of the file content
          const hashBuffer = await crypto.subtle.digest('SHA-256', arrayBuffer);
          const hashArray = Array.from(new Uint8Array(hashBuffer));
          const computedHash = hashArray
            .map((b) => b.toString(16).padStart(2, '0'))
            .join('');

          // Compare the computed hash with the 'hash' parameter
          if (computedHash === hash) {
            setHashMatch(true);
          } else {
            setHashMatch(false);
          }
        } catch (error) {
          console.error('Error fetching file or computing hash:', error);
        }
      }
    };

    fetchFileData();
  }, [fileIpfsHash, hash]);

  const handleSign = () => {
    // Implement your signing logic here
    console.log('Signing hash:', hash);
  };

  const handleDownload = () => {
    if (fileUrl) {
      // Create a temporary link and trigger download
      const link = document.createElement('a');
      link.href = fileUrl;
      link.download = ''; // Let the browser decide the filename
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
    }
  };

  if (!fileUrl) {
    return (
      <Box p={4}>
        <Spinner />
        <Text>Loading...</Text>
      </Box>
    );
  }

  return (
    <Box p={4}>
      <Text fontSize="2xl" mb={4}>
        Sign Document
      </Text>

      {hashMatch === null ? (
        <Box mb={4}>
          <Spinner />
          <Text>Verifying file integrity...</Text>
        </Box>
      ) : hashMatch ? (
        <Text color="green.500" mb={4}>
          File integrity verified. Hashes match.
        </Text>
      ) : (
        <Text color="red.500" mb={4}>
          File integrity verification failed. Hashes do not match.
        </Text>
      )}

      <Box mb={4}>
        {fileType?.includes('application/pdf') ? (
          <iframe
            src={fileUrl}
            width="100%"
            height="600px"
            style={{ border: 'none' }}
          />
        ) : fileType?.includes('image/') ? (
          <img
            src={fileUrl}
            alt="Uploaded File"
            style={{ maxWidth: '100%', height: 'auto' }}
          />
        ) : (
          <Text>Unable to display file. Unsupported format.</Text>
        )}
      </Box>
      <Button
        colorScheme="blue"
        onClick={handleSign}
        mr={2}
        isDisabled={hashMatch === false}
      >
        Sign
      </Button>
      <Button colorScheme="teal" onClick={handleDownload}>
        Download
      </Button>
    </Box>
  );
};

export default SignPage;