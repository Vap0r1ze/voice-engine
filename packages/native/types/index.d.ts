import { AudioDevice, VADOptions } from './audio';
import { CameraDevice, MediaFilterSettings } from './camera';
import {
    ConnectionData,
    ConnectionOptions,
    VoiceConnection,
} from './connection';
import { Callback, FrameCallback, Json } from './data';
import { ScreenPreview, WindowPreview } from './screen';
import { VideoCodecCapabilities } from './video';
import { AudioSubsystem, EngineTransportOptions } from './engine';

declare module '@boisu/core' {
    export const getAudioSubsystem: Callback<
        [
            /** Something */
            audioSubsystem: AudioSubsystem,
            /** I don't think this does anything other than display in the voice info popout */
            audioLayer: string,
        ]
    >;
    export function setAudioSubsystem(audioSubsystem: AudioSubsystem): void;
    export function setTransportOptions(options: EngineTransportOptions): void;

    /* == Debugging == */
    /** Whether voice engine save the last 5m of input audio and echo cancellation data in memory, and save on voice disconnect */
    export function setAecDump(enabled: boolean): void;
    /** Whether voice engine should output debug logs */
    export function setDebugLogging(enabled: boolean): void;
    export function getDebugLogging(): boolean;

    /* == General Devices == */
    export const setDeviceChangeCallback: Callback<
        [
            inputDevices?: AudioDevice[],
            outputDevices?: AudioDevice[],
            videoDevices?: CameraDevice[],
        ]
    >;
    export const setVolumeChangeCallback: Callback<
        [inputVolume: number, outputVolume: number]
    >;

    /* == Camera == */
    export function applyMediaFilterSettings(
        settings: MediaFilterSettings,
    ): void;
    export function applyMediaFilterSettingsWithCallback(
        settings: MediaFilterSettings,
        cb: () => void,
    ): void;
    export const getVideoInputDevices: Callback<[devices: CameraDevice[]]>;
    /** @param deviceGuid `"disabled"` if no device */
    export function setVideoInputDevice(deviceGuid: string): void;
    /** @telemetry */
    export const setVideoInputInitializationCallback: Callback<
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
    export function getScreenPreviews(
        width: number,
        height: number,
        cb: (previews: ScreenPreview[]) => void,
    ): void;
    export function getWindowPreviews(
        width: number,
        height: number,
        cb: (previews: WindowPreview[]) => void,
    ): void;

    // These are called when a video sink is/isn't actively being displayed locally
    export function addDirectVideoOutputSink(sinkId: string): void;
    export function removeDirectVideoOutputSink(sinkId: string): void;
    export function clearVideoOutputSink(streamId: string): void;
    // TODO: what is something
    export function setVideoOutputSink(
        streamId: string,
        onFrame: FrameCallback,
        something: boolean,
    ): void;
    export function signalVideoOutputSinkReady(streamId: string): void;

    /* == Input Audio == */
    export function setInputDevice(deviceGuid: string): void;
    export function setInputVolume(volume: number): void;
    export const getInputDevices: Callback<[devices: AudioDevice[]]>;
    /**
     * Sets a threshold for the voice engine to report silence via `setNoInputCallback`
     * @param threshold Voice level in decibels, `threshold <= -100dB` means silence
     */
    export function setNoInputThreshold(threshold: number): void;
    export const setNoInputCallback: Callback<[crossesThreshold: boolean]>;
    /**
     * If enabled, the voice engine will call the `onVoice` callback with the voice activity level
     * @param enabled Whether to emit voice activity levels
     * @param monitor Whether to monitor the input device (via the output device)
     */
    export function setEmitVADLevel(
        shouldEmit: boolean,
        monitor: boolean,
        options: VADOptions,
    ): void;
    export const setOnVoiceCallback: Callback<
        [
            /** Voice level in decibels, `level <= -100dB` means silence */
            level: number,
            /** Whether the voice level crossed the threshold (`0` or `1`) */
            crossesThreshold: number,
        ]
    >;

    /* == Output Audio == */
    export function setOutputDevice(deviceGuid: string): void;
    export function setOutputVolume(volume: number): void;
    export const getOutputDevices: Callback<[devices: AudioDevice[]]>;

    /* == Connection == */
    export function createVoiceConnectionWithOptions(
        userId: string,
        options: ConnectionOptions,
        // TODO: figure out what `a` is
        onConnect: (a: string, data: ConnectionData) => void,
    ): VoiceConnection;
    /** Usually called when the connection ends */
    // TODO: figure out what returns when not `null`
    export const getCodecSurvey: Callback<[responseData: Json<any | null>]>;
    export const getCodecCapabilities: Callback<
        [capabilities: Json<VideoCodecCapabilities[]>]
    >;
}
