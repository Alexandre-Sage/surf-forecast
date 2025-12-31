import type { ObjectValues } from "./utils.type";

export const UNIT = {
  METER: "meter",
  DEGREE: "degree",
  SECOND: "second",
} as const;

export type Unit = ObjectValues<typeof UNIT>;
