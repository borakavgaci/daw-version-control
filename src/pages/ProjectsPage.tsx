import { AddProjectButton } from "../components/AddProjectButton";
import { ProjectCard } from "../components/ProjectCard";
import type { Project } from "../types/project";

export function ProjectsPage() {
  const projects: Project[] = [];

  return (
    <main className="app-shell">
      <section className="projects-page" aria-labelledby="projects-title">
        <header className="page-header">
          <div>
            <p className="eyebrow">Local macOS archive</p>
            <h1 id="projects-title">Logic Version Archive</h1>
            <p className="page-description">
              Keep full local snapshots of Logic Pro package projects before
              diffing, branching, or cloud features are introduced.
            </p>
          </div>
          <AddProjectButton />
        </header>

        {projects.length === 0 ? (
          <div className="empty-state">
            <div>
              <h2>No Logic projects yet</h2>
              <p>
                Add a .logicx package when the next milestone connects the
                button to the macOS file picker.
              </p>
            </div>
          </div>
        ) : (
          <ul className="project-list">
            {projects.map((project) => (
              <li key={project.id}>
                <ProjectCard project={project} />
              </li>
            ))}
          </ul>
        )}
      </section>
    </main>
  );
}
