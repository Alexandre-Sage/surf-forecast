import { COLORS, type Colors } from "@/commons/styles";
import { Text } from "@chakra-ui/react";

const getColor = (value: number): Colors => {
  if (value < 1.0) return COLORS.GREEN;
  if (value < 1.5) return COLORS.ORANGE;
  return COLORS.RED;
};

export const SwellFormater = (props: { value: number }) => {
  return <Text color={getColor(props.value)}>{props.value} m</Text>;
};
