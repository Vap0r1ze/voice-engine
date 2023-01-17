import { Callback, Json } from './data';
import { CipherMode } from './encryption';
import { DesktopSourceStatus, ScreenSource, StreamParameters } from './screen';
import { ConnectionStats, ConnectionStatsFilter } from './stats';
import { ConnectionTransportOptions } from './transport';

export type ConnectionOptions = {
    /** RTC Server IP */
    address: string;
    port: number;
    ssrc: number;
    experiments: string[];
    modes: CipherMode[];
    qosEnabled: boolean;
    streamParameters: readonly StreamParameters[];
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
    setTransportOptions(options: ConnectionTransportOptions): void;
    destroy(): void;

    /** Sends speaking flags of either local or remote user */
    setOnSpeakingCallback: Callback<[userId: string, flags: SpeakingFlags]>;
    setOnSpeakingWhileMutedCallback: Callback;

    /** Usually sent after `SoundshareAttachRequested` works */
    setOnSoundshare: Callback<[success: boolean]>;
    setOnSoundshareEnded: Callback<[]>;
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
    setOnDesktopSourceEnded: Callback<[]>;

    setSelfDeafen(deafened: boolean): void;
    /** Will be called with true on screenshare connections */
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
    // Discord doesn't use e and t
    // n is likely attempt and r timeout / interval
    // i.handlePingTimeout = function(e, t, n, r) { i.emit(a.Sh.PingTimeout, n, r > 0 ? r : 4e3) }
    setPingTimeoutCallback: Callback<
        [e: unknown, t: unknown, n: number, r: number]
    >; // FIXME
};
