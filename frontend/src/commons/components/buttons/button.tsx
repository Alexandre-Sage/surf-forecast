import { Box, Button as ChakraButton } from "@chakra-ui/react";

interface ButtonProps {
  onClick: () => void;
  loading?: boolean;
  label: string;
  variant?: "outline";
}

export const Button = (props: ButtonProps) => (
  <Box>
    <ChakraButton
      size={"md"}
      variant={props.variant}
      loading={props.loading}
      onClick={props.onClick}
    >
      {props.label}
    </ChakraButton>
  </Box>
);
