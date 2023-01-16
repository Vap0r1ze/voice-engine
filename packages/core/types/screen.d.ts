export type ScreenSource = {
    type: 'video';
    active: boolean;
    rid: string;
    rtxSsrc: number;
    ssrc: number;
    maxBitrate?: number;
    maxFrameRate?: number;
    maxPixelCount?: number;
    maxResolution?: { type: 'fixed'; width: number; height: number };
    quality: number;
};

export type ScreenPreview = {
    /** `"screen-handle:${sourceId}"` */
    id: string;
    url: string;
};
export type WindowPreview = {
    /** `"window:${sourceId}"` */
    id: string;
    url: string;
};

export type StreamParameters = ScreenSource &
    (
        | {
              active: true;
              profile?: any;
          }
        | {
              active: false;
          }
    );

export type DesktopSourceStatus =
    | SoundshareAttachRequested
    | SoundshareStateTransition
    | SoundshareAudioDetected
    | VideoHookStart
    | VideoHookStop
    | VideoHookInit
    | VideoStateUpdate
    | ScreenshareFinish;

export const enum SoundshareState {
    Idle = 0,
}
type SoundshareAttachRequested = {
    type: 'soundshare_attach_requested';
    pid: number;
};
type SoundshareStateTransition = {
    type: 'soundshare_state_transition';
    newState: SoundshareState;
    prevState: SoundshareState;
};
type SoundshareAudioDetected = { type: 'soundshare_audio_detected' };
type VideoHookStart = { type: 'videohook_start' };
type VideoHookStop = { type: 'videohook_stop' };
export const enum VideoState {
    Playing = 'playing',
}
type VideoStateUpdate = {
    type: 'video_state';
    state: VideoState;
};

/** @telemetry */
type VideoHookInit = {
    type: 'videohook_initialize';
    backend: number;
    format: number;
    framebufferFormat: number;
    sampleCount: number;
    success: boolean;
    reinitialization: boolean;
};
/** @telemetry */
type ScreenshareFinish = {
    type: 'screenshare_finish';
    desktopCapturerType: 'window';
    activity: number;
    screens: number;
    windows: number;
    hybridDxgiFrames: number;
    hybridGdiFrames: number;
    hybridGraphicsCaptureFrames: number;
    hybridVideohookFrames: number;
};
