import { AudioCodec } from './audio';

export enum ConnectionStatsFilter {
    TRANSPORT = 1 << 0,
    OUTBOUND = 1 << 1,
    INBOUND = 1 << 2,
    ALL = TRANSPORT | OUTBOUND | INBOUND,
}
export type ConnectionStats = {
    outbound?: {
        id: string;
        audio: {
            codecName: AudioCodec;
            ssrc: number;
            rtt: number;
            codecPayloadType: number;
            bytesSent: number;
            audioLevel: number;
            delayMedian: number;
            delayStd: number;
            echoReturnLoss: number;
            echoReturnLossEnhancement: number;
            fractionLost: number;
            framesCaptured: number;
            framesRendered: number;
            jitter: number;
            noiseCancellerIsEnabled: true;
            noiseCancellerProcessTime: number;
            packetsLost: number;
            packetsSent: number;
            residualEchoLikelihood: number;
            residualEchoLikelihoodRecentMax: number;
            speaking: number;
            typingNoiseDetected: false;
        };
    };
    inbound?: any;
    transport?: {
        decryptionFailures: number;
        inboundBitrateEstimate: number;
        localAddress: string;
        maxPaddingBitrate: number;
        outboundBitrateEstimate: number;
        pacerDelay: number;
        receiverBitrateEstimate: number;
        receiverReports: any[];
        rtt: number;
        sendBandwidth: number;
    };
};
