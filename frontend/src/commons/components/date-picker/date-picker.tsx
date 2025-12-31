import { Flex, Input } from "@chakra-ui/react";
import dayjs from "dayjs";

interface DatePickerProps {
  onChange: (date: Date) => void;
  selected: Date | null;
  label?: string;
}
export const DatePicker = (props: DatePickerProps) => {
  return (
    <Flex flexDir={"column"} width={"15vw"}>
      {props.label ? <label>{props.label}</label> : null}
      <Input
        size={"xs"}
        type="date"
        onChange={(event) => props.onChange(new Date(event.target.value))}
        value={dayjs(props.selected).format("YYYY-MM-DD")}
      />
    </Flex>
  );
};
