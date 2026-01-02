import { Table } from "@/commons/components";
import type { WaveCalculationResponse } from "../../types";

export const ResultTable = ({ data }: { data: WaveCalculationResponse }) => {
  return (
    <Table
      columns={[
        { field: "rss", id: "rss" },
        {
          field: "rssDirectional",
          id: "rssDirectional",
        },
      ]}
      data={[data]}
    />
  );
};
