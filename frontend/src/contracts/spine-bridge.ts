export interface ISpineBridge {
  readonly id: string;
  loadSkeleton(spineData: ArrayBuffer): Promise<string>;
  playAnimation(trackIndex: number, animationName: string): void;
}