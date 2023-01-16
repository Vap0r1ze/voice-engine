import { AudioCodec, AudioEncoder, InputModeOptions } from './audio';
import { CipherMode } from './encryption';
import { StreamParameters } from './screen';
import { VideoCodec } from './video';

export enum DegradationPreference {
    MAINTAIN_RESOLUTION = 0,
    MAINTAIN_FRAMERATE = 1,
    BALANCED = 2,
    DISABLED = 3,
}

export type ConnectionTransportOptions = Partial<InputModeOptions> &
    Partial<{
        hardwareH264: boolean;

        encodingVoiceBitRate: number;
        attenuateWhileSpeakingOthers: boolean;
        attenuateWhileSpeakingSelf: boolean;
        attenuation: boolean;
        attenuationFactor: number;
        prioritySpeakerDucking: number;

        /* Video */
        encodingVideoBitRate: number;
        encodingVideoFrameRate: number;
        encodingVideoMaxBitRate: number;
        encodingVideoMinBitRate: number;
        encodingVideoHeight: number;
        encodingVideoWidth: number;
        remoteSinkWantsMaxFramerate: number;
        remoteSinkWantsPixelCount: number;
        streamParameters: StreamParameters[];
        minimumJitterBufferLevel: number;
        encodingVideoDegradationPreference: DegradationPreference;

        videoEncoder: VideoCodec;
        videoDecoders: VideoCodec[];
        audioEncoder: AudioEncoder;
        audioDecoders: AudioCodec[];
        experimentalEncoders: boolean;

        qos: boolean;
        reconnectInterval: number; // ms

        selfMute: boolean;
        selfDeafen: boolean;

        /** https://datatracker.ietf.org/doc/html/rfc8627 */
        fec: boolean;

        /* I have no idea what these are */
        packetLossRate: number;
        postponeDecodeLevel: number;

        encryptionSettings: {
            // NOTE: Structure may not account for all cipher modes
            mode: CipherMode;
            secretKey: string;
        };
        userChannelIds: { userId: string; channelId: string; guildId?: string };
    }>;
