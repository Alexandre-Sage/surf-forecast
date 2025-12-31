import type { ObjectValues } from "../types";

export const COLORS = {
  GREEN: "green",
  ORANGE: "orange",
  RED: "red",
  BLACK: "black",
} as const;

export type Colors = ObjectValues<typeof COLORS>;
