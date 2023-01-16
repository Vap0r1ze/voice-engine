import { Callback, Json } from './data';
import { CipherMode } from './encryption';
import { ConnectionStats, ConnectionStatsFilter } from './stats';
import { DesktopSourceStatus, ScreenSource, StreamParameters } from './screen';

export type ConnectionOptions = {
    /** RTC Server IP */
    address: string;
    port: number;
    ssrc: number;
    experiments: string[];
    modes: CipherMode[];
    qosEnabled: boolean;
    streamParameters: StreamParameters[];
    streamUserId?: string;
};
export type ConnectionData = {
    address: string;
    port: number;
    protocol: 'udp';
};

export const enum SpeakingFlags {
    NONE = 0,
    VOICE = 1 << 0,
    SOUNDSHARE = 1 << 1,
    PRIORITY = 1 << 2,
}

type RemoteUser = {
    id: '712639419785412668';
    mute: boolean;
    rtxSsrc: number;
    ssrc: number;
    videoSsrc: number;
    volume: number;
};

export type VoiceConnection = {
    destroy(): void;

    /** Sends speaking flags of either local or remote user */
    setOnSpeakingCallback: Callback<[userId: string, flags: SpeakingFlags]>;
    setOnSpeakingWhileMutedCallback: Callback;

    /** Usually sent after `SoundshareAttachRequested` works */
    setOnSoundshare: Callback<[success: boolean]>;
    setOnSoundshareEnded: Callback<[any]>; // FIXME
    setOnSoundshareFailed: Callback<
        [code: number, reason: string, willRetry: boolean] // TODO: check `willRetry`
    >;

    /* == Video == */
    setOnVideoCallback: Callback<
        [
            userId: string,
            /** `0` if no stream */
            activeSsrc: number,
            /** `""` if no stream */
            activeSinkId: string,
            sources: ScreenSource[],
        ]
    >;
    setVideoBroadcast(broadcasting: boolean): void;

    /** == Screen == */
    /** Called when screenshare ends */
    clearDesktopSource(): void;
    setDesktopSourceWithOptions(options: {
        type: 'window' | 'screen-handle';
        sourceId: string;

        /** Screenshare for Windows and Linux? */
        useVideoHook: boolean;
        /** Some Windows thing? */
        useGraphicsCapture: boolean;
        /** macOS? */
        useQuartzCapturer: boolean;
        /** iPhone */
        allowScreenCaptureKit: boolean;

        // not sure what this is
        hdrCaptureMode: 'never' | 'always' | 'permittedDevicesOnly';
    }): void;
    setDesktopSourceStatusCallback: Callback<[status: DesktopSourceStatus]>;
    setOnDesktopSourceEnded: Callback; // FIXME

    setSelfDeafen(deafened: boolean): void;
    /** Will be called with true on stream connections */
    setSelfMute(muted: boolean): void;
    setSelfPan(left: number, right: number): void;
    setPTTActive(active: boolean, isPriority: boolean): void;

    /** @param volume volume multiplier */
    setLocalVolume(userId: string, volume: number): void;
    setLocalMute(userId: string, muted: boolean): void;
    setLocalPan(userId: string, left: number, right: number): void;

    mergeUsers(users: RemoteUser[]): void;
    destroyUser(userId: string): void;

    getFilteredStats(
        filter: ConnectionStatsFilter,
        cb: (stats: Json<ConnectionStats>) => void,
    ): void;
    getStats: Callback<[stats: Json<ConnectionStats>]>;
    getEncryptionModes: Callback<[modes: CipherMode[]]>;

    setPingInterval(ms: number): void;
    setPingCallback: Callback<
        [pingMs: number, remoteIp: string, remotePort: number, i: number]
    >;
};
