export interface WaveCalculationResponse {
  rss: number;
  rssDirectional: number;
}

interface WaveSources {
  height: number;
  period: number;
  direction: number;
}

export type WaveSourcesKey = keyof WaveSources;

export interface WaveInput {
  primarySwell: WaveSources;
  secondarySwell: WaveSources;
  windWaves: WaveSources;
}

export type WaveInputKey = keyof WaveInput;
