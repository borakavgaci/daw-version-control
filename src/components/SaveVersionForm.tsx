import { FormEvent, useState } from "react";

interface SaveVersionFormProps {
  onSave: (note: string) => void;
}

export function SaveVersionForm({ onSave }: SaveVersionFormProps) {
  const [note, setNote] = useState("");

  function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    onSave(note.trim());
    setNote("");
  }

  return (
    <form className="save-version-form" onSubmit={handleSubmit}>
      <label htmlFor="version-note">Version note</label>
      <textarea
        id="version-note"
        onChange={(event) => setNote(event.currentTarget.value)}
        placeholder="Short note for this version"
        value={note}
      />
      <button className="primary-action" type="submit">
        Save New Version
      </button>
    </form>
  );
}
