import {
  Button,
  ButtonProps,
  Dialog as ChakraDialog,
  DialogContext,
  DialogRootProps,
  Portal,
  UseDialogReturn,
} from "@chakra-ui/react";
import { CloseButton } from "./close-button";
import * as React from "react";
import { useTranslation } from "react-i18next";
import { Translated } from "./text";

interface DialogContentProps extends ChakraDialog.ContentProps {
  portalled?: boolean;
  portalRef?: React.RefObject<HTMLElement>;
  backdrop?: boolean;
}

export const DialogContent = React.forwardRef<
  HTMLDivElement,
  DialogContentProps
>(function DialogContent(props, ref) {
  const {
    children,
    portalled = true,
    portalRef,
    backdrop = true,
    ...rest
  } = props;

  return (
    <Portal disabled={!portalled} container={portalRef}>
      {backdrop && <ChakraDialog.Backdrop />}
      <ChakraDialog.Positioner>
        <ChakraDialog.Content ref={ref} {...rest} asChild={false}>
          {children}
        </ChakraDialog.Content>
      </ChakraDialog.Positioner>
    </Portal>
  );
});

export const DialogCloseTrigger = React.forwardRef<
  HTMLButtonElement,
  Omit<ChakraDialog.CloseTriggerProps, "children"> & {
    children?: React.ReactNode;
  }
>(function DialogCloseTrigger(props, ref) {
  return (
    <ChakraDialog.CloseTrigger
      position="absolute"
      top="2"
      insetEnd="2"
      {...props}
      asChild
    >
      <CloseButton size="sm" ref={ref}>
        {props.children}
      </CloseButton>
    </ChakraDialog.CloseTrigger>
  );
});

export const DialogRoot = ChakraDialog.Root;
export const DialogFooter = ChakraDialog.Footer;
export const DialogHeader = ChakraDialog.Header;
export const DialogBody = ChakraDialog.Body;
export const DialogBackdrop = ChakraDialog.Backdrop;
export const DialogTitle = ChakraDialog.Title;
export const DialogDescription = ChakraDialog.Description;
export const DialogTrigger = ChakraDialog.Trigger;
export const DialogActionTrigger = ChakraDialog.ActionTrigger;

export interface DialogProps {
  motion?: DialogRootProps["motionPreset"];
  placement?: DialogRootProps["placement"];
  title?: string;
  triggerTitle: string;
  children: React.ReactNode;
  triggerButtonProps?: ButtonProps;
}

export const Dialog = (props: DialogProps) => {
  const { t } = useTranslation();
  return (
    <DialogRoot placement={props.placement} motionPreset={props.motion}>
      <DialogTrigger asChild>
        <Button {...props.triggerButtonProps}>{t(props.triggerTitle)}</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <Translated>{props.title}</Translated>
          <DialogCloseTrigger />
        </DialogHeader>
        <DialogBody>{props.children}</DialogBody>
      </DialogContent>
    </DialogRoot>
  );
};

interface DialogWithButtonProps extends DialogProps {
  onSave: (_: UseDialogReturn) => unknown;
  actionDisabled?: boolean;
}
export const DialogWithButton = (props: DialogWithButtonProps) => {
  const { t } = useTranslation();
  return (
    <DialogRoot placement={props.placement} motionPreset={props.motion}>
      <DialogTrigger asChild>
        <Button {...props.triggerButtonProps}>{t(props.triggerTitle)}</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <Translated>{props.title}</Translated>
          <DialogCloseTrigger />
        </DialogHeader>
        <DialogBody>{props.children}</DialogBody>
        <DialogFooter>
          <DialogActionTrigger asChild>
            <Button variant="outline">Cancel</Button>
          </DialogActionTrigger>
          <DialogContext>
            {(_) => (
              <Button
                onClick={() => props.onSave(_)}
                colorPalette={"teal"}
                disabled={props.actionDisabled}
              >
                Save
              </Button>
            )}
          </DialogContext>
        </DialogFooter>
        <DialogCloseTrigger />
      </DialogContent>
    </DialogRoot>
  );
};
