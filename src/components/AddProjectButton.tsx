interface AddProjectButtonProps {
  disabled?: boolean;
  onClick?: () => void;
}

export function AddProjectButton({
  disabled = false,
  onClick,
}: AddProjectButtonProps) {
  return (
    <button
      className="primary-action"
      disabled={disabled}
      onClick={onClick}
      type="button"
    >
      Add Logic Project
    </button>
  );
}
