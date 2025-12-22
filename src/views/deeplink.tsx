import { useHome } from "@/lib/data";
import {
  Dialog,
  DialogTrigger,
  DialogSurface,
  DialogTitle,
  DialogBody,
  DialogContent,
  DialogActions,
  Button,
} from "@fluentui/react-components";

export interface DeepLinkMeta {
  isOpen: boolean;
  task: {
    type: "app",
    id: string
  } | {
    type: "category",
    id: number;
  };
}

export const close: DeepLinkMeta = {
  isOpen: false,
  task: {
    type: "category",
    id: 0
  }
};

interface Props {
  meta: DeepLinkMeta;
  setMeta: (_: DeepLinkMeta) => void;
  set: (_: number) => void;
}

export const DeepLink = ({ meta: { isOpen, task }, setMeta, set }: Props) => {
  const home = useHome();

  return (
    <Dialog
      open={isOpen}
    >
      {/* <DialogTrigger disableButtonEnhancement>
        <></>
      </DialogTrigger> */}
      <DialogSurface >
        <DialogBody className="!bg-transparent !max-h-[80vh]">
          <DialogTitle>Please Note</DialogTitle>
          <DialogContent>
            {
              task.type == "category" &&
              <>
                <p>
                  A deep link has requested AHQ Store to navigate to home screen group named <code className="border border-foreground-content border-dashed px-1 rounded-md">{home!![task.id][0]}</code>.
                </p>

                <p className="mt-3 text-warning">
                  <strong>Would you like AHQ Store to acknowledge this interaction by performing the navigation action?</strong>
                </p>
              </>
            }
          </DialogContent>
          <DialogActions>
            <DialogTrigger>
              <Button appearance="subtle" onClick={() => setMeta(close)}>No</Button>
            </DialogTrigger>
            <DialogTrigger disableButtonEnhancement>
              <Button appearance="primary" onClick={() => {
                set(10);
                setMeta(close);
              }}>Yes</Button>
            </DialogTrigger>
          </DialogActions>
        </DialogBody>
      </DialogSurface>
    </Dialog>
  );
};
