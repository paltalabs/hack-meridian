import { ReactNode, useRef } from 'react'
import { Button, Flex, FormControl, FormErrorMessage, FormLabel, Icon, InputGroup } from '@chakra-ui/react'
import { useForm, UseFormRegisterReturn } from 'react-hook-form'
import { FiFile } from 'react-icons/fi'
import { fetchPayrollAddress } from '@/utils/payrollVault'
import { useSorobanReact } from '@soroban-react/core'

type FileUploadProps = {
  register: UseFormRegisterReturn
  accept?: string
  children?: ReactNode
}

const FileUpload = (props: FileUploadProps) => {
  const { register, accept, children } = props
  const inputRef = useRef<HTMLInputElement | null>(null)
  const { ref, ...rest } = register as {ref: (instance: HTMLInputElement | null) => void}

  const handleClick = () => inputRef.current?.click()

  return (
      <InputGroup onClick={handleClick}>
        <input
          type={'file'}
          multiple={false}
          hidden
          accept={accept}
          {...rest}
          ref={(e) => {
            ref(e)
            inputRef.current = e
          }}
        />
        <>
          {children}
        </>
      </InputGroup>
  )
}

type FormValues = {
  file_: FileList
}

const UploadComponent = ({ setFileHash, setSignUrl, employee }: { setFileHash: any, setSignUrl: any, employee: string }) => {
  const { address, activeChain } = useSorobanReact()
  const { register, handleSubmit, formState: { errors } } = useForm<FormValues>()

  // Function to calculate SHA-256 hash (you can replace this with MD5)
  const calculateHash = async (file: File) => {
    return new Promise<string>((resolve, reject) => {
      const reader = new FileReader();
      reader.readAsArrayBuffer(file);

      reader.onload = async (event) => {
        if (event.target?.result) {
          const buffer = event.target.result as ArrayBuffer;

          // Calculate SHA-256 hash using SubtleCrypto
          const hashBuffer = await crypto.subtle.digest('SHA-256', buffer);
          const hashArray = Array.from(new Uint8Array(hashBuffer));
          const hashHex = hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
          resolve(hashHex);
        }
      };

      reader.onerror = () => reject(new Error('File reading failed'));
    });
  }

  const onSubmit = handleSubmit(async (data) => {
    if (!address || !activeChain) return
    const payrollAddress = fetchPayrollAddress(activeChain?.id)

    const file = data.file_[0]; // Get the first file from the FileList
    if (file) {
      const hash = await calculateHash(file);
      console.log('File:', file.name);
      console.log('SHA-256 Hash:', hash); // Log the hash to the console

      const formData = new FormData();
      formData.append('file', file);
      formData.append('hash', hash);

      try {
        const response = await fetch('/api/pinata', {
          method: 'POST',
          body: formData,
        });

        const result = await response.json();
        if (response.ok) {
          console.log('FILE URI:', result.jsonContent);
          setFileHash(result.jsonContent.hash);
          setSignUrl(`${result.jsonContent.sign_url}&employer=${address}&employee=${employee}&vaultAddress=${payrollAddress}`);
        } else {
          console.error('Error:', result.message);
        }
      } catch (error) {
        console.error('Error uploading file:', error);
      }
    } else {
      console.error('No file selected');
      return;
    }
  })

  const validateFiles = (value: FileList) => {
    if (value.length < 1) {
      return 'File is required'
    }
    for (const file of Array.from(value)) {
      const fsMb = file.size / (1024 * 1024)
      const MAX_FILE_SIZE = 10
      if (fsMb > MAX_FILE_SIZE) {
        return 'Max file size 10MB'
      }
    }
    return true
  }

  return (
    <>
      <form onSubmit={onSubmit}>
        <Flex> 
          <FormControl isInvalid={!!errors.file_} isRequired>
              <FileUpload
                accept={'image/*,application/pdf'}
                register={register('file_', { validate: validateFiles })}
                >
                <Button leftIcon={<Icon as={FiFile} />}>
                  Select File
                </Button>
              </FileUpload>

            <FormErrorMessage>
              {errors.file_ && errors?.file_.message}
            </FormErrorMessage>
          </FormControl>

          <Button ml={5} type="submit">Upload</Button>
        </Flex>
      </form>
    </>
  )
}

export default UploadComponent