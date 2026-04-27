export interface IPixiRenderer {
  readonly id: string;
  initialize(canvas: HTMLCanvasElement): void;
  render(): void;
  destroy(): void;
}