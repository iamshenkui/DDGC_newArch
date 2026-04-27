export const RendererType = {
  PixiJS: "pixi-js",
  HtmlCanvas: "html-canvas",
  WebGL: "webgl"
} as const;

export type RendererType = typeof RendererType[keyof typeof RendererType];