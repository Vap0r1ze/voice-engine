import { AudioDevice, VADOptions } from './audio';
import { CameraDevice, MediaFilterSettings } from './camera';
import {
    ConnectionData,
    ConnectionOptions,
    VoiceConnection,
} from './connection';
import { Callback, ImgData, Json } from './data';
import { ScreenPreview, WindowPreview } from './screen';
import { DegradationPreference } from './transport';
import { VideoCodecCapabilities } from './video';

type AudioSubsystem = 'standard' | 'legacy' | 'experimental';

export type EngineTransportOptions = VADOptions &
    Partial<{
        h264Enabled: boolean;
        av1Enabled: boolean;

        ducking: boolean;
        idleJitterBufferFlush: boolean;
    }>;

export type VoiceEngine = {
    DegradationPreference: typeof DegradationPreference;

    getAudioSubsystem: Callback<
        [
            /** Something */
            audioSubsystem: AudioSubsystem,
            /** I don't think this does anything other than display in the voice info popout */
            audioLayer: string,
        ]
    >;
    setAudioSubsystem(audioSubsystem: AudioSubsystem): void;
    setTransportOptions(options: EngineTransportOptions): void;

    /* == Debugging == */
    /** Whether voice engine save the last 5m of input audio and echo cancellation data in memory, and save on voice disconnect */
    setAecDump(enabled: boolean): void;
    /** Whether voice engine should output debug logs */
    setDebugLogging(enabled: boolean): void;
    getDebugLogging(): boolean;

    /* == General Devices == */
    setDeviceChangeCallback: Callback<
        [
            inputDevices?: AudioDevice[],
            outputDevices?: AudioDevice[],
            videoDevices?: CameraDevice[],
        ]
    >;
    setVolumeChangeCallback: Callback<
        [inputVolume: number, outputVolume: number]
    >;
    setActiveSinksChangeCallback: Callback<[sinkId: string, active: boolean]>;
    getNextVideoOutputFrame(
        sinkId: string,
    ): Promise<Pick<ImgData, 'data' | 'height' | 'width'>>;

    /* == Camera == */
    applyMediaFilterSettings(settings: MediaFilterSettings): void;
    applyMediaFilterSettingsWithCallback(
        settings: MediaFilterSettings,
        cb: () => void,
    ): void;
    getVideoInputDevices: Callback<[devices: CameraDevice[]]>;
    /** @param deviceGuid `"disabled"` if no device */
    setVideoInputDevice(deviceGuid: string): void;
    /** @telemetry */
    setVideoInputInitializationCallback: Callback<
        [
            info: {
                description: CameraDevice;
                entropy: number;
                timeToFirstFrame: number;
                initializationTimerExpired: boolean;
            },
        ]
    >;

    /* == Screen == */
    getScreenPreviews(
        width: number,
        height: number,
        cb: (previews: ScreenPreview[]) => void,
    ): void;
    getWindowPreviews(
        width: number,
        height: number,
        cb: (previews: WindowPreview[]) => void,
    ): void;

    // These are called when a video sink is/isn't actively being displayed locally
    // They both update whether the sink is active **syncronously** via `setActiveSinksChangeCallback`
    addDirectVideoOutputSink(sinkId: string): void;
    removeDirectVideoOutputSink(sinkId: string): void;

    /* == Input Audio == */
    setInputDevice(deviceGuid: string): void;
    setInputVolume(volume: number): void;
    getInputDevices: Callback<[devices: AudioDevice[]]>;
    /**
     * Sets a threshold for the voice engine to report silence via `setNoInputCallback`
     * @param threshold Voice level in decibels, `threshold <= -100dB` means silence
     */
    setNoInputThreshold(threshold: number): void;
    setNoInputCallback: Callback<[crossesThreshold: boolean]>;
    /**
     * If enabled, the voice engine will call the `onVoice` callback with the voice activity level
     * @param enabled Whether to emit voice activity levels
     * @param monitor Whether to monitor the input device (via the output device)
     */
    setEmitVADLevel(
        shouldEmit: boolean,
        monitor: boolean,
        options: VADOptions,
    ): void;
    setOnVoiceCallback: Callback<
        [
            /** Voice level in decibels, `level <= -100dB` means silence */
            level: number,
            /** Whether the voice level crossed the threshold (`0` or `1`) */
            crossesThreshold: number,
        ]
    >;

    /* == Output Audio == */
    setOutputDevice(deviceGuid: string): void;
    setOutputVolume(volume: number): void;
    getOutputDevices: Callback<[devices: AudioDevice[]]>;

    /* == Connection == */
    createVoiceConnectionWithOptions(
        userId: string,
        options: ConnectionOptions,
        // TODO: figure out what `a` is
        onConnect: (a: string, data: ConnectionData) => void,
    ): VoiceConnection;
    createOwnStreamConnectionWithOptions(
        userId: string,
        options: ConnectionOptions,
        onConnect: (a: string, data: ConnectionData) => void,
    );
    /** Usually called when the connection ends */
    // TODO: figure out what returns when not `null`
    getCodecSurvey: Callback<[responseData: Json<any | null>]>;
    getCodecCapabilities: Callback<
        [capabilities: Json<VideoCodecCapabilities[]>]
    >;
};
