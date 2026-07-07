import type { Project } from "../types/project";

interface ProjectCardProps {
  project: Project;
  onSelect?: (projectId: string) => void;
}

export function ProjectCard({ project, onSelect }: ProjectCardProps) {
  return (
    <article className="project-card">
      <h3>{project.name}</h3>
      <p className="meta-text">{project.versions.length} versions saved</p>
      <button
        className="secondary-action"
        onClick={() => onSelect?.(project.id)}
        type="button"
      >
        View Project
      </button>
    </article>
  );
}
