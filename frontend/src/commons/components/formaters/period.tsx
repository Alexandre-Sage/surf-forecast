import { COLORS, type Colors } from "@/commons/styles";
import { Text } from "@chakra-ui/react";

const getColor = (value: number): Colors => {
  if (value < 5.0) return COLORS.GREEN;
  if (value < 10.0) return COLORS.ORANGE;
  return COLORS.RED;
};

export const PeriodFormater = ({ value }: { value: number }) => (
  <Text width={50} color={getColor(value)}>
    {value} s
  </Text>
);
