import type { ParentComponent } from "solid-js";

interface AppFrameProps {
  eyebrow: string;
  title: string;
  subtitle: string;
}

export const AppFrame: ParentComponent<AppFrameProps> = (props) => {
  return (
    <main class="app-frame">
      <header class="hero-header">
        <span class="eyebrow">{props.eyebrow}</span>
        <h1 class="hero-title">{props.title}</h1>
        <p class="hero-subtitle">{props.subtitle}</p>
      </header>
      {props.children}
    </main>
  );
};