import React from "react";

interface FlatButtonProps {
  children: React.ReactNode;
  onClick?: () => void;
  disabled?: boolean;
}

export default function FluentButton({ children, onClick, disabled }: FlatButtonProps) {
  return (
    <button
      onClick={disabled ? undefined : onClick}
      disabled={disabled}
      className={`
        px-5 py-2 
        rounded-md font-medium
        transition-colors duration-150
        focus:outline-none
        cursor-pointer
        ${disabled
          ? "bg-gray-200 text-gray-400 dark:bg-gray-700 dark:text-gray-500 cursor-not-allowed"
          : "bg-white text-black hover:bg-gray-100 active:bg-gray-200 dark:bg-neutral-800 dark:text-white dark:hover:bg-neutral-700 dark:active:bg-neutral-600"
        }
      `}
    >
      {children}
    </button>
  );
}
