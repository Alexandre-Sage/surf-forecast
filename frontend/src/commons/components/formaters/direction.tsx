import { Tooltip } from "@/commons/components";
import type { ObjectValues } from "@/commons/types";
import { Icon } from "@chakra-ui/react";
// import { AiOutlineArrowDown } from "react-icons/ai";
import { FiChevronsDown } from "react-icons/fi";
import { match } from "ts-pattern";

const CARDINAL_DIRECTIONS = {
  NORTH: { short: "N", long: "North" },
  NORTH_NORTH_EAST: { short: "NNE", long: "North North East" },
  NORTH_EAST: { short: "NE", long: "North East" },
  EAST_NORTH_EAST: { short: "ENE", long: "East North East" },
  EAST: { short: "E", long: "East" },
  EAST_SOUTH_EAST: { short: "ESE", long: "East South East" },
  SOUTH_EAST: { short: "SE", long: "South East" },
  SOUTH_SOUTH_EAST: { short: "SSE", long: "South South East" },
  SOUTH: { short: "S", long: "South" },
  SOUTH_SOUTH_WEST: { short: "SSW", long: "South South West" },
  SOUTH_WEST: { short: "SW", long: "South West" },
  WEST_SOUTH_WEST: { short: "WSW", long: "West South West" },
  WEST: { short: "W", long: "West" },
  WEST_NORTH_WEST: { short: "WNW", long: "West North West" },
  NORTH_WEST: { short: "NW", long: "North West" },
  NORTH_NORTH_WEST: { short: "NNW", long: "North North West" },
} as const;

type CardinalDirection = ObjectValues<typeof CARDINAL_DIRECTIONS>;
// const cardinalRange = (start: number, end: number) => (value: number) =>
//   value >= start && value < end;

const translation = (value: number): CardinalDirection =>
  match(value)
    .when(
      (v) => v >= 348.75 || v < 11.25,
      () => CARDINAL_DIRECTIONS.NORTH
    )
    .when(
      (v) => v >= 11.25 && v < 33.75,
      () => CARDINAL_DIRECTIONS.NORTH_NORTH_EAST
    )
    .when(
      (v) => v >= 33.75 && v < 56.25,
      () => CARDINAL_DIRECTIONS.NORTH_EAST
    )
    .when(
      (v) => v >= 56.25 && v < 78.75,
      () => CARDINAL_DIRECTIONS.EAST_NORTH_EAST
    )
    .when(
      (v) => v >= 78.75 && v < 101.25,
      () => CARDINAL_DIRECTIONS.EAST
    )
    .when(
      (v) => v >= 101.25 && v < 123.75,
      () => CARDINAL_DIRECTIONS.EAST_SOUTH_EAST
    )
    .when(
      (v) => v >= 123.75 && v < 146.25,
      () => CARDINAL_DIRECTIONS.SOUTH_EAST
    )
    .when(
      (v) => v >= 146.25 && v < 168.75,
      () => CARDINAL_DIRECTIONS.SOUTH_SOUTH_EAST
    )
    .when(
      (v) => v >= 168.75 && v < 191.25,
      () => CARDINAL_DIRECTIONS.SOUTH
    )
    .when(
      (v) => v >= 191.25 && v < 213.75,
      () => CARDINAL_DIRECTIONS.SOUTH_SOUTH_WEST
    )
    .when(
      (v) => v >= 213.75 && v < 236.25,
      () => CARDINAL_DIRECTIONS.SOUTH_WEST
    )
    .when(
      (v) => v >= 236.25 && v < 258.75,
      () => CARDINAL_DIRECTIONS.WEST_SOUTH_WEST
    )
    .when(
      (v) => v >= 258.75 && v < 281.25,
      () => CARDINAL_DIRECTIONS.WEST
    )
    .when(
      (v) => v >= 281.25 && v < 303.75,
      () => CARDINAL_DIRECTIONS.WEST_NORTH_WEST
    )
    .when(
      (v) => v >= 303.75 && v < 326.25,
      () => CARDINAL_DIRECTIONS.NORTH_WEST
    )
    .when(
      (v) => v >= 326.25 && v < 348.75,
      () => CARDINAL_DIRECTIONS.NORTH_NORTH_WEST
    )
    .run();

export const DirectionFormater = ({ value }: { value: number }) => (
  <Tooltip showArrow content={`(${value}°) ${translation(value).short}`}>
    <Icon size={"sm"} transform={`rotate(${value}deg)`}>
      {/* <AiOutlineArrowDown /> */}
      <FiChevronsDown />
    </Icon>
  </Tooltip>
);
