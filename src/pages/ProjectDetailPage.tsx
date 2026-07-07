import { SaveVersionForm } from "../components/SaveVersionForm";
import { VersionCard } from "../components/VersionCard";
import type { Project } from "../types/project";

interface ProjectDetailPageProps {
  project: Project;
}

export function ProjectDetailPage({ project }: ProjectDetailPageProps) {
  return (
    <main className="app-shell">
      <section className="projects-page" aria-labelledby="project-title">
        <div className="detail-panel">
          <p className="eyebrow">Project detail</p>
          <h1 id="project-title">{project.name}</h1>
          <p className="meta-text">{project.sourcePath}</p>
        </div>

        <SaveVersionForm onSave={() => undefined} />

        <ul className="version-list">
          {project.versions.map((version) => (
            <li key={version.number}>
              <VersionCard version={version} />
            </li>
          ))}
        </ul>
      </section>
    </main>
  );
}
