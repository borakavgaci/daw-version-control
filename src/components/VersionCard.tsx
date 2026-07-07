import type { ProjectVersion } from "../types/project";

interface VersionCardProps {
  version: ProjectVersion;
}

export function VersionCard({ version }: VersionCardProps) {
  return (
    <article className="version-card">
      <h3>Version {version.number}</h3>
      <p className="meta-text">{version.createdAt}</p>
      {version.note.length > 0 ? <p>{version.note}</p> : null}
    </article>
  );
}
