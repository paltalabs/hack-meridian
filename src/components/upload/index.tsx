import { ReactNode, useRef } from 'react';
import {
  Button,
  FormControl,
  FormErrorMessage,
  FormLabel,
  Icon,
  InputGroup,
} from '@chakra-ui/react';
import { useForm } from 'react-hook-form';
import { FiFile } from 'react-icons/fi';

type FileUploadProps = {
  register: any;
  accept?: string;
  multiple?: boolean;
  children?: ReactNode;
};

const FileUpload = (props: FileUploadProps) => {
  const { register, accept, multiple, children } = props;
  const inputRef = useRef<HTMLInputElement | null>(null);
  const { ref, ...rest } = register;

  const handleClick = () => inputRef.current?.click();

  return (
    <InputGroup onClick={handleClick}>
      <input
        type="file"
        multiple={multiple || false}
        hidden
        accept={accept}
        {...rest}
        ref={(e) => {
          ref(e);
          inputRef.current = e;
        }}
      />
      {children}
    </InputGroup>
  );
};

type FormValues = {
  file_: FileList;
};

const UploadComponent = () => {
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<FormValues>();

  const onSubmit = async (data: FormValues) => {
    console.log('On Submit: ', data);

    const file = data.file_[0];
    if (!file) {
      console.error('No file selected');
      return;
    }

    const formData = new FormData();
    formData.append('file', file);
    formData.append('nftName', 'MyNFT'); // You can get this from an input field

    try {
      const response = await fetch('/api/pinata', {
        method: 'POST',
        body: formData,
      });

      const result = await response.json();
      if (response.ok) {
        console.log('NFT URI:', result.nftUri);
      } else {
        console.error('Error:', result.message);
      }
    } catch (error) {
      console.error('Error uploading file:', error);
    }
  };

  const validateFiles = (value: FileList) => {
    if (value.length < 1) {
      return 'File is required';
    }
    const file = value[0];
    const fsMb = file.size / (1024 * 1024);
    const MAX_FILE_SIZE = 10;
    if (fsMb > MAX_FILE_SIZE) {
      return 'Max file size is 10MB';
    }
    return true;
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <FormControl isInvalid={!!errors.file_} isRequired>
        <FormLabel>File input</FormLabel>

        <FileUpload
          accept="image/*,application/pdf"
          register={register('file_', { validate: validateFiles })}
        >
          <Button leftIcon={<Icon as={FiFile} />}>Upload</Button>
        </FileUpload>

        <FormErrorMessage>
          {errors.file_ && errors.file_.message}
        </FormErrorMessage>
      </FormControl>

      <Button type="submit">Submit</Button>
    </form>
  );
};

export default UploadComponent;