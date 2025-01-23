import {
  Box,
  Button,
  Flex,
  Heading,
  MenuContent,
  Stack,
} from "@chakra-ui/react";
import { ColorModeButton, ColorModeIcon } from "./color-mode";
import { ReactNode, useState } from "react";
import { FiAlignJustify } from "react-icons/fi";
import {
  DrawerActionTrigger,
  DrawerBackdrop,
  DrawerBody,
  DrawerCloseTrigger,
  DrawerContent,
  DrawerFooter,
  DrawerHeader,
  DrawerRoot,
  DrawerTitle,
  DrawerTrigger,
  useColorMode,
} from "./drawer";
import { useTranslation } from "react-i18next";
import { MenuRoot, MenuTrigger, MenuItem } from "./menu";
const MenuDrawer = () => {
  return (
    <DrawerRoot asChild placement="start">
      <DrawerBackdrop />
      <DrawerTrigger asChild>
        <Button variant="ghost" size="md">
          <FiAlignJustify></FiAlignJustify>
        </Button>
      </DrawerTrigger>
      <DrawerContent>
        <DrawerHeader>
          <DrawerTitle>Drawer Title</DrawerTitle>
        </DrawerHeader>
        <DrawerBody>
          <p>
            Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do
            eiusmod tempor incididunt ut labore et dolore magna aliqua.
          </p>
        </DrawerBody>
        <DrawerFooter>
          <DrawerActionTrigger asChild>
            <Button variant="outline">Cancel</Button>
          </DrawerActionTrigger>
          <Button>Save</Button>
        </DrawerFooter>
        <DrawerCloseTrigger />
      </DrawerContent>
    </DrawerRoot>
  );
};

const TranslationMenu = () => {
  const [language, setLanguage] = useState("en");
  const { t, i18n } = useTranslation();
  return (
    <MenuRoot>
      <MenuTrigger asChild>
        <Button>{language}</Button>
      </MenuTrigger>
      <MenuContent>
        <MenuItem>fr</MenuItem>
        <MenuItem>en</MenuItem>
      </MenuContent>
    </MenuRoot>
  );
};

export interface PageProps {
  children: ReactNode;
  title: string;
}
export const Page = (props: PageProps) => {
  const { t, i18n } = useTranslation();
  return (
    <Flex dir="row" width="100vw" alignContent="center" justifyContent="center">
      <Stack
        alignItems="center"
        justifyContent="space-between"
        flex="content"
        flexDir="column"
        width="95vw"
        height="90vh"
        border="1px solid red"
        //gapY="5"
      >
        <Stack
          borderBottomStyle="solid"
          borderBottomWidth="thin"
          width="95vw"
          flexDir="row"
          justifyContent="space-between"
          alignContent="space-between"
          //border="1px solid red"
        >
          <MenuDrawer />
          <Heading>{t(props.title)}</Heading>
          <Box>
            <ColorModeButton>
              <ColorModeIcon />
            </ColorModeButton>
          </Box>
        </Stack>
        <Stack
          //width="fit-content"
          flexDir="column"
          alignItems="center"
          justifyContent="center"
          height="80vh"
          width={"75vw"}
        >
          {props.children}
        </Stack>
      </Stack>
    </Flex>
  );
};
