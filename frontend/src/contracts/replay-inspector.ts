export interface IReplayInspector {
  readonly id: string;
  captureFrame(): void;
  getRecordedFrames(): ArrayBuffer[];
}