import { Dialog, type DialogProps } from "@/commons/components";
import { ResultTable } from "./result-table";
import type { WaveCalculationResponse } from "../../types";

interface ResultDialogProps extends Omit<DialogProps, "children"> {
  data?: WaveCalculationResponse;
}
export const ResultDialog = (props: ResultDialogProps) => (
  <Dialog
    setisOpen={props.setisOpen}
    title="Calculation Result"
    isOpen={props.isOpen}
  >
    <ResultTable data={props.data!} />
  </Dialog>
);
