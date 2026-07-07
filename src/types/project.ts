export interface ProjectVersion {
  number: number;
  note: string;
  createdAt: string;
  relativePath: string;
}

export interface Project {
  id: string;
  name: string;
  sourcePath: string;
  createdAt: string;
  versions: ProjectVersion[];
}
