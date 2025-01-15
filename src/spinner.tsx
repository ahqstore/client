import { Spinner } from "@fluentui/react-components"

export default function ShowSpinner() {
  return (
    <div className="flex flex-col">
      <Spinner appearance="inverted" size="extra-large" />
      {/* <span className="text-lg">Welcome to AHQ Store</span> */}
    </div>
  );
}