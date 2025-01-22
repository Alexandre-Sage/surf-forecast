import { Text, TextProps } from "@chakra-ui/react";
import { ReactNode } from "react";
import { useTranslation } from "react-i18next";

interface TranslatedProps extends TextProps {
  children: string | ReactNode;
}
export const Translated = (props: TranslatedProps) => {
  const { t } = useTranslation();
  return <Text>{t(props.children as string)}</Text>;
};
