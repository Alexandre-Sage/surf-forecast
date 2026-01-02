import { Box, Flex, Heading, Separator } from "@chakra-ui/react";
import type { ReactNode } from "react";

export const Page = ({
  children,
  title,
}: {
  children: ReactNode;
  title: string;
}) => {
  return (
    <Flex
      flexDir={"column"}
      width={"100vw"}
      height={"100vh"}
      alignItems={"center"}
      gapY={5}
    >
      <Flex flexDir={"column"} alignItems={"start"} width={"90vw"}>
        <Box>
          <Heading>{title}</Heading>
        </Box>
        <Separator width={"90vw"} />
      </Flex>
      <Flex
        width={"90vw"}
        minH={"70vh"}
        flexDir={"row"}
        justifyContent={"center"}
      >
        {children}
      </Flex>
    </Flex>
  );
};
