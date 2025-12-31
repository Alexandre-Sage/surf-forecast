import { SegmentGroup } from "@chakra-ui/react";
import { HOURS } from "./hours";
import type { SetState } from "@/commons/types";
import { Time } from "@/commons/types/time";

interface HoursControlProps {
  onChange: (value: Time | null) => void | SetState<Time | null>;
  value: Time | null;
}

export const HoursControl = (props: HoursControlProps) => {
  return (
    <SegmentGroup.Root
      size={"xs"}
      value={(props.value ?? HOURS[0]!).toString()}
      onValueChange={(e) => {
        return props.onChange(Time.fromString(e.value!)!);
      }}
    >
      <SegmentGroup.Indicator />
      <SegmentGroup.Items items={HOURS.map((hours) => hours!.toString())} />
      {/* <SegmentGroup.Item value="" onClick={() => console.log("play")}> */}
      {/*   <FiPlay /> */}
      {/* </SegmentGroup.Item> */}
    </SegmentGroup.Root>
  );
};
