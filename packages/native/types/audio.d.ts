type AudioCodecName = 'opus';
export type AudioCodec = {
    name: AudioCodecName;
    channels: number;
    freq: number;
    type: number;
    /** Codec params (eg: stereo) */
    params: Record<string, string>;
};
export interface AudioEncoder extends AudioCodec {
    rate: number;
    pacsize: number;
}

export type AudioDevice = {
    guid: string;
    name: string;
    index: number;
};

// Voice Activity Detection
export type VADOptions = {
    automaticGainControl?: boolean;
    builtInEchoCancellation?: boolean;
    echoCancellation?: boolean;
    noiseCancellation?: boolean;
    /** This is Krisp */
    noiseSuppression?: boolean;
};

export enum AudioInputMode {
    Activity = 1,
    PushToTalk = 2,
}
export type InputModeOptions =
    | {
          inputMode: AudioInputMode.Activity;
          inputModeOptions: {
              vadAutoThreshold: number;
              badThreshold: number;
              vadLeading: number;
              vadTrailing: number;
              vadUseKrisp: boolean;
          };
      }
    | {
          type: AudioInputMode.PushToTalk;
          inputModeOptions: { pttReleaseDelay: number };
      };
