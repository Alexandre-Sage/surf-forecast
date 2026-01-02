import { Dialog as ChakraDialog, Portal } from "@chakra-ui/react";
import { CloseButton } from "./close-button";
import type { SetState } from "@/commons/types";
import type { ReactNode } from "react";

// export const Triger = () => (
//   <ChakraDialog.Trigger asChild>
//     <Button label="Open" onClick={() => {}} />
//   </ChakraDialog.Trigger>
// );
//
const DialogHeader = (props: { title: string }) => (
  <ChakraDialog.Header>
    <ChakraDialog.Title>{props.title}</ChakraDialog.Title>
  </ChakraDialog.Header>
);

const DialogCloseTrigger = (props: { setIsOpen: SetState<boolean> }) => (
  <ChakraDialog.CloseTrigger asChild>
    <CloseButton size="sm" onClick={() => props.setIsOpen(false)} />
  </ChakraDialog.CloseTrigger>
);

export interface DialogProps {
  isOpen: boolean;
  setisOpen: SetState<boolean>;
  children: ReactNode;
  title?: string;
}

export const Dialog = (props: DialogProps) => {
  return (
    <ChakraDialog.Root open={props.isOpen}>
      <Portal>
        <ChakraDialog.Backdrop />
        <ChakraDialog.Positioner>
          <ChakraDialog.Content>
            {props.title ? <DialogHeader title={props.title} /> : null}
            <ChakraDialog.Body>{props.children}</ChakraDialog.Body>
            <DialogCloseTrigger setIsOpen={props.setisOpen} />
          </ChakraDialog.Content>
        </ChakraDialog.Positioner>
      </Portal>
    </ChakraDialog.Root>
  );
};
