import type { Dispatch, SetStateAction } from "react";

export type ObjectValues<T> = T[keyof T];

export type SetState<T> = Dispatch<SetStateAction<T>>;
